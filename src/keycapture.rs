use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event as CEvent, KeyEvent};

pub struct KeyCaptureManager {
    sender: mpsc::Sender<KeyEvent>
}


impl KeyCaptureManager {
    pub fn new(sender: mpsc::Sender<KeyEvent>) -> Self {
        Self {
            sender
        }
    }
    pub fn start(&self) {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| std::time::Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    self.sender.send(key).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                //if let Ok(_) = self.sender.send(Event::Tick) {
                last_tick = Instant::now();
                //}
            }
        }
    }
}   
