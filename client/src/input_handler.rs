use common::GameID;
use common::exports::game_object::GameObjE;
use common::exports::units::UnitType;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt};
use tokio::sync::Mutex;

use crate::client::ShutdownChannel;
use crate::game_state::GameState;
use crate::renderer::renderer::Renderer;
use crate::tui::{T2C, Tui};
use crate::ui_state::{Inspect, Interact, UiMode, UiState, UnitSelection};
use common::GameCoord;
use common::r#const::{MAP_COLS, MAP_ROWS};

pub struct InputHandler;

impl InputHandler {
    pub async fn run(
        tx: tokio::sync::mpsc::UnboundedSender<T2C>,
        game_state: Arc<Mutex<GameState>>,
        ui_state: Arc<Mutex<UiState>>,
        shutdown: ShutdownChannel,
    ) {
        while !shutdown.is_shutdown() {
            let mut buf = [0u8; 8];
            let n = io::stdin().read(&mut buf).await.unwrap_or(0);

            // Convert the MutexGuard into &mut SharedState.
            // This helps the borrow checker perform field-level borrowing more precisely
            // (borrow splitting works better on &mut T than on MutexGuard<T> in complex flows).
            let mut ui_guard = ui_state.lock().await;
            let mut ui_state = ui_guard.deref_mut();
            let mut game_guard = game_state.lock().await;
            let game_state = game_guard.deref_mut();

            // Keys that work for any UI mode
            match &buf[..n] {
                [b'q'] => {
                    shutdown.shutdown();
                }
                [b'y'] => ui_state.tab = crate::renderer::ModRightTab::Castle,
                [b'x'] => ui_state.tab = crate::renderer::ModRightTab::Logs,
                [b'c'] => ui_state.tab = crate::renderer::ModRightTab::Debug,
                _ => {}
            }

            // Custom keybinds for each UI mode
            match ui_state.mode {
                UiMode::Std => match &buf[..n] {
                    [b'z'] => {
                        Self::toggle_zoom(&mut ui_state.zoom);
                    }
                    [b'l'] => {
                        Self::toggle_inspect(&mut ui_state);
                    }
                    [0x1b, b'[', b'A'] => Self::move_zoom(0, -1, &mut ui_state.zoom),
                    [0x1b, b'[', b'B'] => Self::move_zoom(0, 1, &mut ui_state.zoom),
                    [0x1b, b'[', b'C'] => Self::move_zoom(1, 0, &mut ui_state.zoom),
                    [0x1b, b'[', b'D'] => Self::move_zoom(-1, 0, &mut ui_state.zoom),
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_zoom(0, -8, &mut ui_state.zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        Self::move_zoom(0, 8, &mut ui_state.zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        Self::move_zoom(8, 0, &mut ui_state.zoom)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        Self::move_zoom(-8, 0, &mut ui_state.zoom)
                    }
                    _ => {}
                },
                UiMode::Inspect(ref mut inspect) => match &buf[..n] {
                    [b'\x1b'] => ui_state.mode = UiMode::Std,
                    [b'n'] => Self::handle_new_castle(&tx, inspect),
                    [b'z'] => {
                        Self::toggle_zoom(&mut ui_state.zoom);
                    }
                    [b'i'] => {
                        Self::toggle_inspect(&mut ui_state);
                    }
                    [b'\r'] => {
                        let looked_objs =
                            Tui::get_looked_objs(inspect.coord, &ui_state.zoom, &game_state.objs);

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

                            ui_state.mode = UiMode::Interact(Interact { obj_id, coord });
                        }
                    }
                    [0x1b, b'[', b'A'] => {
                        Self::move_inspect(0, -1, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'B'] => {
                        Self::move_inspect(0, 1, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'C'] => {
                        Self::move_inspect(1, 0, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'D'] => {
                        Self::move_inspect(-1, 0, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_inspect(0, -8, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        Self::move_inspect(0, 8, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        Self::move_inspect(8, 0, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        Self::move_inspect(-8, 0, inspect, &ui_state.zoom, &game_state.objs)
                    }
                    _ => {}
                },
                UiMode::Interact(ref mut interact) => match &buf[..n] {
                    [b'\x1b'] => ui_state.mode = UiMode::Std,
                    [b'a'] => {
                        ui_state.mode =
                            UiMode::UnitSelection(UnitSelection::from_interact(interact.clone()))
                    }
                    _ => {}
                },
                UiMode::UnitSelection(ref mut selection) => match &buf[..n] {
                    [b'\x1b'] => ui_state.mode = UiMode::Std,
                    [b'a'] => {
                        Self::handle_unit_deploy(&tx, selection);
                        ui_state.mode = UiMode::Std;
                    }
                    [b'\r'] => {
                        selection.selected_units.quantities[selection.active_input.0.as_index()] =
                            selection.active_input.1.parse().unwrap_or(0);
                    }
                    [b] if matches!(b, b'0'..=b'9') => {
                        selection.active_input.1.push(*b as char);
                    }
                    [0x1b, b'[', b'A'] => Self::move_unit_selection(-1, selection),
                    [0x1b, b'[', b'B'] => Self::move_unit_selection(1, selection),
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        Self::move_unit_selection(-8, selection)
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => Self::move_unit_selection(8, selection),
                    _ => {}
                },
            }
        }
    }

    fn handle_new_castle(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, inspect: &Inspect) {
        let _ = tx.send(T2C::NewCastle(inspect.coord));
    }

    fn toggle_zoom(zoom: &mut Option<GameCoord>) {
        *zoom = match zoom {
            None => Some(GameCoord { x: 0, y: 0 }),
            Some(_) => None,
        };
    }

    fn toggle_inspect(ui_state: &mut UiState) {
        if let UiMode::Std = ui_state.mode {
            let coord = match ui_state.zoom {
                None => GameCoord { x: 0, y: 0 },
                Some(zoom_coord) => zoom_coord,
            };
            ui_state.mode = UiMode::Inspect(Inspect {
                coord,
                selection: None,
            });
        } else if let UiMode::Inspect(_) = ui_state.mode {
            ui_state.mode = UiMode::Std;
        }
    }

    fn handle_unit_deploy(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, selection: &UnitSelection) {
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
                .min(MAP_COLS as isize - Renderer::FOV_COLS as isize) as usize;
            zoom.y = (zoom.y as isize + 2 * dy)
                .max(0)
                .min((MAP_ROWS) as isize - (Renderer::FOV_ROWS * 2) as isize)
                as usize;
        }
    }

    fn move_inspect(
        mut dx: isize,
        mut dy: isize,
        inspect: &mut Inspect,
        zoom: &Option<GameCoord>,
        objs: &HashMap<GameID, GameObjE>,
    ) {
        if let Some(ref mut selection) = inspect.selection {
            let looked_objs = Tui::get_looked_objs(inspect.coord, zoom, &objs);
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
                dx *= Renderer::ZOOM_FACTOR as isize;
                dy *= Renderer::ZOOM_FACTOR as isize;
            };
            inspect.coord.x = (inspect.coord.x as isize + dx)
                .max(0)
                .min(MAP_COLS as isize - 1) as usize;
            inspect.coord.y = (inspect.coord.y as isize + dy)
                .max(0)
                .min(MAP_ROWS as isize - 1) as usize;
        }
    }

    fn move_unit_selection(dy: isize, selection: &mut UnitSelection) {
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
