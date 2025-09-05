use crate::ui::cursor::CursorState;

/// State of input from user
#[derive(Debug, Clone)]
pub struct InputState {
    /// Buffer for input string
    pub current_input: String,
    /// Position of cursor
    pub cursor_pos: usize,
    /// ID of object (optional)
    pub id: Option<i64>
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            cursor_pos: 0,
            id: None,
        }
    }
}

impl CursorState for InputState {
    fn get_text(&self) -> &str {
        &self.current_input
    }

    fn get_text_mut(&mut self) -> &mut String {
        &mut self.current_input
    }

    fn get_cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    fn set_cursor_pos(&mut self, pos: usize) {
        self.cursor_pos = pos;
    }
}