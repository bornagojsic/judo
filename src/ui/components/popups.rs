use crate::ui::cursor::CursorState;
use crate::ui::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget, Wrap};

pub struct AddListPopUp;
pub struct ModifyListPopUp;

fn render_list_popup_kernel<T: CursorState>(
    state: &T,
    area: Rect,
    buf: &mut Buffer,
    popup_title: &str,
    theme: &Theme,
) {
    // Command hints for add list popup
    let add_or_modify_list_command_hints = Line::from(vec![
        Span::raw(" "),
        Span::styled("[Esc]", Theme::fg(&theme.accent)),
        Span::raw(" "),
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
    Clear.render(popup_area, buf);
    Block::default()
        .style(Theme::bg(&theme.background))
        .render(popup_area, buf);

    // Define the popup block with styling
    let popup_block = Block::new()
        .padding(Padding::new(2, 2, 1, 1))
        .title(format!("  {}  ", popup_title))
        .title_style(Theme::fg(&theme.foreground))
        .title_bottom(add_or_modify_list_command_hints)
        .borders(Borders::ALL)
        .border_style(Theme::fg(&theme.border))
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    // Define the text to render
    let text_spans = state.create_cursor_text_spans(&theme);
    let text_line = Line::from(text_spans);

    // Render the input field
    Paragraph::new(text_line)
        .wrap(Wrap { trim: true })
        .block(popup_block)
        .render(popup_area, buf);
}

impl AddListPopUp {
    /// Render popup for entering a new list name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        render_list_popup_kernel(state, area, buf, "Add List", theme);
    }
}

impl ModifyListPopUp {
    /// Render popup for entering a new list name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        render_list_popup_kernel(state, area, buf, "Modify List", theme);
    }
}

pub struct AddItemPopUp;
pub struct ModifyItemPopUp;

/// Render popup for entering a new item name
pub fn render_item_popup_kernel<T: CursorState>(
    state: &T,
    area: Rect,
    buf: &mut Buffer,
    popup_title: &str,
    theme: &Theme,
) {
    // Command hints for add item popup
    let add_item_command_hints = Line::from(vec![
        Span::raw(" "),
        Span::styled("[Esc]", Theme::fg(&theme.foreground)),
        Span::raw(" "),
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
    Clear.render(popup_area, buf);
    Block::default()
        .style(Theme::bg(&theme.background))
        .render(popup_area, buf);

    // Define the popup block with styling
    let popup_block = Block::new()
        .padding(Padding::new(2, 2, 1, 1))
        .title(format!("  {}  ", popup_title))
        .title_style(Theme::fg(&theme.foreground))
        .title_bottom(add_item_command_hints)
        .borders(Borders::ALL)
        .border_style(Theme::fg(&theme.border))
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    // Define the text to render
    let text_spans = state.create_cursor_text_spans(&theme);
    let text_line = Line::from(text_spans);

    // Render the input field
    Paragraph::new(text_line)
        .wrap(Wrap { trim: true })
        .block(popup_block)
        .render(popup_area, buf);
}

impl AddItemPopUp {
    /// Render popup for entering a new item
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        render_item_popup_kernel(state, area, buf, "Add Item", theme);
    }
}

impl ModifyItemPopUp {
    /// Render popup for modifying item name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        render_item_popup_kernel(state, area, buf, "Modify Item", theme);
    }
}

pub struct AddDBPopUp;

impl AddDBPopUp {
    /// Render popup for entering a new database name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Command hints for add db popup
        let add_db_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[Esc]", Theme::fg(&theme.accent)),
            Span::raw(" "),
        ]);

        // Clear the entire area background first
        Clear.render(area, buf);
        Block::default()
            .style(Theme::fg_bg(&theme.foreground, &theme.background))
            .render(area, buf);

        // Define the popup block with styling - use full width
        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(" Add Database ")
            .title_style(Theme::fg(&theme.foreground))
            .title_bottom(add_db_command_hints)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render
        let text_spans = state.create_cursor_text_spans(&theme);
        let text_line = Line::from(text_spans);

        // Render the input field using the full area
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(area, buf);
    }
}

pub struct ModifyDBPopUp;

