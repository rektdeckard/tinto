use ratatui::{
    prelude::*,
    style::Style,
    widgets::{Block, Borders, List, ListItem, Padding},
    Frame,
};

use super::lights::create_lights_barchart;
use super::utils::toggleable_item;
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let has_selection = app.view.room_list_state.selected().is_some()
        || app.view.room_zone_list_state.selected().is_some();

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if has_selection {
            [Constraint::Min(30), Constraint::Percentage(100)]
        } else {
            [Constraint::Fill(1), Constraint::Fill(0)]
        })
        .split(area);

    let inner_layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).split(layout[0]);

    let is_active_view = app.view.room_active_view == crate::app::RoomView::RoomList;
    let rooms_list = List::new(
        app.bridge
            .rooms()
            .into_iter()
            .map(|room| ListItem::new(toggleable_item(room.name(), room.group().unwrap().is_on())))
            .collect::<Vec<_>>(),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(
        Block::bordered()
            .border_style(Style::default().dim())
            .title(
                Line::default()
                    .spans(vec!["R".not_dim().underlined(), "OOM".not_dim()])
                    .add_modifier(if is_active_view {
                        Modifier::REVERSED
                    } else {
                        Modifier::default()
                    }),
            )
            .padding(Padding::uniform(1)),
    );
    frame.render_stateful_widget(rooms_list, inner_layout[0], &mut app.view.room_list_state);

    let is_active_view = app.view.room_active_view == crate::app::RoomView::ZoneList;
    let zones_list = List::new(
        app.bridge
            .zones()
            .into_iter()
            .map(|zone| ListItem::new(toggleable_item(zone.name(), zone.group().unwrap().is_on())))
            .collect::<Vec<_>>(),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(
        Block::bordered()
            .border_style(Style::default().dim())
            .title(
                Line::default()
                    .spans(vec!["Z".not_dim().underlined(), "ONE".not_dim()])
                    .add_modifier(if is_active_view {
                        Modifier::REVERSED
                    } else {
                        Modifier::default()
                    }),
            )
            .padding(Padding::uniform(1)),
    );
    frame.render_widget(zones_list, inner_layout[1]);

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
                .map(|r| r.name().to_uppercase().not_dim())
                .unwrap_or("ROOM NAME".not_dim())
                .add_modifier(if is_active_view {
                    Modifier::REVERSED
                } else {
                    Modifier::default()
                }),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().dim())
        .padding(Padding::proportional(1));

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
                        scene.status() != hues::service::SceneStatus::Inactive,
                    ))
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(
            Block::default()
                .title(
                    Line::default()
                        .spans(vec!["S".not_dim().underlined(), "CNS".not_dim()])
                        .add_modifier(if is_active_view {
                            Modifier::REVERSED
                        } else {
                            Modifier::default()
                        }),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().dim())
                .padding(Padding::uniform(1)),
        );

        if is_active_view {
            frame.render_stateful_widget(
                scenes_list,
                layout[0],
                &mut app.view.room_scene_list_state,
            );
        } else {
            frame.render_widget(scenes_list, layout[0]);
        }

        let barchart = create_lights_barchart(app, room.lights());
        frame.render_widget(barchart.block(block), layout[1])
    }
}
