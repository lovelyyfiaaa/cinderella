use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::mpsc, time::Duration};



use crate::{
    events::{events, Event},
    value::DynamicValue,
};

pub struct App {
    pub events: mpsc::Receiver<Event>,
    pub map: HashMap<String, DynamicValue>,
}

impl App {
    pub fn new() -> Self {
        Self {
            events: events(Duration::from_millis(250)),
            map: HashMap::new(),
        }
    }
}
