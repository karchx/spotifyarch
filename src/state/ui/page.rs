use tui::widgets::ListState;

use crate::utils;

#[derive(Clone, Debug)]
pub enum PageState {
    Library {
        state: LibraryPageUIState,
    },
}

pub enum PageType {
    Library,
}

#[derive(Clone, Debug)]
pub struct LibraryPageUIState {
    pub playlist_list: ListState,
}

impl PageState {
    /// the type of the page.
    pub fn page_type(&self) -> PageType {
        match self {
            PageState::Library { .. } => PageType::Library,
        }
    }
}

impl LibraryPageUIState {
    pub fn new() -> Self {
        Self {
            playlist_list: utils::new_list_state(),
        }
    }
}
