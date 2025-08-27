use ratatui::style::{Color, Style};
use ratatui::text::Span;
use std::str::FromStr;

/// Trait for managing cursor-based text input
pub trait CursorState {
    /// Get the current text content
    fn get_text(&self) -> &str;

    /// Get the current cursor position
    fn get_cursor_pos(&self) -> usize;

    /// Set the cursor position
    fn set_cursor_pos(&mut self, pos: usize);

    /// Get a mutable reference to the text content
    fn get_text_mut(&mut self) -> &mut String;

    /// Add a character at the cursor position
    fn add_char(&mut self, c: char) {
        let pos = self.get_cursor_pos();
        self.get_text_mut().insert(pos, c);
        self.set_cursor_pos(pos + 1);
    }

    /// Remove the character before the cursor (backspace)
    fn remove_char_before_cursor(&mut self) {
        let pos = self.get_cursor_pos();
        if pos > 0 {
            self.get_text_mut().remove(pos - 1);
            self.set_cursor_pos(pos - 1);
        }
    }

    /// Delete the character after the cursor (delete key)
    fn delete_char_after_cursor(&mut self) {
        let pos = self.get_cursor_pos();
        let text_len = self.get_text().chars().count();

        if pos < text_len {
            let mut chars: Vec<char> = self.get_text().chars().collect();
            chars.remove(pos);
            *self.get_text_mut() = chars.into_iter().collect();
        }
    }

    /// Move cursor left
    fn move_cursor_left(&mut self) {
        let pos = self.get_cursor_pos();
        if pos > 0 {
            self.set_cursor_pos(pos - 1);
        }
    }

    /// Move cursor right
    fn move_cursor_right(&mut self) {
        let pos = self.get_cursor_pos();
        let text_len = self.get_text().chars().count();
        if pos < text_len {
            self.set_cursor_pos(pos + 1);
        }
    }

    /// Clear the text and reset cursor
    fn clear(&mut self) {
        self.get_text_mut().clear();
        self.set_cursor_pos(0);
    }

    /// Create text spans for rendering with cursor visualization
    fn create_cursor_text_spans(&self) -> Vec<Span<'static>> {
        let text = self.get_text();
        let cursor_pos = self.get_cursor_pos();
        let chars: Vec<char> = text.chars().collect();
        let text_len = chars.len();

        // Ensure cursor position is within bounds
        let safe_cursor_pos = cursor_pos.min(text_len);

        // Text before cursor
        let text_before: String = chars[..safe_cursor_pos].iter().collect();

        // Character at cursor position (or space if at end)
        let cursor_char = if safe_cursor_pos >= text_len {
            "█".to_string()
        } else {
            chars[safe_cursor_pos].to_string()
        };

        // Text after cursor
        let text_after: String = if safe_cursor_pos >= text_len {
            String::new()
        } else {
            chars[(safe_cursor_pos + 1)..].iter().collect()
        };

        vec![
            Span::styled(
                text_before,
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            if cursor_char == "█" {
                Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(Color::from_str("#FCF1D5").unwrap())
                        .bg(Color::from_str("#002626").unwrap()),
                )
            } else {
                Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(Color::from_str("#002626").unwrap())
                        .bg(Color::from_str("#FCF1D5").unwrap()),
                )
            },
            Span::styled(
                text_after,
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]
    }
}
