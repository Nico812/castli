pub mod r#const;
mod map_data;
mod mod_central;
mod mod_inspect;
mod mod_interact;
mod mod_player_info;
mod module;
pub mod renderer;

#[derive(Copy, Clone)]
pub enum ModPlayerInfoTab {
    Castle,
    Debug,
    Logs,
}
