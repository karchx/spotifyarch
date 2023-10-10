extern crate crossbeam;
extern crate rspotify;

use rspotify::{
    model::context::CurrentlyPlayingContext, model::device::Device, model::enums::RepeatState,
    model::page::Page, model::playing::PlayHistory, model::track::SavedTrack, AuthCodePkceSpotify,
    OAuth,
};

pub enum SpotifyAPIEvent {
    Shuffle(bool, Option<String>),
    Pause(Option<String>),
    Device,
    Volume(u8, Option<String>),
    Repeat(RepeatState, Option<String>),
    SeekTrack(u32, Option<String>),
    NextTrack(Option<String>),
    PreviousTrack(Option<String>),
    CurrentPlayBack,
    CurrentUserRecentlyPlayed,
    DeleteCurrentUserSavedTracks(Vec<String>),
    AddCurrentUserSavedTracks(Vec<String>),
    CheckCurrentUserSavedTracks(Vec<String>),
    CurrentUserSavedTracks(Option<u32>), //offset
    StartPlayBack((Option<String>, Option<Vec<String>>)),
}

pub enum SpotifyAPIResult {
    CurrentPlayBack(Option<CurrentlyPlayingContext>),
    CurrentUserPlayingTrack(Option<PlayHistory>),
    CurrentUserRecentlyPlayed(Vec<PlayHistory>),
    CheckCurrentUserSavedTracks(Vec<(String, bool)>),
    CurrentUserSavedTracks(Page<SavedTrack>),
    Device(Vec<Device>),
    SuccessAddCurrentUserSavedTracks(Vec<String>),
    SuccessDeleteCurrentUserSavedTracks(Vec<String>),
}

pub struct SpotifyService {
    pub client: AuthCodePkceSpotify,
    pub oauth: OAuth,
    pub api_result_tx: Option<crossbeam::channel::Sender<SpotifyAPIResult>>,
    pub api_event_tx: crossbeam::channel::Sender<SpotifyAPIEvent>,
    pub api_event_rx: crossbeam::channel::Receiver<SpotifyAPIEvent>,
}

impl SpotifyService {
    //pub fn new(token_info: rspotify::oauth2::TokenInfo, oauth: rspotify::oauth2::SpotifyOAuth,) -> SpotifyService {
    //     let client_credential = rspotify::AuthCodeSpotify::default()
    //         .get_token();
    //
    //    // let client_credential = rspotify::oauth2::SpotifyClientCredentials::default()
    //    //     .token_info(token_info)
    //    //     .build();
    //     let spotify = rspotify::client::Spotify::default()
    //         .client_credentials_manager(client_credential)
    //         .build();

    //     let (tx, rx) = crossbeam::channel::unbounded();

    //     SpotifyService { client: spotify, oauth, api_result_tx: None, api_event_tx: tx, api_event_rx: rx }
    // }

    pub fn api_result_tx(mut self, tx: crossbeam::channel::Sender<SpotifyAPIResult>) -> Self {
        self.api_result_tx = Some(tx);
        self
    }

    // pub async fn run(mut self) -> Result<()> {
    //     let rx = self.api_event_rx.clone();

    //     tokio::spawn(async move {
    //         loop {
    //             match rx.recv().unwrap() {
    //                 SpotifyAPIEvent::StartPlayBack((device_id, uris)) => {
    //                     self.fetch_start_playback(device_id, uris).await
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     });

    //     Ok(())
    // }
  //  async fn fetch_start_playback(
  //      &self,
  //      device_id: Option<String>,
  //      uris: Option<Vec<String>>,
  //  ) -> Result<()> {
  //      self.client
  //          .start_playback(device_id, None, uris, None, None)
  //          .await
  //  }
}
