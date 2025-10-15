use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::widgets::{Block, Widget};

use crate::ui::theme::Theme;

pub struct AppLayout;

impl AppLayout {
    /// Calculate responsive layout areas
    pub fn calculate_main_layout(area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        // Add overall padding around the entire TUI
        // Adjust these values to control how much space you want from terminal borders
        let padded_area = area.inner(Margin {
            horizontal: 2, // 2 columns of padding on left and right
            vertical: 1,   // 1 row of padding on top and bottom
        });

        // Calculate responsive header height based on terminal size
        let header_height = if padded_area.height < 15 {
            // Very small terminal - minimal header
            Constraint::Length(3)
        } else if padded_area.height < 25 {
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

        // Extract the areas from the main layout using the padded area
        let [header_area, content_area] = main_layout.areas(padded_area);

        // Divide header between pure logo and database selector
        let header_layout = Layout::horizontal([Constraint::Min(30), Constraint::Length(50)]);

        // Extract the areas from the header layout
        let [logo_area, db_selector_area] = header_layout.areas(header_area);

        // Split between closed and open selector
        let selector_layout = Layout::vertical([Constraint::Percentage(56), Constraint::Fill(1)]);

        // When the user changes DB it opens up as a dropdown
        let [_, closed_selector_area] = selector_layout.areas(db_selector_area);

        // Further subdivide the content area into list area and item area
        let content_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        // Extract the areas for lists and items
        let [lists_area, items_area] = content_layout.areas(content_area);

        (
            lists_area,
            items_area,
            logo_area,
            db_selector_area,
            closed_selector_area,
        )
    }

    /// Render a background that fills the entire area
    pub fn render_background(area: Rect, buf: &mut Buffer, theme: &Theme) {
        let background = Block::default().style(Theme::fg_bg(&theme.foreground, &theme.background));
        background.render(area, buf);
    }
}
