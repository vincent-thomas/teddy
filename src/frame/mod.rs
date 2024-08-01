use crate::RawBuffer;

pub mod manager;

enum FrameModeAnchor {
    Top,
    Center,
    Bottom,
}

enum FrameMode {
    Floating {
        anchor: FrameModeAnchor,
    },
    Fullscreen
}

struct Frame {
    buffer: RawBuffer,
    frame_mode: FrameMode
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            buffer: RawBuffer::default(),
            frame_mode: FrameMode::Fullscreen
        }
    }
}
