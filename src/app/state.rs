use crate::app::events::EventHandler;
use crate::db::config::{Config, DBConfig};
use crate::db::connections::init_db;
use crate::ui::components::{
    AddItemPopup, AddListPopup, ItemsComponent, ListsComponent, NewItemState, NewListState,
};
use crate::ui::cursor::CursorState;
use crate::ui::layout::AppLayout;
use color_eyre::Result;
use crossterm::event::{self, KeyEvent};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use sqlx::SqlitePool;

/// Enum representing the different screens in the application
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    /// Main screen showing lists and items
    Main,
    /// Pop-up screen for adding a new list
    AddList,
    /// Pop-up screen for adding a new item
    AddItem,
}

/// Main application state
pub struct App {
    /// Configuration of available databases
    pub config: Config,
    /// Config of currently selected database
    pub current_db_config: DBConfig,
    /// Current active screen (Main, AddList, or AddItem)
    pub current_screen: CurrentScreen,
    /// Database connection pool
    pub pool: SqlitePool,
    /// Lists component for managing todo lists
    pub lists_component: ListsComponent,
    /// State of list being added
    pub new_list_state: NewListState,
    /// State of item being added
    pub new_item_state: NewItemState,
    /// Flag to indicate if the application should exit
    pub exit: bool,
}

impl App {
    /// Create new app instance
    ///
    /// Initializes the database connection, loads existing lists from the database,
    /// and sets up the initial UI state.
    pub async fn new() -> Self {
        // Read the config (creates default if missing)
        let config = Config::read().expect("Failed to read config file");

        // Extract the default db and its connection string
        let default_db_config = config
            .get_default()
            .expect("Couldn't fetch default database");
        let pool = init_db(&default_db_config.connection_str)
            .await
            .expect("Failed to connect to database");

        // Start from main screen
        let current_screen = CurrentScreen::Main;

        // Create lists component and load data
        let mut lists_component = ListsComponent::new();
        lists_component
            .load_lists(&pool)
            .await
            .expect("Failed to read lists");

        Self {
            config,
            current_db_config: default_db_config,
            current_screen,
            pool,
            lists_component,
            new_list_state: NewListState::new(),
            new_item_state: NewItemState::new(),
            exit: false,
        }
    }

    /// Run the application
    ///
    /// Main event loop that handles terminal drawing and user input.
    /// Continues until the user exits the application.
    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            // Draw the current state of the application
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            // Handle keyboard input based on current screen
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key_event(key).await;
            }
        }
        Ok(())
    }

    /// Handle key events and delegate to appropriate handler
    async fn handle_key_event(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => EventHandler::handle_main_screen_key(self, key).await,
            CurrentScreen::AddList => EventHandler::handle_add_list_screen_key(self, key).await,
            CurrentScreen::AddItem => EventHandler::handle_add_item_screen_key(self, key).await,
        }
    }

    /// Enter the "Add List" screen by opening the corresponding pop-up
    pub fn enter_add_list_screen(&mut self) {
        self.current_screen = CurrentScreen::AddList;
    }

    /// Enter the "Add Item" screen by opening the corresponding pop-up
    pub fn enter_add_item_screen(&mut self) {
        if self.lists_component.selected().is_some() {
            self.current_screen = CurrentScreen::AddItem;
        }
    }

    /// Exit the Add List screen without saving
    pub fn exit_add_list_without_saving(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.new_list_state.clear();
    }

    /// Exit the Add Item screen without saving
    pub fn exit_add_item_without_saving(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.new_item_state.clear();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render background
        AppLayout::render_background(area, buf);

        // Calculate layout areas
        let (header_area, lists_area, items_area) = AppLayout::calculate_main_layout(area);

        // Render the main areas
        AppLayout::render_header(header_area, buf);
        self.lists_component.render(lists_area, buf);

        // Render items with the selected list
        let selected_list = self.lists_component.get_selected_list_mut();
        ItemsComponent::render(selected_list, items_area, buf);

        // Render popup screens if active
        match self.current_screen {
            CurrentScreen::AddList => AddListPopup::render(&self.new_list_state, lists_area, buf),
            CurrentScreen::AddItem => AddItemPopup::render(&self.new_item_state, items_area, buf),
            _ => {}
        }
    }
}
