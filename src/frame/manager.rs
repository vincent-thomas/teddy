use crossterm::event::KeyEvent;

use super::Frame;

#[derive(Default)]
struct Cursor {
    position: (usize, usize)
}

#[derive(Default)]
pub struct FrameManager {
    // Attached lSPs here
    frames: Vec<Frame>,
    active_frame: usize,
    cursor_manager: Cursor,
}

impl FrameManager {
    pub fn new() -> Self {
        FrameManager {
            frames: vec![Frame::new()],
            active_frame: 0,
            cursor_manager: Cursor::default()
        }
    }

    pub fn send_input(&mut self, _key: KeyEvent) {
        println!("Sending input {}", _key.code);
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_manager.position
    }
}
