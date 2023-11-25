mod page;

pub use page::*;

#[derive(Debug)]
pub struct UIState {
    pub history: Vec<PageState>
}

impl UIState {
    pub fn current_page(&self) -> &PageState {
        self.history.last().expect("History must not be empty")
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            history: vec![PageState::Library {
                state: LibraryPageUIState::new(),
            }],
        }
    }
}