impl ModifyDBPopUp {
    /// Render popup for modifying a database name
    pub fn render<T: CursorState>(state: &T, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Command hints for modify db popup
        let modify_db_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[Esc]", Theme::fg(&theme.accent)),
            Span::raw(" "),
        ]);

        // Clear the entire area background first
        Clear.render(area, buf);
        Block::default()
            .style(Theme::fg_bg(&theme.foreground, &theme.background))
            .render(area, buf);

        // Define the popup block with styling - use full width
        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(" Modify Database ")
            .title_style(Theme::fg(&theme.foreground))
            .title_bottom(modify_db_command_hints)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render
        let text_spans = state.create_cursor_text_spans(&theme);
        let text_line = Line::from(text_spans);

        // Render the input field using the full area
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(area, buf);
    }
}

pub struct HelpPopUp;

impl HelpPopUp {
    /// Render popup for displaying help information
    pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Help text lines
        let mut help_lines = vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "  General",
                Theme::fg(&theme.highlight_fg),
            )]),
            Line::from(vec![
                Span::styled("    * <count> ↑/k", Theme::fg(&theme.accent)),
                Span::raw(" - Select the previous list/item/database (count is optional)"),
            ]),
            Line::from(vec![
                Span::styled("    * <count> ↓/j", Theme::fg(&theme.accent)),
                Span::raw(" - Select the next list/item/database (count is optional)"),
            ]),
            Line::from(vec![
                Span::styled("    * SHIFT + ↑/k", Theme::fg(&theme.accent)),
                Span::raw(" - Move the selected list/item up"),
            ]),
            Line::from(vec![
                Span::styled("    * SHIFT + ↓/j", Theme::fg(&theme.accent)),
                Span::raw(" - Move the selected list/item down"),
            ]),
            Line::from(vec![
                Span::styled("    * q", Theme::fg(&theme.accent)),
                Span::raw(" - Quit"),
            ]),
        ];

        let list_selection_help_lines = vec![
            Line::from(vec![
                Span::styled("  SPACE + 1", Theme::fg(&theme.accent)),
                Span::raw(" - Go to "),
                Span::styled("List Selection", Theme::fg(&theme.highlight_fg)),
            ]),
            Line::from(vec![
                Span::styled("    * →/l", Theme::fg(&theme.accent)),
                Span::raw(" - Go to Item Selection"),
            ]),
        ];

        let item_selection_help_lines = vec![
            Line::from(vec![
                Span::styled("  SPACE + 2", Theme::fg(&theme.accent)),
                Span::raw(" - Go to "),
                Span::styled("Item Selection", Theme::fg(&theme.highlight_fg)),
            ]),
            Line::from(vec![
                Span::styled("    * Enter", Theme::fg(&theme.accent)),
                Span::raw(" - Toggle the current item"),
            ]),
            Line::from(vec![
                Span::styled("    * ←/h", Theme::fg(&theme.accent)),
                Span::raw(" - Go to List Selection"),
            ]),
            Line::from(vec![
                Span::styled("    * g g", Theme::fg(&theme.accent)),
                Span::raw(" - Go to the first item"),
            ]),
            Line::from(vec![
                Span::styled("    * G", Theme::fg(&theme.accent)),
                Span::raw(" - Go to the last item"),
            ]),
        ];

        let db_selection_help_lines = vec![
            Line::from(vec![
                Span::styled("  SPACE + 3", Theme::fg(&theme.accent)),
                Span::raw(" - Go to "),
                Span::styled("Database Selection", Theme::fg(&theme.highlight_fg)),
            ]),
            Line::from(vec![
                Span::styled("    * Enter", Theme::fg(&theme.accent)),
                Span::raw(" - Open the selected database"),
            ]),
        ];

        help_lines.push(Line::from(Span::raw("")));
        help_lines.extend(list_selection_help_lines);
        help_lines.push(Line::from(Span::raw("")));
        help_lines.extend(item_selection_help_lines);
        help_lines.push(Line::from(Span::raw("")));
        help_lines.extend(db_selection_help_lines);

        // Command hints for help popup
        let help_command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[Esc]", Theme::fg(&theme.accent)),
            Span::raw(" - Close this popup "),
        ]);

        // Calculate popup dimensions
        let popup_width = (area.width * 2) / 3; // 66% of the area width
        let popup_height = help_lines.len() as u16 + 4; // Enough for all help lines + padding

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
        Clear.render(popup_area, buf);
        Block::default()
            .style(Theme::bg(&theme.background))
            .render(popup_area, buf);

        // Define the popup block with styling
        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(" Help ")
            .title_style(Theme::fg(&theme.foreground))
            .title_bottom(help_command_hints)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Render the help text
        Paragraph::new(help_lines)
            .block(popup_block)
            .render(popup_area, buf);
    }
}

