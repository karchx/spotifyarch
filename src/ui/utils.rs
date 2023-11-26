use tui::{
    style::{Color, Style},
    widgets::{BorderType, Borders, List, ListItem, ListState},
};

use super::*;

pub fn render_list_window(frame: &mut Frame, widget: List, rect: Rect, len: usize, state: &mut ListState) {
    frame.render_stateful_widget(widget, rect, state);
}

pub fn construct_and_render_block(
    title: &str,
    _state: &SharedState,
    borders: Borders,
    frame: &mut Frame,
    rect: Rect,
) -> Rect {
    let (borders, border_type) = (borders, BorderType::Rounded);

    let block = Block::default()
        .title(title)
        .borders(borders)
        .border_style(Style::default())
        .border_type(border_type);

    let inner_rect = block.inner(rect);

    frame.render_widget(block, rect);
    inner_rect
}

/// construct a generic list widget
pub fn construct_list_widgets<'a>(items: Vec<String>, _is_active: bool) -> (List<'a>, usize) {
    let n_items = items.len();

    (
        List::new(
            items
                .into_iter()
                .map(|s| ListItem::new(s).style(Style::default()))
                .collect::<Vec<_>>(),
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White)),
        n_items,
    )
}
