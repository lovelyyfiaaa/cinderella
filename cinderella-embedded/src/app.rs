use crate::value::DynamicValue;
use cinderella_terminal::backend::auto::Auto;
use cinderella_terminal::events::Event;
use cinderella_terminal::interface::Backend;
use std::{collections::HashMap, sync::mpsc, time::Duration};

pub struct App {
    pub events: mpsc::Receiver<Event>,
    pub map: HashMap<String, DynamicValue>,
}

impl App {
    pub fn new(duration: Duration) -> Self {
        Self {
            events: Auto::events(duration),
            map: HashMap::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(Duration::from_millis(250))
    }
}
