use ratatui::{
    prelude::*,
    style::Style,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::{App, Tab};
use crate::ui::rooms;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let tabs = Tabs::new(vec![
        Tab::Rooms.to_string(),
        Tab::Lights.to_string(),
        Tab::Sensors.to_string(),
        Tab::Routines.to_string(),
    ])
    .block(Block::default().borders(Borders::BOTTOM))
    .style(Style::default())
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .select(app.active_tab().index_of())
    .divider("|")
    .padding(" ", " ");

    frame.render_widget(tabs, main[0]);
    render_active_tab(app, frame, main[1]);
    render_status_bar(app, frame, main[2]);
}

fn render_active_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    match app.active_tab() {
        Tab::Rooms => rooms::render(app, frame, area),
        _ => {
            todo!()
        }
    }
}

fn render_status_bar(app: &mut App, frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .split(area);

    let block = Block::default().borders(Borders::TOP);
    frame.render_widget(block, area);
}
