use crate::ui::cursor::CursorState;

/// State of new list being added but not finalized
#[derive(Debug, Clone)]
pub struct NewListState {
    /// Buffer for new list name input
    pub current_new_list_name: String,
    /// Position of cursor
    pub cursor_pos: usize,
}

impl Default for NewListState {
    fn default() -> Self {
        Self::new()
    }
}

impl NewListState {
    pub fn new() -> Self {
        Self {
            current_new_list_name: String::new(),
            cursor_pos: 0,
        }
    }
}

impl CursorState for NewListState {
    fn get_text(&self) -> &str {
        &self.current_new_list_name
    }

    fn get_text_mut(&mut self) -> &mut String {
        &mut self.current_new_list_name
    }

    fn get_cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    fn set_cursor_pos(&mut self, pos: usize) {
        self.cursor_pos = pos;
    }
}

/// State of new item being added but not finalized
#[derive(Debug, Clone)]
pub struct NewItemState {
    /// Buffer for new item name input
    pub current_new_item_name: String,
    /// Position of cursor
    pub cursor_pos: usize,
}

impl Default for NewItemState {
    fn default() -> Self {
        Self::new()
    }
}

impl NewItemState {
    pub fn new() -> Self {
        Self {
            current_new_item_name: String::new(),
            cursor_pos: 0,
        }
    }
}

impl CursorState for NewItemState {
    fn get_text(&self) -> &str {
        &self.current_new_item_name
    }

    fn get_text_mut(&mut self) -> &mut String {
        &mut self.current_new_item_name
    }

    fn get_cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    fn set_cursor_pos(&mut self, pos: usize) {
        self.cursor_pos = pos;
    }
}
