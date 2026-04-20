use crate::{
    assets::TermCell, game_renderer::r#const::MOD_INTERACT_COLS, shared_state::SharedState,
};

struct ModInteract {}

impl ModRight {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_COLS: usize = MOD_INTERACT_COLS.saturating_sub(2);

    pub fn update(state: &mut SharedState) -> Vec<Vec<TermCell>> {}
}
