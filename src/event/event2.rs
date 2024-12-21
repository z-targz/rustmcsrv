use std::{any::{Any, TypeId}, collections::HashMap, ptr::NonNull, sync::RwLock};

use crate::event::{
    CommandEvent, EventOnDisable, EventOnEnable, EventPlayerLogin
};

use super::TraitEvent;

/// An enum for built-in vanilla events
pub enum Event {
    OnEnable { e: EventOnEnable },
    OnDisable { e: EventOnDisable },
    PlayerLogin { e: EventPlayerLogin },
    Command { e: CommandEvent },
}

/// Used for plugins which emit their own events to be handled by other plugins.
pub struct PluginEvent {

}

pub struct EventManager {
    event_map: RwLock<HashMap<TypeId, HandlerList>>,
}

pub struct HandlerList {
    highest: Vec<EventHandler>,
    high: Vec<EventHandler>,
    normal: Vec<EventHandler>,
    low: Vec<EventHandler>,
    lowest: Vec<EventHandler>,
    monitor: Vec<EventHandler>,
}

impl HandlerList {
    pub fn new() -> Self {
        Self {
            highest: Vec::new(),
            high: Vec::new(),
            normal: Vec::new(),
            low: Vec::new(),
            lowest: Vec::new(),
            monitor: Vec::new(),
        }
    }

    pub fn get_handlers(&self, priority: &EventPriority) -> &Vec<EventHandler> {
        match priority {
            EventPriority::Monitor => &self.monitor,
            EventPriority::Lowest => &self.lowest,
            EventPriority::Low => &self.low,
            EventPriority::Normal => &self.normal,
            EventPriority::High => &self.high,
            EventPriority::Highest => &self.highest,
        }
    }

    pub fn get_handlers_mut(&mut self, priority: &EventPriority) -> &mut Vec<EventHandler> {
        match priority {
            EventPriority::Monitor => &mut self.monitor,
            EventPriority::Lowest => &mut self.lowest,
            EventPriority::Low => &mut self.low,
            EventPriority::Normal => &mut self.normal,
            EventPriority::High => &mut self.high,
            EventPriority::Highest => &mut self.highest,
        }
    }

    pub fn register(&mut self, handler: EventHandler) {
        let vec = match &handler.priority {
            EventPriority::Monitor => &mut self.monitor,
            EventPriority::Lowest => &mut self.lowest,
            EventPriority::Low => &mut self.low,
            EventPriority::Normal => &mut self.normal,
            EventPriority::High => &mut self.high,
            EventPriority::Highest => &mut self.highest,
        };
        if !vec.contains(&handler) {
            vec.push(handler)
        }
    }

    fn unregister_all(&mut self) {
        self.monitor.clear();
        self.lowest.clear();
        self.low.clear();
        self.normal.clear();
        self.high.clear();
        self.highest.clear();
    }
}

pub struct PluginEventManager {

}

impl EventManager {
    pub fn new() -> Self {
        Self {
            event_map: RwLock::new(HashMap::new()),
        }
    }
}

pub fn listen<E: TraitEvent>(manager: &EventManager, e: &mut Event) -> EventResult {
    fn handle<E: TraitEvent>(
        handler: &EventHandler<E>,
        result: &mut EventResult,
        e: &mut E
    ) {
        match (handler.func)(e) {
            EventResult::Deny => *result = EventResult::Deny,
            EventResult::Default => (),
            EventResult::Allow => *result = EventResult::Allow,
        }
    }

    let mut result = EventResult::Default;
    let mut evt: NonNull<()>;
    let id = match e {
        Event::OnEnable { e } => { 
            evt = NonNull::from(e).cast();
            TypeId::of::<EventOnEnable>()
        },
        Event::OnDisable { e } => {
            evt = NonNull::from(e).cast();
            TypeId::of::<EventOnDisable>()
        },
        Event::PlayerLogin { e } => {
            evt = NonNull::from(e).cast();
            TypeId::of::<EventPlayerLogin>()
        },
        Event::Command { e } => {
            evt = NonNull::from(e).cast();
            TypeId::of::<CommandEvent>()
        },
    };

    if let Some(l) = manager.event_map.read().unwrap().get(&id) {
        for handler in l.lowest.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }
        for handler in l.low.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }
        for handler in l.normal.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }
        for handler in l.high.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }
        for handler in l.highest.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }                
        for handler in l.monitor.iter() {
            handle(handler, &mut result, unsafe { evt.cast().as_mut() });
        }
    }
    
    result
}

pub enum EventResult {
    Deny,
    Default,
    Allow,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum EventPriority {
    Monitor,
    Lowest,
    Low,
    Normal,
    High,
    Highest,
}

impl Default for EventPriority {
    fn default() -> Self {
        EventPriority::Normal
    }
}

#[derive(PartialEq, Eq)]
pub struct EventHandler<E: TraitEvent> {
    priority: EventPriority,
    func: fn(&mut E) -> EventResult,
}