pub struct LeaderHelpPopUp;

impl LeaderHelpPopUp {
    /// Render popup for displaying help information
    pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Help text lines
        let help_lines = vec![
            Line::from(vec![
                Span::styled("1", Theme::fg(&theme.accent)),
                Span::raw(" → Go to "),
                Span::styled("List Selection", Theme::fg(&theme.highlight_fg)),
            ]),
            Line::from(vec![
                Span::styled("2", Theme::fg(&theme.accent)),
                Span::raw(" → Go to "),
                Span::styled("Item Selection", Theme::fg(&theme.highlight_fg)),
            ]),
            Line::from(vec![
                Span::styled("3", Theme::fg(&theme.accent)),
                Span::raw(" → Go to "),
                Span::styled("Database Selection", Theme::fg(&theme.highlight_fg)),
            ]),
        ];

        // Calculate popup dimensions
        let popup_width = 35; // 66% of the area width
        let popup_height = help_lines.len() as u16 + 4; // Enough for all help lines + padding

        // Center horizontally within the area
        let popup_x = area.x + area.width.saturating_sub(popup_width) - 3;

        // Center vertically within the area
        let popup_y = area.y + area.height.saturating_sub(popup_height) - 3;

        // Define the pop-up area
        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        // Clear the background of the popup area first
        Clear.render(popup_area, buf);
        Block::default()
            .style(Theme::bg(&theme.background))
            .render(popup_area, buf);

        let title_span = Span::styled(" ␣ ", Theme::fg(&theme.highlight_line_number_fg));

        let bottom_title = Line::from(vec![
            Span::styled(" [Esc]", Theme::fg(&theme.accent)),
            Span::from(" → Close "),
        ]);

        // Define the popup block with styling
        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(title_span)
            .title_bottom(bottom_title)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded);

        // Render the help text
        Paragraph::new(help_lines)
            .block(popup_block)
            .render(popup_area, buf);
    }
}

pub struct DeleteListConfirmationPopUp;

impl DeleteListConfirmationPopUp {
    pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme, list_name: &str) {
        // Confirmation message
        let confirmation_lines = vec![Line::from(vec![
            Span::raw("Are you sure you want to delete the list "),
            Span::styled(list_name, Theme::fg(&theme.accent)),
            Span::raw("?"),
        ])];

        // Command hints for confirmation popup
        let command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[Y] Yes   [N] No", Theme::fg(&theme.accent)),
            Span::raw(" "),
        ]);

        // Calculate popup dimensions
        let popup_width = (area.width * 2) / 3;
        let popup_height = confirmation_lines.len() as u16 + 4;

        let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        // Clear the background of the popup area first
        Clear.render(popup_area, buf);
        Block::default()
            .style(Theme::bg(&theme.background))
            .render(popup_area, buf);

        // Define the popup block with styling
        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(" Confirm Deletion ")
            .title_style(Theme::fg(&theme.foreground))
            .title_bottom(command_hints)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Render the confirmation text
        Paragraph::new(confirmation_lines)
            .wrap(Wrap { trim: false })
            .block(popup_block)
            .render(popup_area, buf);
    }
}

pub struct DeleteDatabaseConfirmationPopUp;

impl DeleteDatabaseConfirmationPopUp {
    pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme, db_name: &str) {
        let confirmation_lines = vec![Line::from(vec![
            Span::raw("Are you sure you want to delete the database "),
            Span::styled(db_name, Theme::fg(&theme.accent)),
            Span::raw("?"),
        ])];

        let command_hints = Line::from(vec![
            Span::raw(" "),
            Span::styled("[Y] Yes   [N] No", Theme::fg(&theme.accent)),
            Span::raw(" "),
        ]);

        let popup_width = (area.width * 2) / 3;
        let popup_height = confirmation_lines.len() as u16 + 4;
        let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;
        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        Clear.render(popup_area, buf);
        Block::default()
            .style(Theme::bg(&theme.background))
            .render(popup_area, buf);

        let popup_block = Block::new()
            .padding(Padding::new(2, 2, 1, 1))
            .title(" Confirm Database Deletion ")
            .title_style(Theme::fg(&theme.foreground))
            .title_bottom(command_hints)
            .borders(Borders::ALL)
            .border_style(Theme::fg(&theme.border))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        Paragraph::new(confirmation_lines)
            .wrap(Wrap { trim: false })
            .block(popup_block)
            .render(popup_area, buf);
    }
}
