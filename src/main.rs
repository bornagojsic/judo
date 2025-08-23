use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget,
};
use sqlx::sqlite::SqlitePool;
use std::str::FromStr;
use td::db::connections::init_db;
use td::db::models::{UIItem, UIList};

//#[derive(Debug, Default)]
pub struct App {
    pool: SqlitePool,
    lists: Vec<UIList>,
    list_state: ListState,
    exit: bool,
}

impl App {
    /// Create new app instance
    async fn new() -> Self {
        // Init connection to db
        let pool = init_db().await.expect("Failed to connect to database");

        // Read the lists from db
        let lists = UIList::get_all(&pool).await.expect("Failed to read lists");

        // Init state of lists
        let list_state = ListState::default();

        Self {
            pool,
            lists,
            list_state,
            exit: false,
        }
    }

    /// Run the application
    async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key).await;
            }
        }
        Ok(())
    }

    /// Handle key press from user
    async fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('s') => self.select_next_list(),
            KeyCode::Char('w') => self.select_previous_list(),
            KeyCode::Char('d') => self.toggle_done().await,
            KeyCode::Down => self.select_next_item(),
            KeyCode::Up => self.select_previous_item(),
            KeyCode::Left => self.remove_item_selection(),
            _ => {}
        }
    }

    /// Select next element in the list of to-do lists
    fn select_next_list(&mut self) {
        self.list_state.select_next();
    }

    /// Select previous element in the list of to-do lists
    fn select_previous_list(&mut self) {
        self.list_state.select_previous();
    }

    /// Select next element in the list of to-do items
    fn select_next_item(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select_next();
        }
    }

    /// Select previous element in the list of to-do items
    fn select_previous_item(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select_previous();
        }
    }

    /// Select previous element in the list of to-do items
    fn remove_item_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select(None);
        }
    }

    /// Toggle "is_done"
    async fn toggle_done(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(j) = self.lists[i].item_state.selected() {
                self.lists[i].items[j]
                    .item
                    .toggle_done(&self.pool)
                    .await
                    .expect("Unable to toggle status");
            }
        }
    }
}

/// Widget trait implements the high-level rendering logic
/// Actual rendering functions to be implemented for App and not as part of a trait
/// It should always implement render
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render a background block that fills the entire area
        let background_color = Color::from_str("#002626").unwrap();
        let foreground_color = Color::from_str("#F0EAD8").unwrap();
        let background =
            Block::default().style(Style::default().bg(background_color).fg(foreground_color));
        background.render(area, buf);

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

    // Render list of to-do lists
    fn render_lists(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Lists ").centered())
            .title_bottom(" w,s ")
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_type(BorderType::Rounded);

        let items: Vec<ListItem> = self
            .lists
            .iter()
            .map(|ui_list| ListItem::from(ui_list.list.name.clone()))
            .collect();
        let list: List = List::new(items)
            .block(block)
            .highlight_symbol(" ▸ ") // Rounded bottom-left corner character
            .highlight_style(
                // Swap foreground and background
                Style::default()
                    .bg(Color::from_str("#F0EAD8").unwrap())
                    .fg(Color::from_str("#002626").unwrap()),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list_state)
    }

    /// Style the item
    fn style_item(ui_item: &UIItem) -> Span<'_> {
        let name = ui_item.item.name.clone();

        if ui_item.item.is_done {
            Span::styled(name, Style::default().add_modifier(Modifier::CROSSED_OUT)).into()
        } else {
            Span::from(name).into()
        }
    }

    /// Render list of items
    fn render_items(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Items ").centered())
            .title_bottom(" ↓↑ ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        // Get the list selected by the user
        if let Some(i) = self.list_state.selected() {
            let selected_list = &mut self.lists[i];

            // Extract the corresponding items
            let items: Vec<ListItem> = selected_list
                .items
                .iter()
                .map(|ui_item| ListItem::from(App::style_item(ui_item)))
                //.map(|ui_item| ListItem::from(ui_item.item.name.clone()))
                .collect();

            let list: List = List::new(items)
                .block(block)
                .highlight_symbol(" ▸ ")
                .highlight_style(
                    // Swap foreground and background
                    Style::default()
                        .bg(Color::from_str("#F0EAD8").unwrap())
                        .fg(Color::from_str("#002626").unwrap()),
                )
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, &mut selected_list.item_state);
        } else {
            Paragraph::new("Select a to-do list first")
                .left_aligned()
                .block(block)
                .render(area, buf);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set the terminal up
    let mut terminal = ratatui::init();

    // Set up the app
    let app = App::new().await;

    // Crate and run the app
    let app_result = app.run(&mut terminal).await;

    ratatui::restore();

    app_result
}
