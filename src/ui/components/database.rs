use crate::db::config::Config;
use crate::db::config::DBConfig;
use crate::ui::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::HighlightSpacing;
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListItem, ListState, Padding, StatefulWidget,
};

pub struct DatabaseComponent {
    pub list_state: ListState,
}

impl DatabaseComponent {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn select_next(&mut self, dbs: &[DBConfig]) {
        let len = dbs.len();
        if len == 0 {
            self.list_state.select(None);
        } else {
            let next = match self.list_state.selected() {
                Some(i) if i + 1 < len => i + 1,
                _ => 0,
            };
            self.list_state.select(Some(next));
        }
    }

    pub fn select_previous(&mut self, dbs: &[DBConfig]) {
        let len = dbs.len();
        if len == 0 {
            self.list_state.select(None);
        } else {
            let prev = match self.list_state.selected() {
                Some(i) if i > 0 => i - 1,
                _ => len - 1,
            };
            self.list_state.select(Some(prev));
        }
    }

    /// Render the database selector
    pub fn render(
        &mut self,
        config: &Config,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        selected: bool,
        active_db_index: Option<usize>,
    ) {
        if self.list_state.selected() != active_db_index {
            self.list_state.select(active_db_index);
        }

        let command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled(" ↑↓ ", Theme::fg(&theme.foreground)),
            Span::styled("[a]", Theme::fg(&theme.accent)),
            Span::styled("dd", Theme::fg(&theme.foreground)),
            Span::styled(" [s]", Theme::fg(&theme.accent)),
            Span::styled("et Default", Theme::fg(&theme.foreground)),
            Span::styled(" [d]", Theme::fg(&theme.accent)),
            Span::styled("el", Theme::fg(&theme.foreground)),
            Span::styled(" [m]", Theme::fg(&theme.accent)),
            Span::styled("odify", Theme::fg(&theme.foreground)),
            Span::raw(" "),
        ])
        .left_aligned();

        let title_line = Line::from(vec![
            Span::raw("  D A T A B A S E "),
            Span::styled("[3]  ", Theme::fg(&theme.accent)),
        ])
        .left_aligned();

        let border_color = if selected {
            Theme::fg(&theme.border_accent)
        } else {
            Theme::fg(&theme.border)
        };

        let block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .title_top(title_line)
            .title_bottom(command_hints)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_color);

        let items: Vec<ListItem> = config
            .dbs
            .iter()
            .map(|db| ListItem::from(Span::styled(db.name.clone(), Theme::fg(&theme.foreground))))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(" ▸ ")
            .highlight_style(Theme::highlight(&theme, selected))
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }
}
