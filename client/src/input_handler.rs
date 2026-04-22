use common::GameID;
use common::exports::game_object::GameObjE;
use common::exports::units::UnitType;
use std::collections::HashMap;
use std::ops::DerefMut;
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
            let n = io::stdin().read(&mut buf).await.unwrap_or(0);

            // Convert the MutexGuard into &mut SharedState.
            // This helps the borrow checker perform field-level borrowing more precisely
            // (borrow splitting works better on &mut T than on MutexGuard<T> in complex flows).
            let mut guard = shared_state.lock().await;
            let mut state = guard.deref_mut();

            // Keys that work for any UI state
            match &buf[..n] {
                [b'q'] => {
                    running = false;
                }
                [b'y'] => state.mod_right_tab = crate::game_renderer::ModRightTab::Castle,
                [b'x'] => state.mod_right_tab = crate::game_renderer::ModRightTab::Logs,
                [b'c'] => state.mod_right_tab = crate::game_renderer::ModRightTab::Debug,
                _ => {}
            }

            // Custom keybinds for each UI state
            match state.ui_state {
                UIState::Std => match &buf[..n] {
                    [b'z'] => {
                        Self::toggle_zoom(&mut state.map_zoom);
                    }
                    [b'l'] => {
                        Self::toggle_inspect(&mut state);
                    }
                    [0x1b, b'[', b'A'] => Self::move_zoom(0, -1, &mut state.map_zoom),
                    [0x1b, b'[', b'B'] => Self::move_zoom(0, 1, &mut state.map_zoom),
                    [0x1b, b'[', b'C'] => Self::move_zoom(1, 0, &mut state.map_zoom),
                    [0x1b, b'[', b'D'] => Self::move_zoom(-1, 0, &mut state.map_zoom),
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_zoom(0, -8, &mut state.map_zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        Self::move_zoom(0, 8, &mut state.map_zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        Self::move_zoom(8, 0, &mut state.map_zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        Self::move_zoom(-8, 0, &mut state.map_zoom)
                    }
                    _ => {}
                },
                UIState::Inspect(ref mut inspect) => match &buf[..n] {
                    [b'\x1b'] => state.ui_state = UIState::Std,
                    [b'n'] => Self::handle_new_castle(tx, inspect),
                    [b'z'] => {
                        Self::toggle_zoom(&mut state.map_zoom);
                    }
                    [b'i'] => {
                        Self::toggle_inspect(&mut state);
                    }
                    [b'\r'] => {
                        let looked_objs = SharedState::get_looked_objs(
                            inspect.coord,
                            &state.map_zoom,
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
                    [0x1b, b'[', b'A'] => {
                        Self::move_inspect(0, -1, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'B'] => {
                        Self::move_inspect(0, 1, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'C'] => {
                        Self::move_inspect(1, 0, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'D'] => {
                        Self::move_inspect(-1, 0, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_inspect(0, -8, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        Self::move_inspect(0, 8, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        Self::move_inspect(8, 0, inspect, &state.map_zoom, &state.game_objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        Self::move_inspect(-8, 0, inspect, &state.map_zoom, &state.game_objs)
                    }
                    _ => {}
                },
                UIState::Interact(ref mut interact) => match &buf[..n] {
                    [b'\x1b'] => state.ui_state = UIState::Std,
                    [b'a'] => {
                        state.ui_state =
                            UIState::UnitSelection(UIUnitSelection::from_interact(interact.clone()))
                    }
                    _ => {}
                },
                UIState::UnitSelection(ref mut selection) => match &buf[..n] {
                    [b'\x1b'] => state.ui_state = UIState::Std,
                    [b'a'] => {
                        Self::handle_unit_deploy(tx, selection);
                        state.ui_state = UIState::Std;
                    }
                    [b'\r'] => {
                        selection.selected_units.quantities[selection.active_input.0.as_index()] =
                            selection.active_input.1.parse().unwrap_or(0);
                    }
                    [b] if matches!(b, b'0'..=b'9') => {
                        selection.active_input.1.push(*b as char);
                    }
                    [0x1b, b'[', b'A'] => Self::move_unit_selection(0, -1, selection),
                    [0x1b, b'[', b'B'] => Self::move_unit_selection(0, 1, selection),
                    [0x1b, b'[', b'C'] => Self::move_unit_selection(1, 0, selection),
                    [0x1b, b'[', b'D'] => Self::move_unit_selection(-1, 0, selection),
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_unit_selection(0, -8, selection)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        Self::move_unit_selection(0, 8, selection)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        Self::move_unit_selection(8, 0, selection)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        Self::move_unit_selection(-8, 0, selection)
                    }
                    _ => {}
                },
            }
        }
    }

    fn handle_new_castle(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, inspect: &UIInspect) {
        let _ = tx.send(T2C::NewCastle(inspect.coord));
    }

    fn toggle_zoom(zoom: &mut Option<GameCoord>) {
        *zoom = match zoom {
            None => Some(GameCoord { x: 0, y: 0 }),
            Some(_) => None,
        };
    }

    fn toggle_inspect(state: &mut SharedState) {
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

    fn handle_unit_deploy(
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        selection: &UIUnitSelection,
    ) {
        match selection.interact.obj_id {
            Some(obj_id) => {
                let _ = tx.send(T2C::AttackCastle(obj_id, selection.selected_units.clone()));
            }
            None => {
                let _ = tx.send(T2C::SendUnits(
                    selection.interact.coord,
                    selection.selected_units.clone(),
                ));
            }
        };
    }

    fn move_zoom(dx: isize, dy: isize, zoom: &mut Option<GameCoord>) {
        if let Some(zoom) = zoom {
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

    fn move_inspect(
        mut dx: isize,
        mut dy: isize,
        inspect: &mut UIInspect,
        zoom: &Option<GameCoord>,
        objs: &HashMap<GameID, GameObjE>,
    ) {
        if let Some(ref mut selection) = inspect.selection {
            let looked_objs = SharedState::get_looked_objs(inspect.coord, zoom, &objs);
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
            if zoom.is_none() {
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

    fn move_unit_selection(dx: isize, dy: isize, selection: &mut UIUnitSelection) {
        match dy {
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
        }
    }
}
