use anyhow::Result;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Borders,
    Frame,
};

use crate::state::{SharedState, UIStateGuard, PageState};

use super::{utils::construct_and_render_block, *};

pub fn render_library_page(
    is_active: bool,
    frame: &mut Frame,
    state: &SharedState,
    ui: &mut UIStateGuard,
    rect: Rect,
) -> Result<()> {
    // Get the data
    let data = state.data.read();

    // Constructor the page's layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(rect);
    let playlist_rect = utils::construct_and_render_block(
        "Playlists",
        state,
        Borders::TOP | Borders::LEFT | Borders::BOTTOM,
        frame,
        chunks[0],
    );

    // Constructor the page's widgets
    // Constructor the playlist window
    let (playlist_list, n_playlists) = utils::construct_list_widgets(
        to_items(&data.user_data.playlists)
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        is_active,
    );

    // Render the page's widgets
    let page_state = match ui.current_page_mut() {
        PageState::Library { state } => state,
    };

    utils::render_list_window(frame, playlist_list, playlist_rect, n_playlists, &mut page_state.playlist_list);

    Ok(())
}

// move to state for search after
fn to_items<'a, T: std::fmt::Display>(items: &'a [T]) -> Vec<&'a T> {
    items.iter().collect::<Vec<_>>()
}
