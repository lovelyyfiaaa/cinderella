use std::{collections::HashMap, sync::mpsc, time::Duration};

use crate::{
    events::{events, Event},
    value::DynamicValue,
};

pub struct App {
    pub events: mpsc::Receiver<Event>,
    pub map: HashMap<String, DynamicValue>,
}

impl App {
    pub fn new(duration: Duration) -> Self {
        Self {
            events: events(duration),
            map: HashMap::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(Duration::from_millis(250))
    }
}
