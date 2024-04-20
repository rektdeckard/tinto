use ratatui::{
    prelude::*,
    style::Style,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::utils::toggleable_item;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let is_active_view = app.view.room_active_view == crate::app::RoomView::RoomList;
    let has_selection = app.view.room_list_state.selected().is_some();

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if has_selection {
            [Constraint::Min(30), Constraint::Percentage(100)]
        } else {
            [Constraint::Fill(1), Constraint::Fill(0)]
        })
        .split(area);
    let block = Block::default()
        .title("ROOMS".add_modifier(if is_active_view {
            Modifier::REVERSED
        } else {
            Modifier::default()
        }))
        .padding(Padding::uniform(1))
        .borders(Borders::ALL);

    let rooms_list = List::new(
        app.bridge
            .rooms()
            .into_iter()
            .map(|room| ListItem::new(toggleable_item(room.name(), room.group().unwrap().is_on())))
            .collect::<Vec<_>>(),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(block);

    frame.render_stateful_widget(rooms_list, layout[0], &mut app.view.room_list_state);
    if has_selection {
        render_rooms_view(app, frame, layout[1]);
    }
}

fn render_rooms_view(app: &mut App, frame: &mut Frame, area: Rect) {
    let is_active_view = app.view.room_active_view == crate::app::RoomView::LightPanel;

    let rooms = app.bridge.rooms();
    let current = app
        .view
        .room_list_state
        .selected()
        .and_then(|i| rooms.get(i));
    let block = Block::default()
        .title(
            current
                .map(|r| r.name().to_uppercase())
                .unwrap_or("ROOM NAME".to_string())
                .add_modifier(if is_active_view {
                    Modifier::REVERSED
                } else {
                    Modifier::default()
                }),
        )
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    if let Some(room) = current {
        let is_active_view = app.view.room_active_view == crate::app::RoomView::SceneList;

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Percentage(100)])
            .split(area);

        let scenes_list = List::new(
            room.scenes()
                .into_iter()
                .map(|scene| {
                    ListItem::new(toggleable_item(
                        scene.name(),
                        scene.status() != hues::SceneStatus::Inactive,
                    ))
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(
            Block::default()
                .title("SCNS".add_modifier(if is_active_view {
                    Modifier::REVERSED
                } else {
                    Modifier::default()
                }))
                .borders(Borders::ALL)
                .padding(Padding::uniform(1)),
        );

        frame.render_stateful_widget(scenes_list, layout[0], &mut app.view.room_scene_list_state);

        frame.render_widget(
            Paragraph::new(format!("{:?}", room.data()))
                .wrap(Wrap { trim: true })
                .block(block),
            layout[1],
        );
    } else {
        frame.render_widget(block, area);
    }
}
