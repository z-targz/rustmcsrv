use crate::event::TraitEvent;

#[derive(Debug, Clone)]
pub struct EventOnDisable {}

impl EventOnDisable {
    pub fn new() -> Self {
        Self {}
    }
}

impl TraitEvent for EventOnDisable {}