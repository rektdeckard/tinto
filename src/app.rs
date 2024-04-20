use hues::{self, Bridge};
use ratatui::widgets::ListState;
use std::error;

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
    pub room_scene_list_state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn with_discovery() -> Self {
        let bridge = Bridge::new([10u8, 0, 0, 143], std::env::var("APP_KEY").unwrap())
            .listen(|_| {})
            .await;
        App {
            running: true,
            bridge,
            view: Default::default(),
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

    pub fn current_room(&self) -> Option<hues::Room> {
        if self.active_tab() == Tab::Rooms {
            if let Some(ri) = self.view.room_list_state.selected() {
                return self.bridge.rooms().into_iter().nth(ri);
            }
        }
        None
    }

    pub fn current_scene(&self) -> Option<hues::Scene> {
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
            Tab::Rooms => {
                self.view.room_active_view = match self.view.room_active_view {
                    RoomView::RoomList => RoomView::SceneList,
                    _ => RoomView::LightPanel,
                }
            }
            _ => todo!(),
        }
    }

    pub fn prev_view(&mut self) {
        match self.active_tab() {
            Tab::Rooms => {
                self.view.room_active_view = match self.view.room_active_view {
                    RoomView::LightPanel => RoomView::SceneList,
                    _ => RoomView::RoomList,
                }
            }
            _ => todo!(),
        }
    }

    pub fn next_list_item(&mut self) {
        match self.active_tab() {
            Tab::Rooms => match self.view.room_active_view {
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
                    todo!()
                }
            },
            _ => todo!(),
        }
    }

    pub fn prev_list_item(&mut self) {
        match self.active_tab() {
            Tab::Rooms => match self.view.room_active_view {
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
                RoomView::SceneList => self.view.room_scene_list_state.select(
                    self.view
                        .room_scene_list_state
                        .selected()
                        .map(|i| i.saturating_sub(1))
                        .or(Some(0)),
                ),
                RoomView::LightPanel => {
                    todo!()
                }
            },
            _ => todo!(),
        }
    }

    pub fn activate_current(&mut self) {
        match self.view.active_tab {
            Tab::Rooms => match self.view.room_active_view {
                RoomView::RoomList => {
                    if let Some(ri) = self.view.room_list_state.selected() {
                        if let Some(room) = self.bridge.rooms().get(ri) {
                            let _ = futures::executor::block_on(room.toggle());
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
                RoomView::LightPanel => todo!(),
            },
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    Rooms,
    Lights,
    Sensors,
    Routines,
}

impl ToString for Tab {
    fn to_string(&self) -> String {
        match self {
            Tab::Rooms => "ROOM".to_string(),
            Tab::Lights => "LGTS".to_string(),
            Tab::Sensors => "SENS".to_string(),
            Tab::Routines => "RTNS".to_string(),
        }
    }
}

impl Tab {
    pub fn index_of(&self) -> usize {
        match self {
            Tab::Rooms => 0,
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
    SceneList,
    LightPanel,
}
