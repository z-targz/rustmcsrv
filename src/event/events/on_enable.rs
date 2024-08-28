use crate::event::TraitEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct EventOnEnable {}

impl EventOnEnable {
    pub fn new() -> Self {
        Self {}
    }
}

impl TraitEvent for EventOnEnable {}