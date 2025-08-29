use crate::db::models::{NewTodoList, TodoList, UIList};
use anyhow::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
    StatefulWidget,
};
use sqlx::SqlitePool;
use std::str::FromStr;

pub struct ListsComponent {
    pub lists: Vec<UIList>,
    pub list_state: ListState,
}

impl Default for ListsComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ListsComponent {
    pub fn new() -> Self {
        Self {
            lists: Vec::new(),
            list_state: ListState::default(),
        }
    }

    /// Initialize lists from database
    pub async fn load_lists(&mut self, pool: &SqlitePool) -> Result<()> {
        self.lists = UIList::get_all(pool).await?;
        Ok(())
    }

    /// Select next element in the list of to-do lists
    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    /// Select previous element in the list of to-do lists
    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    /// Get currently selected list index
    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    /// Create a new list and refresh data
    pub async fn create_list(&mut self, name: String, pool: &SqlitePool) -> Result<()> {
        let new_list = NewTodoList { name };
        TodoList::create(pool, new_list).await?;
        self.load_lists(pool).await?;
        Ok(())
    }

    /// Delete the currently selected list
    pub async fn delete_selected_list(&mut self, pool: &SqlitePool) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            let list = self.lists[i].list.clone();
            list.delete(pool).await?;

            // Refresh the lists from database
            self.load_lists(pool).await?;

            // Adjust selection after deletion
            if self.lists.is_empty() {
                self.list_state.select(None);
            } else if i >= self.lists.len() {
                self.list_state.select(Some(self.lists.len() - 1));
            }
        }
        Ok(())
    }

    /// Get the currently selected list (mutable)
    pub fn get_selected_list_mut(&mut self) -> Option<&mut UIList> {
        if let Some(i) = self.list_state.selected() {
            self.lists.get_mut(i)
        } else {
            None
        }
    }

    /// Get the currently selected list (immutable)
    pub fn get_selected_list(&self) -> Option<&UIList> {
        if let Some(i) = self.list_state.selected() {
            self.lists.get(i)
        } else {
            None
        }
    }

    /// Render the list of todo lists
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // Command hints for lists
        let list_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled(" w,s ", Style::default()),
            Span::styled(
                "[A]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "dd",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            Span::styled(
                " [D]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "el ",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            Span::raw(" "),
        ])
        .left_aligned();

        let block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .title_top(Line::raw("  L I S T S  ").left_aligned())
            .title_bottom(list_command_hints)
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_type(BorderType::Rounded);

        // Convert lists to display items
        let items: Vec<ListItem> = self
            .lists
            .iter()
            .map(|ui_list| ListItem::from(ui_list.list.name.clone()))
            .collect();

        let list: List = List::new(items)
            .block(block)
            .highlight_symbol(" ▸ ") // Selection indicator
            .highlight_style(
                // Swap foreground and background for selected item
                Style::default()
                    .bg(Color::from_str("#FCF1D5").unwrap())
                    .fg(Color::from_str("#002626").unwrap()),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }
}
