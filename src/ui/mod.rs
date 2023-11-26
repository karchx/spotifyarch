use anyhow::{Result, Context};
use tui::{widgets::Block, style, Frame, layout::Rect};

use crate::state::*;

type Terminal = tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;

mod playback;
mod utils;
mod page;

/// run the application ui
pub fn run(state: SharedState) -> Result<()> {
    let mut terminal = init_ui().context("failed to initialize the application's UI")?;

    let ui_refresh_duration = std::time::Duration::from_millis(state.configs.app_config.app_refresh_duration_in_ms);

    loop {
        {
            let mut ui = state.ui.lock();
            if let Err(err) = terminal.draw(|frame| {
                let block = Block::default().style(style::Style::default());
                frame.render_widget(block, frame.size());

                if let Err(err) = render_application(frame, &state, &mut ui, frame.size()) {
                    tracing::error!("Failed to render the application: {err:#}");
                }
            }) {
                 tracing::error!("Failed to draw the application: {err:#}");
            }
        }
    }
}

fn init_ui() -> Result<Terminal> {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

/// renders the application
fn render_application(frame: &mut Frame, state: &SharedState, ui: &mut UIStateGuard, rect: Rect) -> Result<()> {
    // playback window is the window that is always displayed on the screen.
    let (playback_rect, rect) = playback::split_rect_for_playback_window(rect, state);
    //playback::render_playback_window(frame, state, ui, rect);

    render_main_layout(true, frame, state, ui, rect)?;
    Ok(())
}

fn render_main_layout(is_active: bool, frame: &mut Frame, state: &SharedState, ui: &mut UIStateGuard, rect: Rect) -> Result<()> {
    page::render_library_page(is_active, frame, state, ui, rect)
}

