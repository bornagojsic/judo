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
