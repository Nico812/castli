pub mod r#const;
pub mod game_renderer;
mod map_data;
mod mod_central;
mod mod_inspect;
mod mod_interact;
mod mod_right;
mod module_utility;

#[derive(Copy, Clone)]
pub enum ModRightTab {
    Castle,
    Debug,
    Logs,
}
