//! Judo - A terminal-based todo list application

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use judo::db::connections::init_db;
use judo::db::models::{NewTodoItem, NewTodoList, TodoItem, TodoList, UIItem, UIList};
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

/// Main application state
pub struct App {
    /// Current active screen (Main, AddList, or AddItem)
    current_screen: CurrentScreen,
    /// Database connection pool
    pool: SqlitePool,
    /// Vector of todo lists with their UI state
    lists: Vec<UIList>,
    /// State for the list selection widget
    list_state: ListState,
    /// Buffer for new list name input
    new_list_state: NewListState,
    /// Buffer for new item name input
    current_new_item_name: String,
    /// Flag to indicate if the application should exit
    exit: bool,
}

pub struct NewListState {
    /// Buffer for new list name input
    current_new_list_name: String,
    /// Position of cursor
    cursor_pos: usize,
}

/// Enum representing the different screens in the application
pub enum CurrentScreen {
    /// Main screen showing lists and items
    Main,
    /// Pop-up screen for adding a new list
    AddList,
    /// Pop-up screen for adding a new item
    AddItem,
}

impl App {
    /// Create new app instance
    ///
    /// Initializes the database connection, loads existing lists from the database,
    /// and sets up the initial UI state.
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
            new_list_state: NewListState {
                current_new_list_name: String::new(),
                cursor_pos: 0,
            },
            current_new_item_name: String::new(),
            exit: false,
        }
    }

    /// Run the application
    ///
    /// Main event loop that handles terminal drawing and user input.
    /// Continues until the user exits the application.
    async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            // Draw the current state of the application
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            // Handle keyboard input based on current screen
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
    ///
    /// Processes keyboard input when the user is on the main screen.
    /// Handles navigation, item operations, and screen transitions.
    async fn handle_key_in_main_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true, // Quit application
            KeyCode::Char('s') => self.select_next_list(), // Navigate down in lists
            KeyCode::Char('w') => self.select_previous_list(), // Navigate up in lists
            KeyCode::Char('A') => self.enter_add_list_screen(), // Add new list
            KeyCode::Char('a') => self.enter_add_item_screen(), // Add new item
            KeyCode::Char('D') => self.delete_list().await, // Delete selected list
            KeyCode::Char('d') => self.delete_item().await, // Delete selected item
            KeyCode::Enter => self.toggle_done().await, // Toggle item completion
            KeyCode::Down => self.select_next_item(), // Navigate down in items
            KeyCode::Up => self.select_previous_item(), // Navigate up in items
            KeyCode::Left => self.remove_item_selection(), // Deselect item
            KeyCode::Right => self.select_first_item(), // Select first item
            _ => {}
        }
    }

    /// Handle key press from user in add list screen
    ///
    /// Processes keyboard input when the user is adding a new list.
    /// Handles text input and submission/cancellation.
    async fn handle_key_in_add_list_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit_add_list_without_saving(), // Cancel without saving
            KeyCode::Backspace => self.remove_char_from_new_list_name(), // Delete character
            KeyCode::Char(value) => self.add_char_to_new_list_name(value), // Add character
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Enter => self.create_new_list().await, // Submit new list
            _ => {}
        }
    }

    /// Handle key press from user in add item screen
    ///
    /// Processes keyboard input when the user is adding a new item.
    /// Handles text input and submission/cancellation.
    async fn handle_key_in_add_item_screen(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit_add_item_without_saving(), // Cancel without saving
            KeyCode::Backspace => self.remove_char_from_new_item_name(), // Delete character
            KeyCode::Char(value) => self.add_char_to_new_item_name(value), // Add character
            KeyCode::Enter => self.create_new_item().await,      // Submit new item
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
    ///
    /// Only works if a list is currently selected.
    fn select_next_item(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select_next();
        }
    }

    /// Select previous element in the list of to-do items
    ///
    /// Only works if a list is currently selected.
    fn select_previous_item(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select_previous();
        }
    }

    /// Remove item selection (deselect current item)
    ///
    /// Clears the item selection while keeping the list selected.
    fn remove_item_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.lists[i].item_state.select(None);
        }
    }

    /// Select the first item in the currently selected list
    ///
    /// The right arrow is not really meant to be there but it's useful
    /// because of user's muscle memory. Only selects if no item is currently selected.
    fn select_first_item(&mut self) {
        if let Some(i) = self.list_state.selected()
            && let None = self.lists[i].item_state.selected()
        {
            self.lists[i].item_state.select_first();
        }
    }

    /// Toggle the "is_done" status of the currently selected item
    ///
    /// Updates the item status in the database and refreshes the UI.
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

    /// Enter the "Add List" screen by opening the corresponding pop-up
    fn enter_add_list_screen(&mut self) {
        self.current_screen = CurrentScreen::AddList;
    }

    /// Remove last character from new list name input buffer
    fn remove_char_from_new_list_name(&mut self) {
        // The cursor "points" is organized so that it points to the next char
        // If cursor == 0, then the user hasn't written a single char and we do nothing
        if self.new_list_state.cursor_pos > 0 {
            // Since it points to the next char, we remove the char in the previous position
            self.new_list_state
                .current_new_list_name
                .remove(self.new_list_state.cursor_pos - 1);
            // We need to update the position or else it will point e.g. to places in advances if
            // we are removing the last char in the string
            self.new_list_state.cursor_pos -= 1;
        }
    }

    /// Add character to new list name input buffer
    fn add_char_to_new_list_name(&mut self, value: char) {
        // Insert in a specific position
        self.new_list_state
            .current_new_list_name
            .insert(self.new_list_state.cursor_pos, value);
        // Move cursor forward
        self.new_list_state.cursor_pos += 1;
    }

    /// Exit the Add List screen without saving
    ///
    /// Returns to the main screen and clears the input buffer.
    fn exit_add_list_without_saving(&mut self) {
        // Go back to main screen
        self.current_screen = CurrentScreen::Main;

        // Erase any change in the new list name because the user exited without submitting
        self.new_list_state.current_new_list_name = String::new();
        self.new_list_state.cursor_pos = 0; // Reset cursor position
    }

    /// Move cursor one char to the left
    fn move_cursor_left(&mut self) {
        if self.new_list_state.cursor_pos > 0 {
            self.new_list_state.cursor_pos -= 1;
        }
    }

    /// Move cursor one char to the right
    fn move_cursor_right(&mut self) {
        if self.new_list_state.cursor_pos
            < self.new_list_state.current_new_list_name.chars().count()
        {
            self.new_list_state.cursor_pos += 1;
        }
    }

    /// Save new list to database
    ///
    /// Creates a new todo list with the entered name, saves it to the database,
    /// refreshes the list data, and returns to the main screen.
    async fn create_new_list(&mut self) {
        // Create a new todo list
        let new_list = NewTodoList {
            name: self.new_list_state.current_new_list_name.clone(),
        };

        // Write new list to db
        TodoList::create(&self.pool, new_list)
            .await
            .expect("Unable to add new list");

        // Go back to main
        self.current_screen = CurrentScreen::Main;

        // Re-init the new list variable
        self.new_list_state.current_new_list_name = String::new();

        // Re-set the list of lists
        self.lists = UIList::get_all(&self.pool)
            .await
            .expect("Failed to read lists")
    }

    /// Delete the currently selected list
    ///
    /// Removes the list from the database, refreshes the data, and adjusts
    /// the selection to maintain a valid state.
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

    /// Enter the "Add Item" screen by opening the corresponding pop-up
    fn enter_add_item_screen(&mut self) {
        if let Some(_i) = self.list_state.selected() {
            self.current_screen = CurrentScreen::AddItem;
        }
    }

    /// Remove last character from new item name input buffer
    fn remove_char_from_new_item_name(&mut self) {
        self.current_new_item_name.pop();
    }

    /// Add character to new item name input buffer
    fn add_char_to_new_item_name(&mut self, value: char) {
        self.current_new_item_name.push(value);
    }

    /// Exit the Add Item screen without saving
    ///
    /// Returns to the main screen and clears the input buffer.
    fn exit_add_item_without_saving(&mut self) {
        // Go back to main screen
        self.current_screen = CurrentScreen::Main;

        // Erase any change in the new item name because the user exited without submitting
        self.current_new_item_name = String::new();
    }

    /// Save new item to database
    ///
    /// Creates a new todo item in the currently selected list, saves it to the database,
    /// refreshes the data, and returns to the main screen.
    async fn create_new_item(&mut self) {
        // Get the list the item will belong to
        if let Some(i) = self.list_state.selected() {
            // Get the id of the parent list
            let list_id = self.lists[i].list.id;

            // Create a new todo item
            let new_item = NewTodoItem {
                name: self.current_new_item_name.clone(),
                list_id,
                priority: None,
                due_date: None,
            };

            // Write new item to db
            TodoItem::create(&self.pool, new_item)
                .await
                .expect("Unable to add new item");

            // Go back to main
            self.current_screen = CurrentScreen::Main;

            // Re-init the new item variable
            self.current_new_item_name = String::new();

            // Update list elements
            self.lists[i]
                .update_items(&self.pool)
                .await
                .expect("Failed to update list");
        }
    }

    /// Delete the currently selected item
    ///
    /// Removes the item from the database, refreshes the data, and adjusts
    /// the selection to maintain a valid state.
    async fn delete_item(&mut self) {
        if let Some(i) = self.list_state.selected()
            && let Some(j) = self.lists[i].item_state.selected()
        {
            let item = self.lists[i].items[j].item.clone();

            item.delete(&self.pool)
                .await
                .expect("Unable to delete item");

            // Update list elements
            self.lists[i]
                .update_items(&self.pool)
                .await
                .expect("Failed to update list");

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

/// Widget trait implementation for the main application
///
/// This implements the high-level rendering logic that coordinates all the different
/// UI components and handles responsive layout based on terminal size.
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render a background block that fills the entire area
        let background_color = Color::from_str("#002626").unwrap();
        let foreground_color = Color::from_str("#FCF1D5").unwrap();
        let background =
            Block::default().style(Style::default().bg(background_color).fg(foreground_color));
        background.render(area, buf);

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

        // Render the main areas
        App::render_header(header_area, buf);
        self.render_lists(lists_area, buf);
        self.render_items(items_area, buf);

        // Render popup screens if active
        match self.current_screen {
            CurrentScreen::AddList => self.render_add_list(lists_area, buf),
            CurrentScreen::AddItem => self.render_add_item(items_area, buf),
            _ => {}
        }
    }
}

