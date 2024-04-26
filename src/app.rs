use crate::config::Config;
use anyhow::Result;
use clap::{Parser, Subcommand};
use hues::prelude::*;
use ratatui::{prelude::*, text::Line, widgets::ListState};
use std::{cmp::Ordering, error, net::IpAddr};

/// CLI Args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Initialize tinto, creating application credentials on the Hue Bridge
    ///
    /// If --addr <ADDR> argument is passed, or HUE_BRIDGE_ADDR set,
    /// attempts to communicate with a bridge at this address. Otherwise, uses
    /// MDNS protocol to attempt discovery of a local bridge device.
    /// Upon discov
    #[arg(long)]
    pub init: bool,

    /// Set a custom config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Sets a custom Bridge IP
    #[arg(short, long, env = "HUE_BRIDGE_ADDR")]
    pub addr: Option<IpAddr>,

    /// Sets a custom App Key
    #[arg(short, long, env = "HUE_APP_KEY")]
    pub key: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize tinto, creating application credentials on the Hue Bridge
    ///
    /// If --addr <ADDR> argument is passed, or HUE_BRIDGE_ADDR set,
    /// attempts to communicate with a bridge at this address. Otherwise, uses
    /// MDNS protocol to attempt discovery of a local bridge device.
    /// Upon discovery, writes the address and credentials to the config file.
    Init,
    /// Reset tinto, removing application credentials on the Hue Bridge
    ///
    /// If --key <KEY> argument is passed, or HUE_APP_KEY set, attempts to
    /// delete credentials for this application on the bridge device.
    Reset,
}

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub bridge: Bridge,
    pub running: bool,
    pub view: ViewState,
}

