use crate::db::models::{NewTodoItem, TodoItem, UIItem, UIList};
use crate::ui::theme::Theme;
use anyhow::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, Padding, StatefulWidget, Widget,
};
use sqlx::SqlitePool;
use textwrap::wrap;

pub struct ItemsComponent;

impl ItemsComponent {
    /// Apply styling to a todo item based on its completion status
    fn style_item(ui_item: &UIItem, selected_index: i32, theme: Theme, selected: bool) -> Line<'_> {
        let name = ui_item.item.name.clone();

        let item_index = ui_item.item.ordering as i32 - 1;

        fn get_rel_index(current_index: i32, selected_index: i32) -> String {
            if current_index == selected_index {
                format!("{}  ", current_index + 1)
            } else {
                format!("{}  ", (current_index - selected_index).abs())
            }
        }

        let rel_index = get_rel_index(item_index, selected_index);

        let rel_num_span = if item_index - selected_index == 0 && selected {
            Span::styled(rel_index, Theme::line_number(&theme))
        } else if item_index - selected_index == 0 {
            Span::styled(rel_index, Theme::fg(&theme.highlight_not_focused_fg))
        } else {
            Span::from(rel_index)
        };

        if ui_item.item.is_done {
            Line::from(vec![
                rel_num_span,
                Span::styled(name, Style::default().add_modifier(Modifier::CROSSED_OUT)),
            ])
        } else {
            Line::from(vec![rel_num_span, Span::from(name)])
        }
    }

    /// Select next element in the list of to-do items
    pub fn select_next_item(ui_list: &mut UIList) {
        ui_list.item_state.select_next();
    }

    /// Select next element in the list of to-do items by a specified amount
    pub fn scroll_down_by(ui_list: &mut UIList, amount: usize) {
        ui_list.item_state.scroll_down_by(amount as u16);
    }

    /// Select previous element in the list of to-do items
    pub fn select_previous_item(ui_list: &mut UIList) {
        ui_list.item_state.select_previous();
    }

    /// Select previous element in the list of to-do items by a specified amount
    pub fn scroll_up_by(ui_list: &mut UIList, amount: usize) {
        ui_list.item_state.scroll_up_by(amount as u16);
    }

    /// Remove item selection (deselect current item)
    pub fn remove_item_selection(ui_list: &mut UIList) {
        ui_list.item_state.select(None);
    }

    pub fn select_first(ui_list: &mut UIList) {
        ui_list.item_state.select_first();
    }

    pub fn select_last(ui_list: &mut UIList) {
        ui_list.item_state.select_last();
    }

    /// Select the first item in the list
    pub fn select_first_item(ui_list: &mut UIList) {
        if ui_list.item_state.selected().is_none() {
            ui_list.item_state.select_first();
        }
    }

    /// Select the last item in the list
    pub fn select_last_item(ui_list: &mut UIList) {
        if ui_list.item_state.selected().is_none() {
            ui_list.item_state.select_last();
        }
    }

    /// Toggle the "is done" status of the currently selected item
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

    /// Update an existing item
    pub async fn update_item(ui_list: &mut UIList, name: String, pool: &SqlitePool) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let mut item = ui_list.items[j].item.clone();
            item.update_name(pool, name).await?;

            // Update list elements
            ui_list.update_items(pool).await?;
        }
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

    /// Move the currently selected item up
    pub async fn move_selected_item_up(ui_list: &mut UIList, pool: &SqlitePool) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let mut item = ui_list.items[j].item.clone();
            item.move_up(pool).await?;

            // Update list elements to reflect the new order
            ui_list.update_items(pool).await?;

            // Adjust selection to follow the moved item
            if j > 0 {
                ui_list.item_state.select(Some(j - 1));
            }
        }
        Ok(())
    }

    pub async fn move_selected_item_up_by(
        ui_list: &mut UIList,
        pool: &SqlitePool,
        amount: usize,
    ) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let real_amount = if amount > j { j } else { amount };

            let mut item = ui_list.items[j].item.clone();
            item.move_up_by(pool, real_amount).await?;

            // Update list elements to reflect the new order
            ui_list.update_items(pool).await?;

            // Adjust selection to follow the moved item
            if j > 0 {
                ui_list.item_state.select(Some(j - real_amount));
            }
        }
        Ok(())
    }

    /// Move the currently selected item down
    pub async fn move_selected_item_down(ui_list: &mut UIList, pool: &SqlitePool) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let mut item = ui_list.items[j].item.clone();
            item.move_down(pool).await?;

            // Update list elements to reflect the new order
            ui_list.update_items(pool).await?;

            // Adjust selection to follow the moved item
            if j + 1 < ui_list.items.len() {
                ui_list.item_state.select(Some(j + 1));
            }
        }
        Ok(())
    }

    pub async fn move_selected_item_down_by(
        ui_list: &mut UIList,
        pool: &SqlitePool,
        amount: usize,
    ) -> Result<()> {
        if let Some(j) = ui_list.item_state.selected() {
            let real_amount = if amount > ui_list.items.len() - j - 1 {
                ui_list.items.len() - j - 1
            } else {
                amount
            };

            let mut item = ui_list.items[j].item.clone();
            item.move_down_by(pool, real_amount).await?;

            // Update list elements to reflect the new order
            ui_list.update_items(pool).await?;

            // Adjust selection to follow the moved item
            if j + real_amount < ui_list.items.len() {
                ui_list.item_state.select(Some(j + real_amount));
            }
        }
        Ok(())
    }

    /// Render the list of todo items for the selected list
    pub fn render(
        selected_list: Option<&mut UIList>,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        selected: bool,
        recent_keys_str: String,
    ) {
        // Command hints for items
        let list_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled(" ↓↑ ", Style::default()),
            Span::styled("[a]", Theme::fg(&theme.accent)),
            Span::styled("dd", Theme::fg(&theme.foreground)),
            Span::styled(" [d]", Theme::fg(&theme.accent)),
            Span::styled("el", Theme::fg(&theme.foreground)),
            Span::styled(" [m]", Theme::fg(&theme.accent)),
            Span::styled("odify ", Theme::fg(&theme.foreground)),
            Span::raw(" "),
        ])
        .left_aligned();

        // Add "quit" hint, in the bottom right corner
        let quit_hint = Line::from(vec![
            Span::raw(" "),
            Span::raw(recent_keys_str),
            Span::raw(" "),
            Span::styled("[Ctrl + h]", Theme::fg(&theme.accent)),
            Span::styled("elp ", Theme::fg(&theme.foreground)),
            Span::raw(" "),
            Span::styled("[q]", Theme::fg(&theme.accent)),
            Span::styled("uit ", Theme::fg(&theme.foreground)),
            Span::raw(" "),
        ])
        .right_aligned();

        let title_line = Line::from(vec![
            Span::raw("  I T E M S "),
            Span::styled("[SPACE + 2]  ", Theme::fg(&theme.accent)),
        ])
        .left_aligned();

        let border_color = if selected {
            Theme::fg(&theme.border_accent)
        } else {
            Theme::fg(&theme.border)
        };

        let block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .title_top(title_line.left_aligned())
            .title_bottom(list_command_hints)
            .title_bottom(quit_hint)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_color);

        let inner = block.inner(area);
        let width = inner.width as usize;

        if let Some(ui_list) = selected_list {
            if ui_list.items.is_empty() {
                // Render an empty block or a message
                block.render(area, buf);
                return;
            }

            let selected_index: i32 = match ui_list.item_state.selected() {
                Some(idx) => {
                    if idx < ui_list.items.len() {
                        idx as i32
                    } else {
                        (idx - 1) as i32
                    }
                }
                None => -1,
            };

            let max_index = ui_list.items.len();

            let max_index_digits = max_index.to_string().len();

            // Extract the corresponding items with styling
            let items: Vec<ListItem> = ui_list
                .items
                .iter()
                .map(|ui_item| {
                    let styled_line =
                        Self::style_item(ui_item, selected_index, theme.to_owned(), selected);

                    // Assume styled_line.spans[0] and styled_line.spans[1] exist
                    let padding = "   "; // 3 spaces, adjust as needed

                    // Wrap each span individually
                    let wrapped_line_number = wrap(
                        &styled_line.spans[0].content,
                        width.saturating_sub(max_index_digits + 5),
                    );
                    let wrapped_item = wrap(
                        &styled_line.spans[1].content,
                        width.saturating_sub(max_index_digits + 5),
                    );

                    // Find the max number of lines
                    let max_lines = std::cmp::max(wrapped_line_number.len(), wrapped_item.len());

                    let mut lines: Vec<Line> = Vec::with_capacity(max_lines);

                    let newline_symbol = "↪ ";

                    for i in 0..max_lines {
                        let mut line_spans = Vec::new();

                        // First span (with style)
                        if let Some(w) = wrapped_line_number.get(i) {
                            let line_number_padding =
                                " ".repeat(max_index_digits.saturating_sub(w.len()));
                            line_spans.push(Span::styled(
                                line_number_padding.to_string() + w,
                                styled_line.spans[0].style,
                            ));
                        }

                        // Padding span
                        line_spans.push(Span::raw(padding));

                        // Second span (with style)
                        if let Some(w) = wrapped_item.get(i) {
                            if i > 0 {
                                line_spans.push(Span::styled(
                                    newline_symbol,
                                    Theme::color_from_hex("#565f89"),
                                ));
                            }
                            line_spans
                                .push(Span::styled(w.to_string(), styled_line.spans[1].style));
                        }

                        lines.push(Line::from(line_spans));
                    }

                    ListItem::from(lines)
                })
                .collect();

            let list: List = List::new(items)
                .block(block)
                .highlight_style(Theme::bg(match selected {
                    true => &theme.highlight_bg,
                    false => &theme.highlight_not_focused_bg,
                }))
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, &mut ui_list.item_state);
        } else {
            // No list selected - render empty block
            block.render(area, buf);
        }
    }
}
