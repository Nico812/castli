use common::courtyard::FacilityType;
use common::units::{UnitType, UnitsTarget};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use tokio::sync::mpsc::UnboundedSender;

use crate::camera::{Camera, CameraLocation};
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::ModPlayerInfoTab;
use crate::shutdown::{ShutdownChannel, ShutdownReason};
use crate::tui::{T2C, Tui};
use crate::ui_state::{FacilitySelection, Inspect, InteractTarget, UiMode, UiState, UnitSelection};
use common::GameCoord;
use common::r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_input(
        event: &Event,
        tx: &UnboundedSender<T2C>,
        game_state: &mut GameState,
        ui_state: &mut UiState,
        shutdown: ShutdownChannel,
    ) {
        if let Event::Resize(col, row) = event {
            ui_state.term_size_change = Some(TermCoord::new(*row as isize, *col as isize));
            return;
        }

        let key = if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            key
        } else {
            return;
        };

        match key.code {
            KeyCode::Char('q') => {
                shutdown.shutdown(ShutdownReason::Key);
            }
            KeyCode::Char('y') => ui_state.tab = ModPlayerInfoTab::Castle,
            KeyCode::Char('x') => ui_state.tab = ModPlayerInfoTab::Logs,
            KeyCode::Char('c') => ui_state.tab = ModPlayerInfoTab::Debug,
            KeyCode::Esc => ui_state.restore_prev_mode(),
            _ => {}
        }

        match ui_state.mode {
            UiMode::Std => match (key.code, key.modifiers) {
                (KeyCode::Char('M'), _) => {
                    ui_state.change_camera_location(CameraLocation::WorldMap, tx);
                }
                (KeyCode::Char('m'), _) => {
                    ui_state.change_camera_location(CameraLocation::Map, tx);
                }
                (KeyCode::Char('k'), _) => {
                    ui_state.change_camera_location(CameraLocation::Courtyard, tx);
                }
                (KeyCode::Char('l'), _) => {
                    ui_state.toggle_inspect();
                }
                (KeyCode::Up, KeyModifiers::NONE) => ui_state.move_camera(0, -1),
                (KeyCode::Down, KeyModifiers::NONE) => ui_state.move_camera(0, 1),
                (KeyCode::Right, KeyModifiers::NONE) => ui_state.move_camera(1, 0),
                (KeyCode::Left, KeyModifiers::NONE) => ui_state.move_camera(-1, 0),
                (KeyCode::Up, KeyModifiers::CONTROL) => ui_state.move_camera(0, -8),
                (KeyCode::Down, KeyModifiers::CONTROL) => ui_state.move_camera(0, 8),
                (KeyCode::Right, KeyModifiers::CONTROL) => ui_state.move_camera(8, 0),
                (KeyCode::Left, KeyModifiers::CONTROL) => ui_state.move_camera(-8, 0),
                _ => {}
            },
            UiMode::Inspect(ref mut inspect) => match (key.code, key.modifiers) {
                (KeyCode::Char('M'), _) => {
                    ui_state.change_camera_location(CameraLocation::WorldMap, tx);
                }
                (KeyCode::Char('m'), _) => {
                    ui_state.change_camera_location(CameraLocation::Map, tx);
                }
                (KeyCode::Char('k'), _) => {
                    ui_state.change_camera_location(CameraLocation::Courtyard, tx);
                }
                (KeyCode::Char('l'), _) => {
                    ui_state.toggle_inspect();
                }
                (KeyCode::Enter, _) => match ui_state.camera.location {
                    CameraLocation::Map | CameraLocation::WorldMap => {
                        let in_world_map = ui_state.camera.location == CameraLocation::WorldMap;

                        if game_state.player.castle_id.is_none() {
                            if !in_world_map {
                                Self::handle_new_castle(tx, inspect)
                            }
                            return;
                        }
                        let looked_objs =
                            Tui::get_looked_objs(game_state, inspect.coord, &ui_state.camera);

                        if looked_objs.len() > 1 && inspect.selection.is_none() {
                            inspect.selection = Some(looked_objs[0].0);
                        } else {
                            let interact_target = match looked_objs.len() {
                                0 => InteractTarget::MapPos(inspect.coord),
                                1 => InteractTarget::GameObj(looked_objs[0].0),
                                _ => InteractTarget::GameObj(inspect.selection.unwrap()),
                            };

                            ui_state.change_mode(UiMode::Interact(interact_target));
                        }
                    }
                    CameraLocation::Courtyard => {
                        if game_state.player.castle_id.is_none() {
                            return;
                        }
                        let looked_facility =
                            Tui::get_looked_facility(game_state, inspect.coord, &ui_state.camera);
                        if let Some(looked_facility) = looked_facility {
                            ui_state.change_mode(UiMode::Interact(InteractTarget::Facility(
                                looked_facility.0,
                            )));
                        } else {
                            let inspect_coord = inspect.coord;
                            ui_state.change_mode(UiMode::Interact(InteractTarget::CourtyardPos(
                                inspect_coord,
                            )));
                        }
                    }
                },
                (KeyCode::Up, KeyModifiers::NONE) => {
                    inspect.displace(0, -2, &ui_state.camera, game_state)
                }
                (KeyCode::Down, KeyModifiers::NONE) => {
                    inspect.displace(0, 2, &ui_state.camera, game_state)
                }
                (KeyCode::Right, KeyModifiers::NONE) => {
                    inspect.displace(2, 0, &ui_state.camera, game_state)
                }
                (KeyCode::Left, KeyModifiers::NONE) => {
                    inspect.displace(-2, 0, &ui_state.camera, game_state)
                }
                (KeyCode::Up, KeyModifiers::CONTROL) => {
                    inspect.displace(0, -8, &ui_state.camera, game_state)
                }
                (KeyCode::Down, KeyModifiers::CONTROL) => {
                    inspect.displace(0, 8, &ui_state.camera, game_state)
                }
                (KeyCode::Right, KeyModifiers::CONTROL) => {
                    inspect.displace(8, 0, &ui_state.camera, game_state)
                }
                (KeyCode::Left, KeyModifiers::CONTROL) => {
                    inspect.displace(-8, 0, &ui_state.camera, game_state)
                }
                _ => {}
            },
            UiMode::Interact(ref interact_target) => match (key.code, key.modifiers) {
                (KeyCode::Char('a'), _) => match interact_target {
                    InteractTarget::MapPos(_) | InteractTarget::GameObj(_) => {
                        ui_state.change_mode(UiMode::UnitSelection(UnitSelection::from_interact(
                            interact_target.clone(),
                        )))
                    }
                    _ => {}
                },
                (KeyCode::Char('n'), _) => {
                    if let InteractTarget::CourtyardPos(pos) = interact_target {
                        ui_state
                            .change_mode(UiMode::FacilitySelection(FacilitySelection::new(*pos)));
                    }
                }
                _ => {}
            },
            UiMode::UnitSelection(ref mut selection) => {
                let Some(ref castle) = game_state.castle else {
                    return;
                };

                match (key.code, key.modifiers) {
                    (KeyCode::Char('a'), _) => {
                        if selection.selected_units.is_empty() {
                            game_state.add_log("You didn't select any units. Make sure to confirm the selection with enter.");
                        } else {
                            Self::handle_unit_deploy(tx, selection);
                        };
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
                    (KeyCode::Up, KeyModifiers::NONE) => selection.move_selection(-1),
                    (KeyCode::Down, KeyModifiers::NONE) => selection.move_selection(1),
                    (KeyCode::Up, KeyModifiers::CONTROL) => selection.move_selection(-8),
                    (KeyCode::Down, KeyModifiers::CONTROL) => selection.move_selection(8),
                    _ => {}
                }
            }
            UiMode::FacilitySelection(ref mut selection) => {
                if game_state.castle.is_none() {
                    return;
                };
                match (key.code, key.modifiers) {
                    (KeyCode::Enter, _) => {
                        let _ = tx.send(T2C::NewFacility((selection.pos, selection.active)));

                        // TODO: Should the client wait for a erver response and if positive return to Inspect mode?
                        // More user friendly but also more useless communication. But the server responds anyways. Maybe i have to review the communication.
                        // ui_state.mode = UiMode::Inspect(Inspect {
                        //     coord: selection.pos,
                        //     selection: None,
                        // });
                    }
                    (KeyCode::Up, KeyModifiers::NONE) => selection.move_selection(-1),
                    (KeyCode::Down, KeyModifiers::NONE) => selection.move_selection(1),
                    _ => {}
                }
            }
        }
    }

    fn handle_new_castle(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, inspect: &Inspect) {
        let _ = tx.send(T2C::NewCastle(inspect.coord));
    }

    fn handle_unit_deploy(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, selection: &UnitSelection) {
        let target = match selection.interact_target {
            InteractTarget::GameObj(obj_id) => UnitsTarget::Castle(obj_id),
            InteractTarget::MapPos(pos) => UnitsTarget::Coord(pos),
            _ => {
                return;
            }
        };

        let _ = tx.send(T2C::SendUnits(target, selection.selected_units.clone()));
    }
}
