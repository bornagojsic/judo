use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

// #[derive(Debug, Default)]
pub struct App {
    status: i32,
    exit: bool,
}

// Default trait should "load" the initial state of the app
impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            status: 42,
        }
    }
}

impl App {
    /// Run the application
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    /// Handle key press from user
    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }
}

/// Widget trait implements the high-level rendering logic
/// Actual rendering functions to be implemented for App and not as part of a trait
/// It should always implement render
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Define the main layout of the app
        let main_layout = Layout::vertical([
            Constraint::Percentage(20), // Header
            Constraint::Percentage(70), // Main content
            Constraint::Percentage(10), // Footer
        ]);

        // Extract the areas from the main layout
        let [header_area, content_area, footer_area] = main_layout.areas(area);

        // Further subdivide the content area into list area and item area
        let content_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        // Extract the areas for lists and items
        let [lists_area, items_area] = content_layout.areas(content_area);

        // Render the four areas
        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_lists(lists_area, buf);
        self.render_items(items_area, buf);
    }
}

impl App {
    // Render the header
    fn render_header(area: Rect, buf: &mut Buffer) {
        // Use the td ascii logo
        let ascii_logo = r#"
████████╗  ██████═╗ 
╚══██╔══╝  ██╔═══██╗
   ██║     ██║   ██║
   ██║     ██║   ██║
   ██║     ██████╔═╝
   ╚═╝     ╚═════╝
        "#;

        // Define a block and pad. Can't pad on a paragraph, so we need to insert the paragraph inside
        // the padded block
        let block = Block::default().padding(Padding::horizontal(2));

        Paragraph::new(ascii_logo)
            .bold()
            .left_aligned()
            .block(block)
            .render(area, buf);
    }

    // Render footer with instructions
    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    // Render list of items
    fn render_lists(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("List area")
            .bold()
            .left_aligned()
            .render(area, buf);
    }

    // Render list of items
    fn render_items(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Item area")
            .bold()
            .left_aligned()
            .render(area, buf);
    }

    // fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
    //     let block = Block::new()
    //         .title(Line::raw("TODO List").centered())
    //         .borders(Borders::TOP)
    //         .border_set(symbols::border::EMPTY)
    //         .border_style(TODO_HEADER_STYLE)
    //         .bg(NORMAL_ROW_BG);

    //     // Iterate through all elements in the `items` and stylize them.
    //     let items: Vec<ListItem> = self
    //         .todo_list
    //         .items
    //         .iter()
    //         .enumerate()
    //         .map(|(i, todo_item)| {
    //             let color = alternate_colors(i);
    //             ListItem::from(todo_item).bg(color)
    //         })
    //         .collect();

    //     // Create a List from all list items and highlight the currently selected one
    //     let list = List::new(items)
    //         .block(block)
    //         .highlight_style(SELECTED_STYLE)
    //         .highlight_symbol(">")
    //         .highlight_spacing(HighlightSpacing::Always);

    //     // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
    //     // same method name `render`.
    //     StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    // }

    // fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
    //     // We get the info depending on the item's state.
    //     let info = if let Some(i) = self.todo_list.state.selected() {
    //         match self.todo_list.items[i].status {
    //             Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
    //             Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
    //         }
    //     } else {
    //         "Nothing selected...".to_string()
    //     };

    //     // We show the list item's info under the list in this paragraph
    //     let block = Block::new()
    //         .title(Line::raw("TODO Info").centered())
    //         .borders(Borders::TOP)
    //         .border_set(symbols::border::EMPTY)
    //         .border_style(TODO_HEADER_STYLE)
    //         .bg(NORMAL_ROW_BG)
    //         .padding(Padding::horizontal(1));

    //     // We can now render the item info
    //     Paragraph::new(info)
    //         .block(block)
    //         .fg(TEXT_FG_COLOR)
    //         .wrap(Wrap { trim: false })
    //         .render(area, buf);
    // }
}

fn main() -> Result<()> {
    // Set the terminal up
    let mut terminal = ratatui::init();

    // Crate and run the app
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();

    app_result
}
