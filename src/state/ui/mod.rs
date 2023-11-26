
pub type UIStateGuard<'a> = parking_lot::MutexGuard<'a, UIState>;

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

    pub fn current_page_mut(&mut self) -> &mut PageState {
        self.history.last_mut().expect("History must not be empty")
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
