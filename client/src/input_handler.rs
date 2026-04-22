use common::exports::units::UnitType;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt};
use tokio::sync::Mutex;

use crate::game_renderer::game_renderer::GameRenderer;
use crate::shared_state::{SharedState, UIInspect, UIInteract, UIState, UIUnitSelection};
use crate::tui::T2C;
use common::r#const::{MAP_COLS, MAP_ROWS};
use common::{GameCoord, exports::units::UnitGroupE};

pub struct InputHandler;

impl InputHandler {
    pub async fn run(
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        shared_state: Arc<Mutex<SharedState>>,
    ) {
        let mut running = true;
        while running {
            let mut buf = [0u8; 8];
            let n = io::stdin().read(&mut buf).await.unwrap();

            {
                let mut state = shared_state.lock().await;

                if n == 1 {
                    Self::handle_single_char(&buf[0], tx, &mut state, &mut running).await;
                } else if n >= 3 {
                    Self::handle_special_chars(&buf[..n], &mut state).await;
                }
            }
        }
    }

    async fn handle_single_char(
        byte: &u8,
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        state: &mut SharedState,
        running: &mut bool,
    ) {
        match *byte as char {
            'q' => *running = false,
            'z' => Self::toggle_zoom(state).await,
            'l' => Self::toggle_inspect(state).await,
            'a' => {
                if let UIState::Interact(ref interact) = state.ui_state {
                    state.ui_state =
                        UIState::UnitSelection(UIUnitSelection::from_interact(interact.clone()));
                }
            }
            'n' => Self::handle_new_castle(tx, state).await,
            'y' => state.mod_right_tab = crate::game_renderer::ModRightTab::Castle,
            'x' => state.mod_right_tab = crate::game_renderer::ModRightTab::Logs,
            'c' => state.mod_right_tab = crate::game_renderer::ModRightTab::Debug,
            '\r' => Self::apply_enter(state).await,
            '\u{1b}' => Self::apply_esc(state).await,
            '0'..='9' => {
                if let UIState::UnitSelection(ref mut unit_selection) = state.ui_state {
                    unit_selection.active_input.1.push(*byte as char);
                }
            }
            _ => {}
        }
    }

    async fn handle_special_chars(buf: &[u8], state: &mut SharedState) {
        match buf {
            [0x1b, b'[', b'A'] => Self::apply_move(state, 0, -1).await,
            [0x1b, b'[', b'B'] => Self::apply_move(state, 0, 1).await,
            [0x1b, b'[', b'C'] => Self::apply_move(state, 1, 0).await,
            [0x1b, b'[', b'D'] => Self::apply_move(state, -1, 0).await,
            [0x1b, b'[', b'1', b';', b'5', b'A'] => Self::apply_move(state, 0, -8).await,
            [0x1b, b'[', b'1', b';', b'5', b'B'] => Self::apply_move(state, 0, 8).await,
            [0x1b, b'[', b'1', b';', b'5', b'C'] => Self::apply_move(state, 8, 0).await,
            [0x1b, b'[', b'1', b';', b'5', b'D'] => Self::apply_move(state, -8, 0).await,
            _ => {}
        }
    }

