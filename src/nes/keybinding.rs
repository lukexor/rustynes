use super::{action::Action, view::ViewType};
use pix_engine::event::PixEvent;

/// A mapping of a user event to a UI action
#[derive(Debug, Clone)]
pub struct Keybind {
    pub event: PixEvent,
    pub view_type: ViewType,
    pub modifiers: Vec<PixEvent>,
    pub action: Action,
}

impl Keybind {
    pub fn new(
        event: PixEvent,
        view_type: ViewType,
        modifiers: &[PixEvent],
        action: Action,
    ) -> Self {
        Self {
            event,
            view_type,
            modifiers: modifiers.to_vec(),
            action,
        }
    }
}