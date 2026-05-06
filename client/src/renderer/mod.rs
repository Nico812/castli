pub mod r#const;
mod map_data;
mod mod_central;
mod mod_inspect;
mod mod_interact;
mod mod_right;
mod module;
mod module_utility;
pub mod renderer;

#[derive(Copy, Clone)]
pub enum ModRightTab {
    Castle,
    Debug,
    Logs,
}
