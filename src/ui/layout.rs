use crate::{
    app::{App, Tab},
    ui::{lights, rooms},
};
use ratatui::{
    prelude::*,
    style::Style,
    widgets::{Block, Borders, Tabs},
    Frame,
};

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

    let tabs = Tabs::new(vec![Tab::Areas, Tab::Lights, Tab::Sensors, Tab::Routines])
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().dim()),
        )
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
        Tab::Areas => rooms::render(app, frame, area),
        Tab::Lights => lights::render(app, frame, area),
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

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().dim());

    let id = app.bridge.data().unwrap().bridge_id;
    let lgts = app.bridge.n_lights();
    let zons = app.bridge.n_zones();
    let room = app.bridge.n_rooms();
    let info_str = format!("{} LGTS — {} ROOMS — {} ZONES", lgts, zons, room);

    frame.render_widget(info_str, layout[0]);
    frame.render_widget(id, layout[2]);
    // frame.render_widget(block, area);
}
