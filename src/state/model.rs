use std::borrow::Cow;

pub use rspotify::model as rspotify_model;

use rspotify::model::{PlaylistId, TrackId, UserId};
use serde::Serialize;

use crate::utils::format_duration;

#[derive(Clone, Debug, PartialEq, Eq)]
/// A context Id
pub enum ContextId {
    Playlist(PlaylistId<'static>),
}

#[derive(Serialize, Debug, Clone)]
/// A Spotify track
pub struct Track {
    pub id: TrackId<'static>,
    pub name: String,
    #[serde(serialize_with = "serialize_duration")]
    pub duration: chrono::Duration,
    pub explicit: bool,
}

pub fn serialize_duration<S>(dur: &chrono::Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&format_duration(dur))
}

#[derive(Serialize, Debug, Clone)]
/// A Spotify playlist
pub struct Playlist {
    pub id: PlaylistId<'static>,
    pub collaborative: bool,
    pub name: String,
    pub owner: (String, UserId<'static>),
}

impl From<rspotify_model::SimplifiedPlaylist> for Playlist {
    fn from(playlist: rspotify_model::SimplifiedPlaylist) -> Self {
        Self {
            id: playlist.id,
            name: playlist.name,
            collaborative: playlist.collaborative,
            owner: (
                playlist.owner.display_name.unwrap_or_default(),
                playlist.owner.id,
            ),
        }
    }
}

impl std::fmt::Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} • {}", self.name, self.owner.0)
    }
}

impl Track {
    pub fn display_name(&self) -> Cow<'_, str> {
        if self.explicit {
            Cow::Owned(format!("{} (E)", self.name))
        } else {
            Cow::Borrowed(self.name.as_str())
        }
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} • ▎", self.display_name())
    }
}
