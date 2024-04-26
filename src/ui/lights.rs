use ratatui::{
    layout::{Direction, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Bar, BarChart, BarGroup, Block, Padding},
    Frame,
};
use std::cmp::Ordering;

use super::utils::toggleable_item;
use crate::app::{App, RoomView};

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_style(Style::default().dim())
        .title("ALL".not_dim().add_modifier(Modifier::REVERSED))
        .padding(Padding::proportional(1));
    let barchart = create_lights_barchart(app, app.bridge.lights());
    frame.render_widget(barchart.block(block), area);
}

pub fn create_lights_barchart<'a>(
    app: &App,
    mut lights: Vec<hues::service::Light<'a>>,
) -> BarChart<'a> {
    let active_index = app.view.room_lights_list_state.selected();

    let mut barchart = BarChart::default()
        .direction(Direction::Vertical)
        .bar_width(3)
        .bar_gap(1)
        .group_gap(7)
        .max(100);

    lights.sort_by(|a, b| {
        if a.supports_color() && !b.supports_color() {
            Ordering::Less
        } else {
            a.data().metadata.name.cmp(&b.data().metadata.name)
        }
    });
    for (i, light) in lights.into_iter().enumerate() {
        let data = light.data();
        let (col, bri, hue) = {
            let bri = if light.is_on() {
                data.dimming.brightness
            } else {
                0.0
            };
            let hue = if let Some(c) = &data.color {
                let (r, g, b) = c.xy.as_rgb(Some(data.dimming.brightness / 100.0));
                Color::Rgb(r, g, b)
            } else {
                let (r, g, b) = &data.color_temperature.as_rgb();
                Color::Rgb(*r, *g, *b)
            };
            let col = 100.0;
            (col, bri, hue)
        };

        let group = BarGroup::default()
            .label(
                Span::from(toggleable_item(&data.metadata.name, light.is_on()))
                    .into_centered_line()
                    .add_modifier(
                        active_index
                            .map(|n| {
                                if app.view.room_active_view == RoomView::LightPanel && n == i {
                                    Modifier::REVERSED
                                } else {
                                    Modifier::default()
                                }
                            })
                            .unwrap_or_default(),
                    ),
            )
            .bars(&[
                Bar::default()
                    .value(col as u64)
                    .value_style(Style::default().add_modifier(Modifier::REVERSED))
                    .text_value("HUE".into())
                    .style(Style::default().fg(hue)),
                Bar::default()
                    .value(bri as u64)
                    .value_style(Style::default().add_modifier(Modifier::REVERSED))
                    .text_value("BRI".into()),
            ]);
        barchart = barchart.data(group);
    }

    barchart
}
