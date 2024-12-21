use std::{alloc::Allocator, any::{Any, TypeId}, collections::{HashMap, HashSet}, hash::Hash, marker::PhantomData, sync::RwLock};

pub use super::events::{
    command::CommandEvent, 
    on_disable::EventOnDisable, 
    on_enable::EventOnEnable, 
    player_login::EventPlayerLogin
};



mod private {
    pub trait Sealed {}
}

pub enum Event {
    OnEnable { e: EventOnEnable },
    OnDisable { e: EventOnDisable },
    PlayerLogin { e: EventPlayerLogin },
    Command { e: CommandEvent },
}



pub struct EventManager {
    event_map: RwLock<HashMap<TypeId, Box<HandlerList<dyn TraitEvent>>>>,
}


impl EventManager {

    pub fn new() -> Self {
        Self { 
            event_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_event_map(&self) -> &RwLock<HashMap<TypeId, Box<HandlerList<dyn TraitEvent>>>> {
        &self.event_map
    }

    pub fn register_event_handler<E: TraitEvent + PartialEq + Clone + 'static + ?Sized>(&self, handler: EventHandler<E>) {
        let mut lock = self.event_map.write().unwrap();
        match lock.get_mut(&TypeId::of::<E>()) {
            Some(list) => {
                unsafe { &mut *(list as *mut dyn Any as *mut Box<HandlerList<E>>) }.register(handler);
            },
            None => {
                let mut list = HandlerList::new();
                list.register(handler);
                lock.insert(TypeId::of::<E>(), unsafe {Box::from_raw(Box::into_raw(Box::new(list)) as *mut HandlerList<dyn TraitEvent>)});
            }
        }
    }
}



pub fn listen<E: TraitEvent + Clone + 'static>(manager: &EventManager, e: &mut E) -> EventResult {
    match manager.get_event_map().read().unwrap().get(&TypeId::of::<E>()) {
        Some(ref_box) => {
            let list = unsafe { &*(ref_box as *const dyn Any as *const Box<HandlerList<E>>) };
            let mut result: EventResult = EventResult::Default;
            
            fn handle<E: TraitEvent + Clone>(
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
            
            for handler in list.lowest.iter() {
                handle(handler, &mut result, e);
            }
            for handler in list.low.iter() {
                handle(handler, &mut result, e);
            }
            for handler in list.normal.iter() {
                handle(handler, &mut result, e);
            }
            for handler in list.high.iter() {
                handle(handler, &mut result, e);
            }
            for handler in list.highest.iter() {
                handle(handler, &mut result, e);
            }                
            for handler in list.monitor.iter() {
                handle(handler, &mut result, e);
            }
            result
        },
        None => EventResult::Default,
    }
}


pub struct HandlerList<E: TraitEvent + ?Sized> {
    highest: Vec<EventHandler<E>>,
    high: Vec<EventHandler<E>>,
    normal: Vec<EventHandler<E>>,
    low: Vec<EventHandler<E>>,
    lowest: Vec<EventHandler<E>>,
    monitor: Vec<EventHandler<E>>,
}

impl<E: 'static + TraitEvent + PartialEq + ?Sized> HandlerList<E> {
    pub fn new() -> Self {
        Self { 
            highest: Vec::new(), 
            high: Vec::new(), 
            normal: Vec::new(), 
            low: Vec::new(), 
            lowest: Vec::new(), 
            monitor: Vec::new()
        }
    }
    
    pub fn get_handlers(&self, priority: &EventPriority) -> &Vec<EventHandler<E>> {
        match priority {
            EventPriority::Monitor => &self.monitor,
            EventPriority::Lowest => &self.lowest,
            EventPriority::Low => &self.low,
            EventPriority::Normal => &self.normal,
            EventPriority::High => &self.high,
            EventPriority::Highest => &self.highest,
        }
    }

    pub fn get_handlers_mut(&mut self, priority: &EventPriority) -> &mut Vec<EventHandler<E>> {
        match priority {
            EventPriority::Monitor => &mut self.monitor,
            EventPriority::Lowest => &mut self.lowest,
            EventPriority::Low => &mut self.low,
            EventPriority::Normal => &mut self.normal,
            EventPriority::High => &mut self.high,
            EventPriority::Highest => &mut self.highest,
        }
    }

    pub fn register(&mut self, handler: EventHandler<E>) {
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
}

impl<E: TraitEvent> private::Sealed for HandlerList<E> {}

impl<E: TraitEvent> HandlerList<E>
{
    fn unregister_all(&mut self) {
        self.monitor.clear();
        self.lowest.clear();
        self.low.clear();
        self.normal.clear();
        self.high.clear();
        self.highest.clear();
    }
}


#[derive(PartialEq, Eq)]
pub struct EventHandler<E: TraitEvent + ?Sized> {
    priority: EventPriority,
    func: fn(&mut E) -> EventResult,
}


impl<E: TraitEvent> EventHandler<E> {
    pub fn new(priority: EventPriority, func: fn(&mut E) -> EventResult) -> Self {
        Self { 
            priority: priority, 
            func: func,
        }
    }
}



pub trait TraitEvent: std::fmt::Debug {}

impl PartialEq for dyn TraitEvent {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}


impl Hash for dyn TraitEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
    }
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

pub enum EventResult {
    Deny,
    Default,
    Allow,
}