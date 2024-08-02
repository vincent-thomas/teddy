use std::sync::mpsc;


#[derive(Default, Debug, PartialEq, Eq)]
pub enum EditorMode {
    #[default]
    Normal(ModeHandler),
    Insert(ModeHandler),
    Visual(VisualModeKind),
    Command(CommandToExecute)
}

#[derive(Eq, PartialEq, Debug)]
struct ModeHandler {
    sender: mpsc::Sender<EditorAction>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualModeKind {
    Normal {
        selection_start: (usize, usize),
        selection_end: (usize, usize),
        mode_handler: ModeHandler
    },
    Line {
        line_from: usize,
        line_to: usize,
        mode_handler: ModeHandler
    },
}

impl Default for VisualModeKind {
    fn default() -> Self {
        Self::normal_at((0, 0))
    }
}

impl VisualModeKind {
    pub fn normal_at(cor: (usize, usize)) -> Self {
        VisualModeKind::Normal {
            selection_start: (cor.0, cor.1),
            selection_end: (cor.0 + 1, cor.1 + 1),
            mode_handler: ModeHandler
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CommandToExecute {
    raw: Vec<u8>
}

#[derive(Default)]
pub struct CommandBuffer {
    cmd: String,
    is_active: bool
}

impl CommandBuffer {
    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
    pub fn append(&mut self, c: char) {
        self.cmd.push(c);
    }

    fn set_inactive(&mut self) {
        self.is_active = false;
        self.cmd.clear();
    }

    pub fn execute(&mut self) -> String {
        let cmd = self.cmd.clone();
        self.set_inactive();
        cmd
    }
}

#[derive(Default)]
pub struct State {
    command_buffer: CommandBuffer,
    editor_mode: EditorMode
}

impl State {
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.editor_mode = mode;
    }
    pub fn get_mode(&self) -> &EditorMode {
        &self.editor_mode
    }

    pub fn cmd_mut(&mut self) -> &mut CommandBuffer {
        &mut self.command_buffer
    }
}
