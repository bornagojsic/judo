pub struct DBSelector;
use crate::db::models::{NewTodoItem, TodoItem, UIItem, UIList};
use anyhow::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph,
    StatefulWidget, Widget,
};
use sqlx::SqlitePool;
use std::str::FromStr;

impl DBSelector {
    pub fn render(area: Rect, buf: &mut Buffer) {
        // Command hints for db
        let list_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "[C]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "hange",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            Span::raw(" "),
        ])
        .left_aligned();

        let block = Block::default()
            .padding(Padding::new(2, 2, 0, 0))
            .title_top(Line::raw("  D A T A B A S E  ").left_aligned())
            .title_bottom(list_command_hints)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new("dojo")
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
