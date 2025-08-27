use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph, Widget};
use std::str::FromStr;

pub struct AppLayout;

impl AppLayout {
    /// Render the application header with ASCII logo
    pub fn render_header(area: Rect, buf: &mut Buffer) {
        // Use the judo ascii logo
        let ascii_logo = r#"
     ██╗██╗   ██╗██████╗  ██████╗ 
     ██║██║   ██║██╔══██╗██╔═══██╗
     ██║██║   ██║██║  ██║██║   ██║
██   ██║██║   ██║██║  ██║██║   ██║
╚█████╔╝╚██████╔╝██████╔╝╚██████╔╝
 ╚════╝  ╚═════╝ ╚═════╝  ╚═════╝ 
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

    /// Calculate responsive layout areas
    pub fn calculate_main_layout(area: Rect) -> (Rect, Rect, Rect) {
        // Calculate responsive header height based on terminal size
        let header_height = if area.height < 15 {
            // Very small terminal - minimal header
            Constraint::Length(3)
        } else if area.height < 25 {
            // Small terminal - reduced header
            Constraint::Length(8)
        } else {
            // Normal terminal - full header
            Constraint::Percentage(20)
        };

        let main_layout = Layout::vertical([
            header_height,
            Constraint::Min(10), // Ensure minimum content area
        ]);

        // Extract the areas from the main layout
        let [header_area, content_area] = main_layout.areas(area);

        // Further subdivide the content area into list area and item area
        let content_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        // Extract the areas for lists and items
        let [lists_area, items_area] = content_layout.areas(content_area);

        (header_area, lists_area, items_area)
    }

    /// Render a background that fills the entire area
    pub fn render_background(area: Rect, buf: &mut Buffer) {
        let background_color = Color::from_str("#002626").unwrap();
        let foreground_color = Color::from_str("#FCF1D5").unwrap();
        let background =
            Block::default().style(Style::default().bg(background_color).fg(foreground_color));
        background.render(area, buf);
    }
}
