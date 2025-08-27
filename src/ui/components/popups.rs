use crate::ui::cursor::CursorState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap};
use std::str::FromStr;

pub struct AddListPopup;

impl AddListPopup {
    /// Render popup for entering a new list name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer) {
        // Command hints for add list popup
        let add_list_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]);

        // Calculate popup dimensions
        let popup_width = (area.width * 3) / 4; // 75% of the area width
        let popup_height = 4; // Fixed height for just the input field

        // Center horizontally within the area
        let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;

        // Center vertically within the area
        let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        // Define the pop-up area
        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        // Clear the background of the popup area first
        Block::default()
            .style(Style::default().bg(Color::from_str("#002626").unwrap()))
            .render(popup_area, buf);

        // Define the popup block with styling
        let popup_block = Block::new()
            .title(" Add List ")
            .title_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()).bold())
            .title_bottom(add_list_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render
        let text_spans = state.create_cursor_text_spans();
        let text_line = Line::from(text_spans);

        // Render the input field
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
    }
}

pub struct AddItemPopup;

impl AddItemPopup {
    /// Render popup for entering a new item name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer) {
        // Command hints for add item popup
        let add_item_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]);

        // Calculate popup dimensions
        let popup_width = (area.width * 3) / 4; // 75% of the area width
        let popup_height = 4; // Fixed height for just the input field

        // Center horizontally within the area
        let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;

        // Center vertically within the area
        let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        // Define the pop-up area
        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        // Clear the background of the popup area first
        Block::default()
            .style(Style::default().bg(Color::from_str("#002626").unwrap()))
            .render(popup_area, buf);

        // Define the popup block with styling
        let popup_block = Block::new()
            .title(" Add Item ")
            .title_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()).bold())
            .title_bottom(add_item_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render
        let text_spans = state.create_cursor_text_spans();
        let text_line = Line::from(text_spans);

        // Render the input field
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
    }
}