impl App {
    /// Create text spans for rendering a string with a cursor at a specific position
    ///
    /// Returns a vector of spans: text before cursor, cursor character, text after cursor
    fn create_cursor_text_spans(text: &str, cursor_pos: usize) -> Vec<Span<'static>> {
        let chars: Vec<char> = text.chars().collect();
        let text_len = chars.len();

        // Ensure cursor position is within bounds
        let safe_cursor_pos = cursor_pos.min(text_len);

        // Text before cursor
        let text_before: String = chars[..safe_cursor_pos].iter().collect();

        // Character at cursor position (or space if at end)
        let cursor_char = if safe_cursor_pos >= text_len {
            "█".to_string()
        } else {
            chars[safe_cursor_pos].to_string()
        };

        // Text after cursor
        let text_after: String = if safe_cursor_pos >= text_len {
            String::new()
        } else {
            chars[(safe_cursor_pos + 1)..].iter().collect()
        };

        vec![
            Span::styled(
                text_before,
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            if cursor_char == "█" {
                Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(Color::from_str("#FCF1D5").unwrap())
                        .bg(Color::from_str("#002626").unwrap()),
                )
            } else {
                Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(Color::from_str("#002626").unwrap())
                        .bg(Color::from_str("#FCF1D5").unwrap()),
                )
            },
            Span::styled(
                text_after,
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]
    }

    /// Render the application header with ASCII logo
    ///
    /// Displays the "JUDO" ASCII art logo in the header area.
    fn render_header(area: Rect, buf: &mut Buffer) {
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

    /// Render the list of todo lists
    ///
    /// Displays all available todo lists with navigation hints and selection highlighting.
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
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
            Span::styled(
                " [D]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "el ",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ),
        ]);

        let block = Block::default()
            .padding(Padding::horizontal(2))
            .title_top(Line::raw(" Lists ").centered())
            .title_bottom(list_command_hints)
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_type(BorderType::Rounded);

        // Convert lists to display items
        let items: Vec<ListItem> = self
            .lists
            .iter()
            .map(|ui_list| ListItem::from(ui_list.list.name.clone()))
            .collect();

        let list: List = List::new(items)
            .block(block)
            .highlight_symbol(" ▸ ") // Selection indicator
            .highlight_style(
                // Swap foreground and background for selected item
                Style::default()
                    .bg(Color::from_str("#FCF1D5").unwrap())
                    .fg(Color::from_str("#002626").unwrap()),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list_state)
    }

    /// Apply styling to a todo item based on its completion status
    ///
    /// Returns a styled span that shows completed items with strikethrough.
    fn style_item(ui_item: &UIItem) -> Span<'_> {
        let name = ui_item.item.name.clone();

        if ui_item.item.is_done {
            // Strike through completed items
            Span::styled(name, Style::default().add_modifier(Modifier::CROSSED_OUT))
        } else {
            Span::from(name)
        }
    }

    /// Render the list of todo items for the selected list
    ///
    /// Displays items from the currently selected list with navigation hints and completion status.
    /// Shows a message if no list is selected.
    fn render_items(&mut self, area: Rect, buf: &mut Buffer) {
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

        // Get the list selected by the user
        if let Some(i) = self.list_state.selected() {
            let selected_list = &mut self.lists[i];

            // Extract the corresponding items with styling
            let items: Vec<ListItem> = selected_list
                .items
                .iter()
                .map(|ui_item| ListItem::from(App::style_item(ui_item)))
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

            StatefulWidget::render(list, area, buf, &mut selected_list.item_state);
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

    /// Render popup for entering a new list name
    ///
    /// Displays a centered popup dialog for adding a new todo list.
    /// Only the list name is required for creation.
    fn render_add_list(&mut self, area: Rect, buf: &mut Buffer) {
        // Command hints for add list popup
        let add_list_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
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

        // Define the popup block with styling
        let popup_block = Block::new()
            .title(" Add List ")
            .title_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()).bold())
            .title_bottom(add_list_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render with a blinking cursor
        let text_spans = Self::create_cursor_text_spans(
            &self.new_list_state.current_new_list_name,
            self.new_list_state.cursor_pos,
        );

        let text_line = Line::from(text_spans);

        // Render the input field
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
    }

    /// Render popup for entering a new item name
    ///
    /// Displays a centered popup dialog for adding a new todo item to the selected list.
    fn render_add_item(&mut self, area: Rect, buf: &mut Buffer) {
        // Command hints for add item popup
        let add_item_command_hints = Line::from(vec![
            Span::styled(
                "[E]",
                Style::default().fg(Color::from_str("#FFA69E").unwrap()),
            ),
            Span::styled(
                "sc",
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
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

        // Define the popup block with styling
        let popup_block = Block::new()
            .title(" Add Item ")
            .title_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()).bold())
            .title_bottom(add_item_command_hints)
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::from_str("#FCF1D5").unwrap()))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        // Define the text to render with a blinking cursor at the end
        // Note: Blinking cursor doesn't appear in some terminals like Warp
        let text_line = Line::from(vec![
            Span::styled(
                self.current_new_item_name.clone(),
                Style::default().fg(Color::from_str("#FCF1D5").unwrap()),
            ), // User input text
            Span::styled(
                "█",
                Style::default()
                    .fg(Color::from_str("#FCF1D5").unwrap())
                    .add_modifier(Modifier::RAPID_BLINK), // Blinking cursor
            ),
        ]);

        // Render the input field
        Paragraph::new(text_line)
            .wrap(Wrap { trim: true })
            .block(popup_block)
            .render(popup_area, buf);
    }
}

/// Application entry point
///
/// Initializes the terminal, creates the application instance, runs the main loop,
/// and properly restores the terminal on exit.
#[tokio::main]
async fn main() -> Result<()> {
    // Set the terminal up
    let mut terminal = ratatui::init();

    // Set up the app
    let app = App::new().await;

    // Create and run the app
    let app_result = app.run(&mut terminal).await;

    // Restore terminal to original state
    ratatui::restore();

    app_result
}
