pub mod canvas;
pub mod central_module;
pub mod r#const;
mod inspect_module;
mod module_utility;
mod right_module;

#[derive(Copy, Clone)]
pub enum RightModuleTab {
    Castle,
    Debug,
    Logs,
}
