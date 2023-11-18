
use super::model::*;

#[derive(Default)]
/// the application's data
pub struct AppData {
    pub user_data: UserData,
}

#[derive(Default, Debug)]
/// current user's data
pub struct UserData {
    pub user: Option<rspotify_model::PrivateUser>,
    pub playlists: Vec<Playlist>,
}

impl AppData {
//    pub fn get_tracks_by_id_mut(&mut self, id: &ContextId) -> Option<&mut Vec<Track>> {
//    }
}
