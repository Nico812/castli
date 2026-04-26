use common::GameId;
use common::exports::game_object::GameObjE;
use common::exports::units::UnitType;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::client::{ShutdownChannel, ShutdownReason};
use crate::game_state::GameState;
use crate::renderer::renderer::Renderer;
use crate::tui::{T2C, Tui};
use crate::ui_state::{Inspect, Interact, UiMode, UiState, UnitSelection};
use common::GameCoord;
use common::r#const::{MAP_COLS, MAP_ROWS};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key(
        key: &KeyEvent,
        tx: &UnboundedSender<T2C>,
        game_state: &mut GameState,
        ui_state: &mut UiState,
        shutdown: ShutdownChannel,
    ) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Keys that work for any UI mode
        match key.code {
            KeyCode::Char('q') => {
                shutdown.shutdown(ShutdownReason::Key);
            }
            KeyCode::Char('y') => ui_state.tab = crate::renderer::ModRightTab::Castle,
            KeyCode::Char('x') => ui_state.tab = crate::renderer::ModRightTab::Logs,
            KeyCode::Char('c') => ui_state.tab = crate::renderer::ModRightTab::Debug,
            _ => {}
        }

        // Custom keybinds for each UI mode
        match ui_state.mode {
            UiMode::Std => match (key.code, key.modifiers) {
                (KeyCode::Char('z'), _) => {
                    Self::toggle_zoom(&mut ui_state.zoom, None);
                }
                (KeyCode::Char('l'), _) => {
                    Self::toggle_inspect(ui_state);
                }
                (KeyCode::Up, KeyModifiers::NONE) => Self::move_zoom(0, -1, &mut ui_state.zoom),
                (KeyCode::Down, KeyModifiers::NONE) => Self::move_zoom(0, 1, &mut ui_state.zoom),
                (KeyCode::Right, KeyModifiers::NONE) => Self::move_zoom(1, 0, &mut ui_state.zoom),
                (KeyCode::Left, KeyModifiers::NONE) => Self::move_zoom(-1, 0, &mut ui_state.zoom),
                (KeyCode::Up, KeyModifiers::CONTROL) => Self::move_zoom(0, -8, &mut ui_state.zoom),
                (KeyCode::Down, KeyModifiers::CONTROL) => Self::move_zoom(0, 8, &mut ui_state.zoom),
                (KeyCode::Right, KeyModifiers::CONTROL) => {
                    Self::move_zoom(8, 0, &mut ui_state.zoom)
                }
                (KeyCode::Left, KeyModifiers::CONTROL) => {
                    Self::move_zoom(-8, 0, &mut ui_state.zoom)
                }
                _ => {}
            },
            UiMode::Inspect(ref mut inspect) => match (key.code, key.modifiers) {
                (KeyCode::Esc, _) => ui_state.mode = UiMode::Std,
                (KeyCode::Char('n'), _) => Self::handle_new_castle(&tx, inspect),
                (KeyCode::Char('z'), _) => {
                    Self::toggle_zoom(&mut ui_state.zoom, Some(inspect));
                }
                (KeyCode::Char('l'), _) => {
                    Self::toggle_inspect(ui_state);
                }
                (KeyCode::Enter, _) => {
                    if game_state.player.castle_id.is_none() {
                        return;
                    }
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
                (KeyCode::Up, KeyModifiers::NONE) => {
                    Self::move_inspect(0, -1, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Down, KeyModifiers::NONE) => {
                    Self::move_inspect(0, 1, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Right, KeyModifiers::NONE) => {
                    Self::move_inspect(1, 0, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Left, KeyModifiers::NONE) => {
                    Self::move_inspect(-1, 0, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Up, KeyModifiers::CONTROL) => {
                    Self::move_inspect(0, -8, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Down, KeyModifiers::CONTROL) => {
                    Self::move_inspect(0, 8, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Right, KeyModifiers::CONTROL) => {
                    Self::move_inspect(8, 0, inspect, &ui_state.zoom, &game_state.objs)
                }
                (KeyCode::Left, KeyModifiers::CONTROL) => {
                    Self::move_inspect(-8, 0, inspect, &ui_state.zoom, &game_state.objs)
                }
                _ => {}
            },
            UiMode::Interact(ref mut interact) => match (key.code, key.modifiers) {
                (KeyCode::Esc, _) => ui_state.mode = UiMode::Std,
                (KeyCode::Char('a'), _) => {
                    ui_state.mode =
                        UiMode::UnitSelection(UnitSelection::from_interact(interact.clone()))
                }
                _ => {}
            },
            UiMode::UnitSelection(ref mut selection) => {
                let Some(ref castle) = game_state.castle else {
                    return;
                };

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) => ui_state.mode = UiMode::Std,
                    (KeyCode::Char('a'), _) => {
                        Self::handle_unit_deploy(&tx, selection);
                        ui_state.mode = UiMode::Std;
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(ref string) = selection.active_input.1 {
                            let unit_index = selection.active_input.0.as_index();

                            selection.selected_units.quantities[unit_index] = string
                                .parse()
                                .unwrap_or(0)
                                .min(castle.units.quantities[unit_index]);
                            selection.active_input.1 = None;
                        } else {
                            selection.active_input.1 = Some(String::new());
                        }
                    }
                    (KeyCode::Char(c), _) if c.is_ascii_digit() => {
                        if let Some(ref mut string) = selection.active_input.1 {
                            string.push(c);
                        }
                    }
                    (KeyCode::Backspace, _) => {
                        if let Some(ref mut string) = selection.active_input.1 {
                            let _ = string.pop();
                        }
                    }
                    (KeyCode::Up, KeyModifiers::NONE) => Self::move_unit_selection(-1, selection),
                    (KeyCode::Down, KeyModifiers::NONE) => Self::move_unit_selection(1, selection),
                    (KeyCode::Up, KeyModifiers::CONTROL) => {
                        Self::move_unit_selection(-8, selection)
                    }
                    (KeyCode::Down, KeyModifiers::CONTROL) => {
                        Self::move_unit_selection(8, selection)
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_new_castle(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, inspect: &Inspect) {
        let _ = tx.send(T2C::NewCastle(inspect.coord));
    }

    fn toggle_zoom(zoom: &mut Option<GameCoord>, inspect: Option<&mut Inspect>) {
        *zoom = match zoom {
            None => Some(GameCoord { x: 0, y: 0 }),
            Some(_) => {
                if let Some(inspect) = inspect {
                    inspect.coord.y -= inspect.coord.y % Renderer::ZOOM_FACTOR;
                    inspect.coord.x -= inspect.coord.x % Renderer::ZOOM_FACTOR;
                }
                None
            }
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
        match selection.obj_id {
            Some(obj_id) => {
                let _ = tx.send(T2C::AttackCastle(obj_id, selection.selected_units.clone()));
            }
            None => {
                let _ = tx.send(T2C::SendUnits(
                    selection.coord,
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
        objs: &HashMap<GameId, GameObjE>,
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
        if selection.active_input.1.is_some() {
            return;
        }

        match dy {
            dy if dy > 0 => {
                let new_unit_index =
                    (selection.active_input.0.as_index() + 1).min(UnitType::COUNT - 1);
                selection.active_input.0 = UnitType::form_index(new_unit_index);
            }
            dy if dy < 0 => {
                let new_unit_index = selection.active_input.0.as_index().saturating_sub(1);
                selection.active_input.0 = UnitType::form_index(new_unit_index);
            }
            _ => {}
        }
    }
}
