use anyhow::Result;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Borders, Wrap, Paragraph},
    Frame,
};

use super::utils::construct_and_render_block;

use crate::state::{SharedState, UIStateGuard};

pub fn render_playback_window(
    frame: &mut Frame,
    state: &SharedState,
    _ui: &mut UIStateGuard,
    rect: Rect,
) -> Result<()> {
    let rect = construct_and_render_block("Playback", state, Borders::ALL, frame, rect);

    frame.render_widget(
            Paragraph::new(
                "No playback found.\n \
                 Please make sure there is a running Spotify device and try to connect to one using the `SwitchDevice` command.\n \
                 You may also need to set up Spotify Connect to see available devices as in https://github.com/aome510/spotify-player#spotify-connect."
            )
            .wrap(Wrap { trim: true }),
            rect,
        );

    Ok(())
}

pub fn split_rect_for_playback_window(rect: Rect, state: &SharedState) -> (Rect, Rect) {
    let playback_width = state.configs.app_config.playback_window_width;

    let playback_width = (playback_width + 2) as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(playback_width), Constraint::Min(0)].as_ref())
        .split(rect);

    (chunks[0], chunks[1])
}
