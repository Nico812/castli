mod central_module;
pub mod r#const;
mod module_utility;
pub mod renderer;
mod right_module;

#[derive(Copy, Clone)]
pub enum RightModuleTab {
    Castle,
    Debug,
    Logs,
}
