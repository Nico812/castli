use std::sync::Arc;
use tokio::io::{self, AsyncReadExt};
use tokio::sync::Mutex;

use crate::game_renderer::game_renderer::GameRenderer;
use crate::shared_state::{InteractTarget, SharedState};
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
            'l' => Self::toggle_look(state).await,
            'a' => Self::handle_attack(tx, state).await,
            'n' => Self::handle_new_castle(tx, state).await,
            '1' => state.mod_right_tab = crate::game_renderer::ModRightTab::Castle,
            '2' => state.mod_right_tab = crate::game_renderer::ModRightTab::Logs,
            '3' => state.mod_right_tab = crate::game_renderer::ModRightTab::Debug,
            '\r' => Self::apply_enter(state).await,
            '\u{1b}' => state.inspect_select = None,
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
        if let Some(inspect_select) = state.inspect_select {
            match dy {
                dy if dy > 0 => {
                    let looked_objs = state.get_looked_objs();
                    let current_pos = looked_objs.iter().position(|(id, _)| *id == inspect_select);

                    if let Some(pos) = current_pos {
                        let new_pos = (pos + 1).min(looked_objs.len() - 1);
                        if let Some(new_sel_obj) = looked_objs.get(new_pos) {
                            state.inspect_select = Some(new_sel_obj.0);
                        }
                    }
                }
                dy if dy < 0 => {
                    let looked_objs = state.get_looked_objs();
                    let current_pos = looked_objs.iter().position(|(id, _)| *id == inspect_select);

                    if let Some(pos) = current_pos {
                        let new_pos = pos.saturating_sub(1);
                        if let Some(new_sel_obj) = looked_objs.get(new_pos) {
                            state.inspect_select = Some(new_sel_obj.0);
                        }
                    }
                }
                _ => {}
            }
        } else if let Some(ref mut look) = state.map_look {
            if state.map_zoom.is_none() {
                dx *= GameRenderer::ZOOM_FACTOR as isize;
                dy *= GameRenderer::ZOOM_FACTOR as isize;
            };
            look.x = (look.x as isize + dx).max(0).min(MAP_COLS as isize - 1) as usize;
            look.y = (look.y as isize + dy).max(0).min(MAP_ROWS as isize - 1) as usize;
        } else if let Some(ref mut zoom) = state.map_zoom {
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

    async fn apply_enter(state: &mut SharedState) {
        let Some(look_coord) = state.map_look else {
            return;
        };

        match state.get_looked_objs() {
            ref objs if objs.len() > 1 => match state.inspect_select {
                None => state.inspect_select = Some(objs[0].0),
                Some(selected_id) => {
                    state.interact_target = Some(InteractTarget {
                        obj_id: Some(selected_id),
                        pos: look_coord,
                    })
                }
            },
            ref objs if objs.len() == 1 => {
                state.interact_target = Some(InteractTarget {
                    obj_id: Some(objs[0].0),
                    pos: look_coord,
                })
            }
            _ => {
                state.interact_target = Some(InteractTarget {
                    obj_id: None,
                    pos: look_coord,
                })
            }
        }
    }

    async fn toggle_zoom(state: &mut SharedState) {
        state.map_zoom = match state.map_zoom {
            None => Some(GameCoord { x: 0, y: 0 }),
            Some(_) => None,
        };
    }

    async fn toggle_look(state: &mut SharedState) {
        state.map_look = match state.map_look {
            None => {
                let map_zoom = state.map_zoom.unwrap_or(GameCoord { x: 0, y: 0 });

                Some(GameCoord {
                    x: map_zoom.x,
                    y: map_zoom.y,
                })
            }
            Some(_) => None,
        };
    }

    async fn handle_attack(tx: &tokio::sync::mpsc::UnboundedSender<T2C>, state: &mut SharedState) {
        if state.map_look == None {
            return;
        }
        let target_coord = state.map_look.unwrap();

        if let Some((id, _)) = state.get_looked_objs().get(0) {
            let _ = tx.send(T2C::AttackCastle(
                id.clone(),
                UnitGroupE {
                    quantities: [1, 0, 0],
                },
            ));
            state.add_log(format!("Requesting to attack object {}!", id));
        } else {
            let _ = tx.send(T2C::SendUnits(
                target_coord,
                UnitGroupE {
                    quantities: [1, 0, 0],
                },
            ));
            state.add_log(format!("Requesting to send troops to {}!", target_coord));
        }
    }

    async fn handle_new_castle(
        tx: &tokio::sync::mpsc::UnboundedSender<T2C>,
        shared_state: &mut SharedState,
    ) {
        if let Some(coords) = shared_state.map_look {
            let _ = tx.send(T2C::NewCastle(coords));
        }
    }
}
