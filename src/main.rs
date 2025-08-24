use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use sqlx::sqlite::SqlitePool;
use std::str::FromStr;
use td::db::connections::init_db;
use td::db::models::{NewTodoItem, NewTodoList, TodoItem, TodoList, UIItem, UIList};

pub struct App {
    current_screen: CurrentScreen,
    pool: SqlitePool,
    lists: Vec<UIList>,
    list_state: ListState,
    current_new_list_name: String,
    current_new_item_name: String,
    exit: bool,
}

pub enum CurrentScreen {
    Main,
    AddList,
    AddItem,
}

impl App {
    /// Create new app instance
    async fn new() -> Self {
        // Start from main screen
        let current_screen = CurrentScreen::Main;

        // Init connection to db
        let pool = init_db().await.expect("Failed to connect to database");

        // Read the lists from db
        let lists = UIList::get_all(&pool).await.expect("Failed to read lists");

        // Init state of lists
        let list_state = ListState::default();

        Self {
            current_screen,
            pool,
            lists,
            list_state,
            current_new_list_name: String::new(),
            current_new_item_name: String::new(),
            exit: false,
        }
    }

    /// Run the application
    async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match self.current_screen {
                    CurrentScreen::Main => self.handle_key_in_main_screen(key).await,
                    CurrentScreen::AddList => self.handle_key_in_add_list_screen(key).await,
                    CurrentScreen::AddItem => self.handle_key_in_add_item_screen(key).await,
                }
            }
        }
        Ok(())
    }

    /// Handle key press from user in main screen
    async fn handle_key_in_main_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('s') => self.select_next_list(),
            KeyCode::Char('w') => self.select_previous_list(),
            KeyCode::Char('A') => self.enter_add_list_screen(),
            KeyCode::Char('a') => self.enter_add_item_screen(),
            KeyCode::Char('D') => self.delete_list().await,
            KeyCode::Char('d') => self.delete_item().await,
            KeyCode::Enter => self.toggle_done().await,
            KeyCode::Down => self.select_next_item(),
            KeyCode::Up => self.select_previous_item(),
            KeyCode::Left => self.remove_item_selection(),
            KeyCode::Right => self.select_first_item(),
            _ => {}
        }
    }

    /// Handle key press from user in add list
    async fn handle_key_in_add_list_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit_add_list_without_saving(),
            KeyCode::Backspace => self.remove_char_from_new_list_name(),
            KeyCode::Char(value) => self.add_char_to_new_list_name(value),
            KeyCode::Enter => self.create_new_list().await,
            _ => {}
        }
    }

    /// Handle key press from user in add item
    async fn handle_key_in_add_item_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit_add_item_without_saving(),
            KeyCode::Backspace => self.remove_char_from_new_item_name(),
            KeyCode::Char(value) => self.add_char_to_new_item_name(value),
            KeyCode::Enter => self.create_new_item().await,
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

    /// The right arrow is not really meant to be there
    /// but it's useful because of user's muscle memory
    fn select_first_item(&mut self) {
        if let Some(i) = self.list_state.selected()
            && let None = self.lists[i].item_state.selected()
        {
            self.lists[i].item_state.select_first();
        }
    }

    /// Toggle "is_done"
    async fn toggle_done(&mut self) {
        if let Some(i) = self.list_state.selected()
            && let Some(j) = self.lists[i].item_state.selected()
        {
            self.lists[i].items[j]
                .item
                .toggle_done(&self.pool)
                .await
                .expect("Unable to toggle status");
        }
    }

    /// Enter the "Add List" by opening the corresponding pop-up
    fn enter_add_list_screen(&mut self) {
        self.current_screen = CurrentScreen::AddList;
    }

    /// Remove last char from new list name
    fn remove_char_from_new_list_name(&mut self) {
        self.current_new_list_name.pop();
    }

    /// Add char to new list name
    fn add_char_to_new_list_name(&mut self, value: char) {
        self.current_new_list_name.push(value);
    }

    /// Exit the Add List screen without saving
    fn exit_add_list_without_saving(&mut self) {
        // Go back to main screen
        self.current_screen = CurrentScreen::Main;

        // Erase any change in the new list name because the user exited without submitting
        self.current_new_list_name = String::new();
    }

    /// Save new list to database
    async fn create_new_list(&mut self) {
        // Create a new todo list
        let new_list = NewTodoList {
            name: self.current_new_list_name.clone(),
        };

        // Write new list to db
        TodoList::create(&self.pool, new_list)
            .await
            .expect("Unable to add new list");

        // Go back to main
        self.current_screen = CurrentScreen::Main;

        // Re-init the new list variable
        self.current_new_list_name = String::new();

        // Re-set the list of lists
        self.lists = UIList::get_all(&self.pool)
            .await
            .expect("Failed to read lists")
    }

    /// Delete list
    async fn delete_list(&mut self) {
        if let Some(i) = self.list_state.selected() {
            let list = self.lists[i].list.clone();
            list.delete(&self.pool)
                .await
                .expect("Unable to delete list");

            // Refresh the lists from database
            self.lists = UIList::get_all(&self.pool)
                .await
                .expect("Failed to read lists");

            // Adjust selection after deletion
            if self.lists.is_empty() {
                self.list_state.select(None);
            } else if i >= self.lists.len() {
                self.list_state.select(Some(self.lists.len() - 1));
            }
        }
    }

    /// Enter the "Add Item" by opening the corresponding pop-up
    fn enter_add_item_screen(&mut self) {
        self.current_screen = CurrentScreen::AddItem;
    }

    /// Remove last char from new list name
    fn remove_char_from_new_item_name(&mut self) {
        self.current_new_item_name.pop();
    }

    /// Add char to new list name
    fn add_char_to_new_item_name(&mut self, value: char) {
        self.current_new_item_name.push(value);
    }

    /// Exit the Add List screen without saving
    fn exit_add_item_without_saving(&mut self) {
        // Go back to main screen
        self.current_screen = CurrentScreen::Main;

        // Erase any change in the new list name because the user exited without submitting
        self.current_new_item_name = String::new();
    }

    /// Save new list to database
    async fn create_new_item(&mut self) {
        // Get the list the item will belong to
        if let Some(i) = self.list_state.selected() {
            // Get the id of the parent list
            let list_id = self.lists[i].list.id;

            // Create a new todo list
            let new_item = NewTodoItem {
                name: self.current_new_item_name.clone(),
                list_id,
                priority: None,
                due_date: None,
            };

            // Write new list to db
            TodoItem::create(&self.pool, new_item)
                .await
                .expect("Unable to add new item");

            // Go back to main
            self.current_screen = CurrentScreen::Main;

            // Re-init the new list variable
            self.current_new_item_name = String::new();

            // Re-set the list of lists
            self.lists = UIList::get_all(&self.pool)
                .await
                .expect("Failed to read lists")
        }
    }

    /// Delete item
    async fn delete_item(&mut self) {
        if let Some(i) = self.list_state.selected()
            && let Some(j) = self.lists[i].item_state.selected()
        {
            let item = self.lists[i].items[j].item.clone();

            item.delete(&self.pool)
                .await
                .expect("Unable to delete item");

            // Refresh the lists from database
            self.lists = UIList::get_all(&self.pool)
                .await
                .expect("Failed to read lists");

            // Adjust selection after deletion - check bounds first
            if let Some(list) = self.lists.get_mut(i) {
                if list.items.is_empty() {
                    list.item_state.select(None);
                } else if j >= list.items.len() {
                    list.item_state.select(Some(list.items.len() - 1));
                }
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

        // Render the main areas
        App::render_header(header_area, buf);
        self.render_lists(lists_area, buf);
        self.render_items(items_area, buf);

        match self.current_screen {
            CurrentScreen::AddList => self.render_add_list(lists_area, buf),
            CurrentScreen::AddItem => self.render_add_item(items_area, buf),
            _ => {}
        }
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
        // Command hints for lists
        let list_command_hints = Line::from(vec![
            Span::styled(" w,s ", Style::default()),
            Span::styled(
                "[A]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "dd",
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ),
            Span::styled(
                " [D]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "el ",
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ),
        ]);

        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Lists ").centered())
            .title_bottom(list_command_hints)
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
            Span::styled(name, Style::default().add_modifier(Modifier::CROSSED_OUT))
        } else {
            Span::from(name)
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

    /// Render pop-up for entering a new list (only name is required)
    fn render_add_list(&mut self, area: Rect, buf: &mut Buffer) {
        // Command hints for lists
        let add_list_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ),
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
        Block::default()
            .style(Style::default().bg(Color::from_str("#002626").unwrap()))
            .render(popup_area, buf);

        // Define the block
        let popup_block = Block::new()
            .title(" Add List ")
            .title_style(Style::new().fg(Color::from_str("#F0EAD8").unwrap()).bold())
            .title_bottom(add_list_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#F0EAD8").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render with a blinking character at the end
        // Doesn't appear in Warp
        let text_line = Line::from(vec![
            Span::styled(
                self.current_new_list_name.clone(),
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ), // Text
            Span::styled(
                "█",
                Style::default()
                    .fg(Color::from_str("#F0EAD8").unwrap())
                    .add_modifier(Modifier::RAPID_BLINK), // Blinking cursor
            ),
        ]);

        // Render with center alignment
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
    }

    /// Render the pop up for adding an item
    fn render_add_item(&mut self, area: Rect, buf: &mut Buffer) {
        // Command hints for items
        let add_item_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ),
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
        Block::default()
            .style(Style::default().bg(Color::from_str("#002626").unwrap()))
            .render(popup_area, buf);

        // Define the block
        let popup_block = Block::new()
            .title(" Add Item ")
            .title_style(Style::new().fg(Color::from_str("#F0EAD8").unwrap()).bold())
            .title_bottom(add_item_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#F0EAD8").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render with a blinking character at the end
        // Doesn't appear in Warp
        let text_line = Line::from(vec![
            Span::styled(
                self.current_new_item_name.clone(),
                Style::default().fg(Color::from_str("#F0EAD8").unwrap()),
            ), // Text
            Span::styled(
                "█",
                Style::default()
                    .fg(Color::from_str("#F0EAD8").unwrap())
                    .add_modifier(Modifier::RAPID_BLINK), // Blinking cursor
            ),
        ]);

        // Render with center alignment
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
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
