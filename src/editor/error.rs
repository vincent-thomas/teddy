use crate::state::EditorMode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Invalid mode switch: {0:?}")]
    InvalidmodeSwitch(EditorMode)
}