#[derive(Debug, Default)]
pub struct ViewState {
    pub active_tab: Tab,
    pub room_active_view: RoomView,
    pub room_list_state: ListState,
    pub room_zone_list_state: ListState,
    pub room_scene_list_state: ListState,
    pub room_lights_list_state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn try_init(args: Args) -> Result<Self> {
        if args.init {
            let mut bridge = if let Some(addr) = args.addr {
                Ok(Bridge::new(addr, ""))
            } else {
                let bridge = Bridge::discover()
                    .await
                    .map(|b| b.build())
                    .map_err(|_| anyhow::anyhow!("did not discover bridge"));
                bridge
            }?;
            dbg!(&bridge);

            let addr = bridge.addr().clone();
            let key = bridge
                .create_app("tinto", std::process::id().to_string())
                .await
                .unwrap();

            Config::write_config_toml(&addr, key)?;

            Ok(App {
                running: true,
                bridge,
                view: Default::default(),
            })
        } else {
            let config = Config::try_init(&args).expect("failed to find config");
            let bridge = Bridge::new(config.bridge_ip, config.app_key)
                .listen(|_| {})
                .await;

            Ok(App {
                running: true,
                bridge,
                view: Default::default(),
            })
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn active_tab(&self) -> Tab {
        self.view.active_tab
    }

    pub fn current_room(&self) -> Option<hues::service::Room> {
        if self.active_tab() == Tab::Areas {
            if let Some(ri) = self.view.room_list_state.selected() {
                return self.bridge.rooms().into_iter().nth(ri);
            }
        }
        None
    }

    pub fn current_scene(&self) -> Option<hues::service::Scene> {
        if let Some(room) = self.current_room() {
            if let Some(si) = self.view.room_list_state.selected() {
                todo!()
                // return room.scenes().into_iter().nth(si);
            }
        }
        None
    }

    pub fn next_view(&mut self) {
        match self.active_tab() {
            Tab::Areas => {
                self.view.room_active_view = match self.view.room_active_view {
                    RoomView::RoomList => RoomView::ZoneList,
                    RoomView::ZoneList => RoomView::SceneList,
                    RoomView::SceneList => RoomView::LightPanel,
                    RoomView::LightPanel => {
                        self.view.room_lights_list_state.select(
                            self.view
                                .room_lights_list_state
                                .selected()
                                .map(|i| {
                                    (i + 1).min(
                                        self.current_room()
                                            .map(|r| r.lights().len().saturating_sub(1))
                                            .unwrap_or_default(),
                                    )
                                })
                                .or(Some(0)),
                        );
                        RoomView::LightPanel
                    }
                }
            }
            _ => todo!(),
        }
    }

    pub fn prev_view(&mut self) {
        match self.active_tab() {
            Tab::Areas => {
                self.view.room_active_view = match self.view.room_active_view {
                    RoomView::LightPanel => {
                        let li = self.view.room_lights_list_state.selected();
                        match li {
                            None | Some(0) => RoomView::SceneList,
                            Some(_n) => {
                                self.view.room_lights_list_state.select(
                                    self.view
                                        .room_lights_list_state
                                        .selected()
                                        .map(|i| i.saturating_sub(1))
                                        .or(Some(0)),
                                );
                                RoomView::LightPanel
                            }
                        }
                    }
                    RoomView::SceneList => RoomView::ZoneList,
                    _ => RoomView::RoomList,
                }
            }
            _ => todo!(),
        }
    }

    pub fn next_list_item(&mut self) {
        match self.active_tab() {
            Tab::Areas => match self.view.room_active_view {
                RoomView::RoomList => {
                    self.view.room_list_state.select(
                        self.view
                            .room_list_state
                            .selected()
                            .map(|i| (i + 1).min(self.bridge.n_rooms().saturating_sub(1)))
                            .or(Some(0)),
                    );
                    self.view.room_scene_list_state.select(None);
                }
                RoomView::ZoneList => {
                    self.view.room_zone_list_state.select(
                        self.view
                            .room_zone_list_state
                            .selected()
                            .map(|i| (i + 1).min(self.bridge.n_zones().saturating_sub(1)))
                            .or(Some(0)),
                    );
                    // TODO: we also need to make Rooms deselected if a zone is
                    self.view.room_scene_list_state.select(None);
                }

                RoomView::SceneList => {
                    if let Some(room) = self.current_room() {
                        self.view.room_scene_list_state.select(
                            self.view
                                .room_scene_list_state
                                .selected()
                                .map(|i| (i + 1).min(room.scenes().len().saturating_sub(1)))
                                .or(Some(0)),
                        );
                    }
                }
                RoomView::LightPanel => {
                    if let Some(room) = self.current_room() {
                        let mut lights = room.lights();
                        lights.sort_by(|a, b| {
                            if a.supports_color() && !b.supports_color() {
                                Ordering::Less
                            } else {
                                a.data().metadata.name.cmp(&b.data().metadata.name)
                            }
                        });

                        if let Some(light) = lights.get(
                            self.view
                                .room_lights_list_state
                                .selected()
                                .unwrap_or_default(),
                        ) {
                            let _ = futures::executor::block_on(light.send(&[
                                LightCommand::DimDelta {
                                    action: Some(DeltaAction::Down),
                                    brightness_delta: Some(10.0),
                                },
                            ]));
                        }
                    }
                }
            },
            _ => todo!(),
        }
    }

    pub fn prev_list_item(&mut self) {
        match self.active_tab() {
            Tab::Areas => match self.view.room_active_view {
                RoomView::RoomList => {
                    self.view.room_list_state.select(
                        self.view
                            .room_list_state
                            .selected()
                            .map(|i| i.saturating_sub(1))
                            .or(Some(0)),
                    );
                    self.view.room_scene_list_state.select(None);
                }
                RoomView::ZoneList => {
                    self.view.room_zone_list_state.select(
                        self.view
                            .room_zone_list_state
                            .selected()
                            .map(|i| i.saturating_sub(1))
                            .or(Some(0)),
                    );
                    self.view.room_scene_list_state.select(None);
                }

                RoomView::SceneList => self.view.room_scene_list_state.select(
                    self.view
                        .room_scene_list_state
                        .selected()
                        .map(|i| i.saturating_sub(1))
                        .or(Some(0)),
                ),
                RoomView::LightPanel => {
                    if let Some(room) = self.current_room() {
                        let mut lights = room.lights();
                        lights.sort_by(|a, b| {
                            if a.supports_color() && !b.supports_color() {
                                Ordering::Less
                            } else {
                                a.data().metadata.name.cmp(&b.data().metadata.name)
                            }
                        });

                        if let Some(light) = lights.get(
                            self.view
                                .room_lights_list_state
                                .selected()
                                .unwrap_or_default(),
                        ) {
                            let _ = futures::executor::block_on(light.send(&[
                                LightCommand::DimDelta {
                                    action: Some(DeltaAction::Up),
                                    brightness_delta: Some(10.0),
                                },
                            ]));
                        }
                    }
                }
            },
            _ => todo!(),
        }
    }

    pub fn activate_current(&mut self) {
        match self.view.active_tab {
            Tab::Areas => match self.view.room_active_view {
                RoomView::RoomList => {
                    if let Some(ri) = self.view.room_list_state.selected() {
                        if let Some(room) = self.bridge.rooms().get(ri) {
                            let _ = futures::executor::block_on(room.toggle());
                        }
                    }
                }
                RoomView::ZoneList => {
                    if let Some(zi) = self.view.room_zone_list_state.selected() {
                        if let Some(zone) = self.bridge.zones().get(zi) {
                            let _ = futures::executor::block_on(zone.toggle());
                        }
                    }
                }

                RoomView::SceneList => {
                    if let Some(room) = self.current_room() {
                        if let Some(si) = self.view.room_scene_list_state.selected() {
                            if let Some(scene) = room.scenes().get(si) {
                                let _ = futures::executor::block_on(scene.recall());
                            }
                        }
                    }
                }
                RoomView::LightPanel => {
                    if let Some(room) = self.current_room() {
                        if let Some(li) = self.view.room_lights_list_state.selected() {
                            let mut lights = room.lights();
                            lights.sort_by(|a, b| {
                                if a.supports_color() && !b.supports_color() {
                                    Ordering::Less
                                } else {
                                    a.data().metadata.name.cmp(&b.data().metadata.name)
                                }
                            });
                            if let Some(light) = lights.get(li) {
                                let _ = futures::executor::block_on(light.toggle());
                            }
                        } else {
                            let _ = futures::executor::block_on(room.group().unwrap().send(&[
                                GroupCommand::Signaling {
                                    signal: hues::service::SignalType::Alternating,
                                    duration: 8000,
                                    colors: Some(SignalColor::Two(
                                        hues::service::CIEColor::from_hex("#d2991d").unwrap(),
                                        hues::service::CIEColor::from_hex("#1a5c85").unwrap(),
                                    )),
                                },
                            ]));
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Areas,
    Lights,
    Sensors,
    Routines,
}

impl ToString for Tab {
    fn to_string(&self) -> String {
        match self {
            Tab::Areas => "AREA".to_string(),
            Tab::Lights => "LGTS".to_string(),
            Tab::Sensors => "SENS".to_string(),
            Tab::Routines => "RTNS".to_string(),
        }
    }
}

impl<'a> Into<Line<'a>> for Tab {
    fn into(self) -> Line<'a> {
        match self {
            Tab::Areas => Line::default().spans(vec!["A".underlined(), "REA".into()]),
            Tab::Lights => Line::default().spans(vec!["L".into(), "G".underlined(), "TS".into()]),
            Tab::Sensors => Line::default().spans(vec!["S".into(), "E".underlined(), "NS".into()]),
            Tab::Routines => Line::default().spans(vec!["R".into(), "T".underlined(), "NS".into()]),
        }
    }
}

impl Tab {
    pub fn index_of(&self) -> usize {
        match self {
            Tab::Areas => 0,
            Tab::Lights => 1,
            Tab::Sensors => 2,
            Tab::Routines => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum RoomView {
    #[default]
    RoomList,
    ZoneList,
    SceneList,
    LightPanel,
}