    async fn apply_move(state: &mut SharedState, mut dx: isize, mut dy: isize) {
        match state.ui_state {
            UIState::Std => {
                if let Some(ref mut zoom) = state.map_zoom {
                    zoom.x = (zoom.x as isize + 2 * dx)
                        .max(0)
                        .min(MAP_COLS as isize - GameRenderer::FOV_COLS as isize)
                        as usize;
                    zoom.y = (zoom.y as isize + 2 * dy)
                        .max(0)
                        .min((MAP_ROWS) as isize - (GameRenderer::FOV_ROWS * 2) as isize)
                        as usize;
                }
            }
            UIState::Inspect(ref mut inspect) => {
                if let Some(ref mut selection) = inspect.selection {
                    let looked_objs = SharedState::get_looked_objs(
                        inspect.coord,
                        state.map_zoom.is_some(),
                        &state.game_objs,
                    );
                    let new_selection = {
                        let current_pos = looked_objs.iter().position(|(id, _)| *id == *selection);
                        match dy {
                            dy if dy > 0 => current_pos.and_then(|pos| {
                                let new_pos = (pos + 1).min(looked_objs.len() - 1);
                                looked_objs.get(new_pos).map(|(id, _)| *id)
                            }),
                            dy if dy < 0 => current_pos.and_then(|pos| {
                                let new_pos = pos.saturating_sub(1);
                                looked_objs.get(new_pos).map(|(id, _)| *id)
                            }),
                            _ => None,
                        }
                    };

                    if let Some(new_id) = new_selection {
                        *selection = new_id;
                    }
                } else {
                    if state.map_zoom.is_none() {
                        dx *= GameRenderer::ZOOM_FACTOR as isize;
                        dy *= GameRenderer::ZOOM_FACTOR as isize;
                    };
                    inspect.coord.x = (inspect.coord.x as isize + dx)
                        .max(0)
                        .min(MAP_COLS as isize - 1) as usize;
                    inspect.coord.y = (inspect.coord.y as isize + dy)
                        .max(0)
                        .min(MAP_ROWS as isize - 1) as usize;
                }
            }
            UIState::UnitSelection(ref mut selection) => match dy {
                dy if dy > 0 => {
                    let new_unit_index =
                        (selection.active_input.0.as_index() + 1).min(UnitType::COUNT - 1);
                    selection.active_input.0 = UnitType::form_index(new_unit_index);
                    selection.active_input.1 =
                        selection.selected_units.quantities[new_unit_index].to_string();
                }
                dy if dy < 0 => {
                    let new_unit_index = selection.active_input.0.as_index().saturating_sub(1);
                    selection.active_input.0 = UnitType::form_index(new_unit_index);
                    selection.active_input.1 =
                        selection.selected_units.quantities[new_unit_index].to_string();
                }
                _ => {}
            },
            _ => {}
        }
    }

    async fn apply_enter(state: &mut SharedState) {
        if let UIState::Inspect(ref mut inspect) = state.ui_state {
            let looked_objs = SharedState::get_looked_objs(
                inspect.coord,
                state.map_zoom.is_some(),
                &state.game_objs,
            );

            if looked_objs.len() > 1 && inspect.selection.is_none() {
                inspect.selection = Some(looked_objs[0].0);
            } else {
                let (obj_id, coord) = match looked_objs.len() {
                    0 => (None, inspect.coord),
                    1 => {
                        let obj = looked_objs[0];
                        (Some(obj.0), obj.1.get_pos())
                    }
                    _ => {
                        let selected_id = inspect.selection.unwrap();
                        let pos = looked_objs
                            .iter()
                            .find(|(id, _)| *id == selected_id)
                            .map(|(_, obj)| obj.get_pos())
                            .unwrap();
                        (Some(selected_id), pos)
                    }
                };

                state.ui_state = UIState::Interact(UIInteract { obj_id, coord });
            }
        }
    }

    async fn apply_esc(state: &mut SharedState) {
        state.ui_state = UIState::Std;
    }

    async fn toggle_zoom(state: &mut SharedState) {
        state.map_zoom = match state.map_zoom {
            None => Some(GameCoord { x: 0, y: 0 }),
            Some(_) => None,
        };
    }

    async fn toggle_inspect(state: &mut SharedState) {
        if let UIState::Std = state.ui_state {
            let coord = match state.map_zoom {
                None => GameCoord { x: 0, y: 0 },
                Some(zoom_coord) => zoom_coord,
            };
            state.ui_state = UIState::Inspect(UIInspect {
                coord,
                selection: None,
            });
        } else if let UIState::Inspect(_) = state.ui_state {
            state.ui_state = UIState::Std;
        }
    }

    async fn handle_unit_deploy(
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        state: &mut SharedState,
    ) {
        if let UIState::Interact(ref interact) = state.ui_state {
            match interact.obj_id {
                Some(obj_id) => {
                    let _ = tx.send(T2C::AttackCastle(
                        obj_id,
                        UnitGroupE {
                            quantities: [1, 0, 0],
                        },
                    ));
                    state.add_log(format!("Requesting to attack object {}!", obj_id));
                }
                None => {
                    let _ = tx.send(T2C::SendUnits(
                        interact.coord,
                        UnitGroupE {
                            quantities: [1, 0, 0],
                        },
                    ));
                    state.add_log(format!("Requesting to send troops to {}!", interact.coord));
                }
            };
        }
    }

    async fn handle_new_castle(
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        state: &mut SharedState,
    ) {
        if let UIState::Inspect(ref inspect) = state.ui_state {
            let _ = tx.send(T2C::NewCastle(inspect.coord));
        }
    }
}
