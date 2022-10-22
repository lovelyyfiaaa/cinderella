use std::{io, sync::mpsc, thread};

use termion::{event::Key, input::TermRead};

pub enum Event {
  Input(Key),
  Tick,
}

///
/// Listen to
///
/// This is copied from the Termion temo of TUI
/// https://github.com/fdehau/tui-rs/blob/a6b25a487786534205d818a76acb3989658ae58c/examples/demo/termion.rs#L60
///
pub fn events(tick_rate: std::time::Duration) -> mpsc::Receiver<Event> {
  let (tx, rx) = mpsc::channel();
  let keys_tx = tx.clone();
  thread::spawn(move || {
    let stdin = io::stdin();
    for key in stdin.keys().flatten() {
      if let Err(err) = keys_tx.send(Event::Input(key)) {
        eprintln!("{}", err);
        return;
      }
    }
  });
  thread::spawn(move || loop {
    if let Err(err) = tx.send(Event::Tick) {
      eprintln!("{}", err);
      break;
    }
    thread::sleep(tick_rate);
  });
  rx
}
