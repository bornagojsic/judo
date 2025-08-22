use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::style::Style;
use ratatui::widgets::{
    Block, Padding, Paragraph,
    StatefulWidget, Widget, Borders, BorderType,
};
use ratatui::text::Line;
use ratatui::DefaultTerminal;
use ratatui::symbols;

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
            Constraint::Percentage(80), // Main content
        ]);

        // Extract the areas from the main layout
        let [header_area, content_area] = main_layout.areas(area);

        // Further subdivide the content area into list area and item area
        let content_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        // Extract the areas for lists and items
        let [lists_area, items_area] = content_layout.areas(content_area);

        // Render the four areas
        App::render_header(header_area, buf);
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

    // // Render footer with instructions
    // fn render_footer(area: Rect, buf: &mut Buffer) {
    //     Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
    //         .centered()
    //         .render(area, buf);
    // }

    // Render list of items
    fn render_lists(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Lists ").centered())
            .title_bottom(" ↓↑ ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new("List area")
            .left_aligned()
            .block(block)
            .render(area, buf);
    }

    // Render list of items
    fn render_items(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Items ").centered())
            .title_bottom(" ↓↑ ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new("Item area")
            .left_aligned()
            .block(block)
            .render(area, buf);
    }

}

fn main() -> Result<()> {
    // Set the terminal up
    let mut terminal = ratatui::init();

    // Crate and run the app
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();

    app_result
}
