pub struct DBSelector;
use crate::ui::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget};

impl DBSelector {
    pub fn render(area: Rect, buf: &mut Buffer, current_db_name: &str, theme: &Theme) {
        // Command hints for db
        let list_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[C]", Theme::fg(&theme.accent)),
            Span::styled("hange", Theme::fg(&theme.foreground)),
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

        Paragraph::new(current_db_name)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
