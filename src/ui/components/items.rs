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

pub struct ItemsComponent;

impl ItemsComponent {
    /// Apply styling to a todo item based on its completion status
    fn style_item(ui_item: &UIItem) -> Span<'_> {
        let name = ui_item.item.name.clone();

        if ui_item.item.is_done {
            // Strike through completed items
            Span::styled(name, Style::default().add_modifier(Modifier::CROSSED_OUT))
        } else {
            Span::from(name)
        }
    }

    /// Select next element in the list of to-do items
    pub fn select_next_item(ui_list: &mut UIList) {
        ui_list.item_state.select_next();
    }

    /// Select previous element in the list of to-do items
    pub fn select_previous_item(ui_list: &mut UIList) {
        ui_list.item_state.select_previous();
    }

    /// Remove item selection (deselect current item)
    pub fn remove_item_selection(ui_list: &mut UIList) {
        ui_list.item_state.select(None);
    }

    /// Select the first item in the list
    pub fn select_first_item(ui_list: &mut UIList) {
        if ui_list.item_state.selected().is_none() {
            ui_list.item_state.select_first();
        }
    }

    /// Toggle the "is_done" status of the currently selected item
    pub async fn toggle_item_done(ui_list: &mut UIList, pool: &SqlitePool) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            ui_list.items[j].item.toggle_done(pool).await?;
        }
        Ok(())
    }

    /// Create a new item in the given list
    pub async fn create_item(ui_list: &mut UIList, name: String, pool: &SqlitePool) -> Result<()> {
        let new_item = NewTodoItem {
            name,
            list_id: ui_list.list.id,
            priority: None,
            due_date: None,
        };

        TodoItem::create(pool, new_item).await?;
        ui_list.update_items(pool).await?;
        Ok(())
    }

    /// Delete the currently selected item
    pub async fn delete_selected_item(ui_list: &mut UIList, pool: &SqlitePool) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let item = ui_list.items[j].item.clone();
            item.delete(pool).await?;

            // Update list elements
            ui_list.update_items(pool).await?;

            // Adjust selection after deletion - check bounds first
            if ui_list.items.is_empty() {
                ui_list.item_state.select(None);
            } else if j >= ui_list.items.len() {
                ui_list.item_state.select(Some(ui_list.items.len() - 1));
            }
        }
        Ok(())
    }

    /// Render the list of todo items for the selected list
    pub fn render(selected_list: Option<&mut UIList>, area: Rect, buf: &mut Buffer) {
        // Command hints for items
        let list_command_hints = Line::from(vec![
            Span::styled(" ↓↑ ", Style::default()),
            Span::styled(
                "[a]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "dd",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            Span::styled(
                " [d]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "el ",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]);

        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Items ").centered())
            .title_bottom(list_command_hints)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        if let Some(ui_list) = selected_list {
            // Extract the corresponding items with styling
            let items: Vec<ListItem> = ui_list
                .items
                .iter()
                .map(|ui_item| ListItem::from(Self::style_item(ui_item)))
                .collect();

            let list: List = List::new(items)
                .block(block)
                .highlight_symbol(" ▸ ")
                .highlight_style(
                    // Swap foreground and background for selected item
                    Style::default()
                        .bg(Color::from_str("#FCF1D5").unwrap())
                        .fg(Color::from_str("#002626").unwrap()),
                )
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, &mut ui_list.item_state);
        } else {
            // No list selected - show instruction message
            Paragraph::new(Span::styled(
                "Select or add a to-do list first",
                Style::default().italic(),
            ))
            .left_aligned()
            .block(block)
            .render(area, buf);
        }
    }
}
