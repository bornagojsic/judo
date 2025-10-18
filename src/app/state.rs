use crate::app::events::EventHandler;
use crate::db::config::{Config, DBConfig};
use crate::db::connections::init_db;
use crate::db::models::{TodoList, UIList};
use crate::ui::components::{
    AddDBPopUp, AddItemPopUp, AddListPopUp, DatabaseComponent, HelpPopUp, InputState,
    ItemsComponent, LeaderHelpPopUp, ListsComponent, Logo, ModifyDBPopUp, ModifyItemPopUp,
    ModifyListPopUp,
};
use crate::ui::cursor::CursorState;
use crate::ui::layout::AppLayout;
use crate::ui::theme::Theme;
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
    /// scren for selecting a database
    DBSelection,
    /// Screen for selecting a list
    ListSelection,
    /// Screen for selecting an item
    ItemSelection,
    /// Pop-up screen for adding a new list
    AddList,
    /// Pop-up screen for modifying an existing list
    ModifyList,
    /// Pop-up screen for adding a new item
    AddItem,
    /// Pop-up screen for adding a new item
    ModifyItem,
    /// Pop-up for adding a new database
    AddDB,
    /// Pop-up screen for modifying an existing database
    ModifyDB,
    /// Screen for showing help
    Help,
    /// Screen for showing leader help
    LeaderHelp,
    /// Pop-up screen for deleting a list
    DeleteListConfirmation,
    /// Pop-up screen for deleting a database
    DeleteDatabaseConfirmation,
}

/// Main application state
pub struct App {
    /// Configuration of available databases
    pub config: Config,
    /// Config of currently selected database
    pub current_db_config: DBConfig,
    /// Current active screen (Main, AddList, ModifyList, or AddItem)
    pub current_screen: CurrentScreen,
    /// Database connection pool
    pub pool: SqlitePool,
    /// Database component for managing databases
    pub database_component: DatabaseComponent,
    /// Lists component for managing todo lists
    pub lists_component: ListsComponent,
    /// State of user-provided input
    pub input_state: InputState,
    /// Selected database index for DB selector
    pub selected_db_index: usize,
    /// Flag to indicate if the application should exit
    pub exit: bool,
    /// Theme configuration for the application
    pub theme: Theme,
    /// Pending delete list name
    pub pending_delete_list_name: Option<String>,
    /// Pending delete database name
    pub pending_delete_db_name: Option<String>,
    /// Flag to indicate if the application is awaiting user input
    pub leader_awaiting: bool,
    /// Modifier for number input like 3k or 15j
    pub number_modifier: u16,
    /// Buffer for keys pressed in the last 1000ms
    pub keys_buffer: Vec<(String, bool)>,
    /// Flag to indicate if the application is awaiting the second 'g' key press
    pub awaiting_second_g: bool,
}

impl App {
    /// Create new app instance
    ///
    /// Initializes the database connection, loads existing lists from the database,
    /// and sets up the initial UI state.
    pub async fn new() -> Self {
        // Read the config (creates default if missing)
        let config = Config::read().expect("Failed to read config file");
        let theme = config.theme.clone().unwrap_or(Theme::default());

        // Extract the default db and its connection string
        let default_db_config = config
            .get_default()
            .expect("Couldn't fetch default database");
        let pool = init_db(&default_db_config.connection_str)
            .await
            .expect("Failed to connect to database");

        // Start from main screen
        let current_screen = CurrentScreen::ListSelection;

        // Create lists component and load data
        let mut lists_component = ListsComponent::new();
        lists_component
            .load_lists(&pool)
            .await
            .expect("Failed to read lists");

        let selected_db_index = config
            .dbs
            .iter()
            .position(|db| db.name == default_db_config.name)
            .unwrap_or(0);

        Self {
            config,
            current_db_config: default_db_config,
            current_screen,
            pool,
            database_component: DatabaseComponent::new(),
            lists_component,
            input_state: InputState::new(),
            selected_db_index,
            exit: false,
            theme,
            pending_delete_list_name: None,
            pending_delete_db_name: None,
            leader_awaiting: false,
            number_modifier: 0,
            keys_buffer: Vec::new(),
            awaiting_second_g: false,
        }
    }

    /// Add a key to the buffer and clean up old keys
    pub fn add_key_to_buffer(&mut self, key: &String, visible: bool) {
        if self.keys_buffer.len() > 0 {
            if let Some((_, last_is_visible)) = self.keys_buffer.last() {
                if *last_is_visible || !["k", "j", "K", "J"].contains(&key.as_str()) {
                    self.keys_buffer = Vec::new();
                }
            }
        }

        self.keys_buffer.push((key.clone(), visible));
    }

    /// Reset the key buffer
    pub fn reset_key_buffer(&mut self) {
        self.keys_buffer = Vec::new();
    }

    /// Get keys pressed in the last 1000ms
    pub fn recent_keys(&self) -> Vec<String> {
        if self.keys_buffer.len() > 0 {
            if let Some((_, visible)) = self.keys_buffer.last() {
                if !visible {
                    return Vec::new();
                }
            }
        }
        self.keys_buffer
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn reset_number_modifier(&mut self) {
        self.number_modifier = 0;
    }

    pub fn add_number_modifier(&mut self, modifier: u16) {
        if self.number_modifier > self.lists_component.lists.len() as u16 {
            self.number_modifier = self.lists_component.lists.len() as u16;
        }
        self.number_modifier *= 10;
        self.number_modifier += modifier;
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

    /// Create a new database with the given name
    pub async fn create_new_database(
        &mut self,
        db_name: String,
        set_as_default: bool,
    ) -> Result<()> {
        // Use data directory to standardize storage
        let data_dir = dirs::data_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find data directory"))?
            .join("judo");

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to create data directory: {}", e))?;

        // Create path to new db file
        let db_file = format!("{}.db", db_name);
        let path = data_dir.join(db_file);

        // Create connection string (only SQLite is admissible)
        let connection_str = format!("sqlite:{}", path.display());

        // Create new database config
        let new_db_config = DBConfig {
            name: db_name.clone(),
            connection_str: connection_str.clone(),
        };

        // Initialize the new database (this creates the file and runs migrations)
        init_db(&connection_str)
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize new database: {}", e))?;

        // Add to config
        self.config.dbs.push(new_db_config);

        // Set as default if requested
        if set_as_default {
            self.config.default = db_name.clone();
        }

        // Write updated config to file
        let config_dir = dirs::config_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?
            .join("judo");
        let config_path = config_dir.join("judo.toml");

        self.config
            .write(&config_path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to save config: {}", e))?;

        // Update selected index to point to the new database
        self.selected_db_index = self.config.dbs.len() - 1;

        Ok(())
    }

    /// Handle key events and delegate to appropriate handler
    async fn handle_key_event(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::AddList | CurrentScreen::ModifyList => {
                EventHandler::handle_add_or_modify_list_screen_key(self, key).await
            }
            CurrentScreen::AddItem => {
                EventHandler::handle_add_or_modify_item_screen_key(self, key).await
            }
            CurrentScreen::ModifyItem => {
                EventHandler::handle_add_or_modify_item_screen_key(self, key).await
            }
            CurrentScreen::DBSelection => {
                EventHandler::handle_change_db_screen_key(self, key).await
            }
            CurrentScreen::AddDB => EventHandler::handle_add_db_screen_key(self, key).await,
            CurrentScreen::ModifyDB => EventHandler::handle_modify_db_screen_key(self, key).await,
            CurrentScreen::ListSelection => {
                EventHandler::handle_list_selection_screen_key(self, key).await
            }
            CurrentScreen::ItemSelection => {
                EventHandler::handle_item_selection_screen_key(self, key).await
            }
            CurrentScreen::Help => EventHandler::handle_help_screen_key(self, key).await,
            CurrentScreen::LeaderHelp => {
                EventHandler::handle_leader_help_screen_key(self, key).await
            }
            CurrentScreen::DeleteListConfirmation => {
                EventHandler::handle_delete_list_confirmation_key(self, key).await
            }
            CurrentScreen::DeleteDatabaseConfirmation => {
                EventHandler::handle_delete_database_confirmation_key(self, key).await
            }
        }
    }

    /// Enter the "Add List" screen by opening the corresponding pop-up
    pub fn enter_add_list_screen(&mut self) {
        self.input_state = InputState::default();
        self.current_screen = CurrentScreen::AddList;
    }

    /// Enter the "Modify List" screen by opening the corresponding pop-up
    pub fn enter_modify_list_screen(&mut self, selected_list: &TodoList) {
        self.input_state = InputState {
            current_input: selected_list.name.clone(),
            cursor_pos: 0,
            is_modifying: true,
        };
        self.current_screen = CurrentScreen::ModifyList;
    }

    /// Enter the "Add Item" screen by opening the corresponding pop-up
    pub fn enter_add_item_screen(&mut self) {
        if self.lists_component.selected().is_some() {
            self.input_state = InputState::default();
            self.current_screen = CurrentScreen::AddItem;
        }
    }

    /// Enter the "Modify Item" screen by opening the corresponding pop-up
    pub fn enter_modify_item_screen(&mut self, ui_list: &UIList) {
        if self.lists_component.selected().is_some()
            && let Some(j) = ui_list.item_state.selected()
        {
            let selected_item = ui_list.items[j].item.clone();

            self.input_state = InputState {
                current_input: selected_item.name.clone(),
                cursor_pos: 0,
                is_modifying: true,
            };
            self.current_screen = CurrentScreen::ModifyItem;
        }
    }

    /// Exit the Add List screen without saving
    pub fn exit_add_or_modify_list_without_saving(&mut self) {
        self.current_screen = CurrentScreen::ListSelection;
        self.input_state.clear();
    }

    /// Exit the Add Item screen without saving
    pub fn exit_add_item_without_saving(&mut self) {
        self.current_screen = CurrentScreen::ItemSelection;
        self.input_state.clear();
    }

    /// Enter the "Change DB" screen by opening the corresponding pop-up
    pub fn enter_change_db_screen(&mut self) {
        // Find the index of the current database in the config
        self.selected_db_index = self
            .config
            .dbs
            .iter()
            .position(|db| db.name == self.current_db_config.name)
            .unwrap_or(0);
        self.current_screen = CurrentScreen::DBSelection;
    }

    /// Exit the Change DB screen without saving
    pub fn exit_change_db_without_saving(&mut self) {
        self.current_screen = CurrentScreen::DBSelection;
    }

    /// Enter the "Add DB" screen by opening the corresponding pop-up
    pub fn enter_add_db_screen(&mut self) {
        self.current_screen = CurrentScreen::AddDB;
    }

    /// Exit the Add DB screen without saving
    pub fn exit_add_db_without_saving(&mut self) {
        self.current_screen = CurrentScreen::DBSelection;
        self.input_state.clear();
    }

    /// Move selection up in DB list
    pub fn select_previous_db(&mut self) {
        if self.config.dbs.is_empty() {
            return;
        }
        self.selected_db_index = if self.selected_db_index == 0 {
            self.config.dbs.len() - 1
        } else {
            self.selected_db_index - 1
        };
    }

    /// Move selection down in DB list
    pub fn select_next_db(&mut self) {
        if self.config.dbs.is_empty() {
            return;
        }
        self.selected_db_index = (self.selected_db_index + 1) % self.config.dbs.len();
    }

    /// Switch to the selected database
    pub async fn switch_to_selected_db(&mut self) -> Result<()> {
        if let Some(selected_db) = self.config.dbs.get(self.selected_db_index) {
            // Initialize connection to the new database
            let new_pool = init_db(&selected_db.connection_str)
                .await
                .map_err(|e| color_eyre::eyre::eyre!("Failed to connect to database: {}", e))?;

            // Update app state
            self.current_db_config = selected_db.clone();
            self.pool = new_pool;

            // Reload all lists from the new database
            self.lists_component = ListsComponent::new();
            self.lists_component
                .load_lists(&self.pool)
                .await
                .map_err(|e| color_eyre::eyre::eyre!("Failed to load lists: {}", e))?;

            // Select the first list if available
            if !self.lists_component.lists.is_empty() {
                self.lists_component.list_state.select(Some(0));
            } else {
                self.lists_component.list_state.select(None);
            }

            // Return to main screen
            self.current_screen = CurrentScreen::ListSelection;
        }
        Ok(())
    }

    /// Set the selected database as default
    pub async fn set_selected_db_as_default(&mut self) -> Result<()> {
        if let Some(selected_db) = self.config.dbs.get(self.selected_db_index) {
            // Update the default in config
            self.config.default = selected_db.name.clone();

            // Write updated config to file
            let config_dir = dirs::config_dir()
                .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?
                .join("judo");
            let config_path = config_dir.join("judo.toml");

            self.config
                .write(&config_path)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to save config: {}", e))?;
        }
        Ok(())
    }

    /// Delete the selected database
    pub async fn delete_selected_db(&mut self) -> Result<()> {
        if self.selected_db_index < self.config.dbs.len() {
            let removed_db = self.config.dbs.remove(self.selected_db_index);

            // If the removed DB was the default, clear or update the default
            if self.config.default == removed_db.name {
                self.config.default = if !self.config.dbs.is_empty() {
                    self.config.dbs[0].name.clone() // Set to first DB if any remain
                } else {
                    String::new() // Or use Option<String> if default can be None
                };
            }

            // Write updated config to file
            let config_dir = dirs::config_dir()
                .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?
                .join("judo");
            let config_path = config_dir.join("judo.toml");

            self.config
                .write(&config_path)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to save config: {}", e))?;
        }
        Ok(())
    }

    /// Modify the selected database's name and/or connection string
    pub async fn modify_selected_db(
        &mut self,
        new_name: Option<String>,
        new_connection_str: Option<String>,
    ) -> Result<()> {
        if let Some(selected_db) = self.config.dbs.get_mut(self.selected_db_index) {
            // Update name if provided
            if let Some(name) = new_name {
                // If this DB is the default, update default name as well
                if self.config.default == selected_db.name {
                    self.config.default = name.clone();
                }
                selected_db.name = name;
            }
            // Update connection string if provided
            if let Some(conn_str) = new_connection_str {
                selected_db.connection_str = conn_str;
            }

            // Write updated config to file
            let config_dir = dirs::config_dir()
                .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?
                .join("judo");
            let config_path = config_dir.join("judo.toml");

            self.config
                .write(&config_path)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to save config: {}", e))?;
        }
        Ok(())
    }

    /// Enter the "Modify DB" screen by opening the corresponding pop-up
    pub fn enter_modify_db_screen(&mut self) {
        if let Some(selected_db) = self.config.dbs.get(self.selected_db_index) {
            self.input_state = InputState {
                current_input: selected_db.name.clone(),
                cursor_pos: 0,
                is_modifying: true,
            };
            self.current_screen = CurrentScreen::ModifyDB;
        }
    }

    /// Exit the Modify DB screen without saving
    pub fn exit_modify_db_without_saving(&mut self) {
        self.current_screen = CurrentScreen::DBSelection;
        self.input_state.clear();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render background
        AppLayout::render_background(area, buf, &self.theme);

        // Calculate layout areas
        let (lists_area, items_area, logo_area, db_selector_area) =
            AppLayout::calculate_main_layout(area);

        // Render logo
        Logo::render(logo_area, buf);

        // Render db selector
        self.database_component.render(
            &self.config,
            db_selector_area,
            buf,
            &self.theme,
            self.current_screen == CurrentScreen::DBSelection,
            Some(self.selected_db_index),
        );

        // Render the main areas
        self.lists_component.render(
            lists_area,
            buf,
            &self.theme,
            self.current_screen == CurrentScreen::ListSelection,
        );

        // Render items with the selected list
        let recent_keys_str = self.recent_keys().concat();
        let selected_list = self.lists_component.get_selected_list_mut();

        ItemsComponent::render(
            selected_list,
            items_area,
            buf,
            &self.theme,
            self.current_screen == CurrentScreen::ItemSelection,
            recent_keys_str,
        );

        // Render popup screens if active
        match self.current_screen {
            CurrentScreen::AddList => {
                AddListPopUp::render(&self.input_state, lists_area, buf, &self.theme)
            }
            CurrentScreen::ModifyList => {
                ModifyListPopUp::render(&self.input_state, lists_area, buf, &self.theme)
            }
            CurrentScreen::AddItem => {
                AddItemPopUp::render(&self.input_state, items_area, buf, &self.theme)
            }
            CurrentScreen::ModifyItem => {
                ModifyItemPopUp::render(&self.input_state, items_area, buf, &self.theme)
            }
            CurrentScreen::AddDB => {
                AddDBPopUp::render(&self.input_state, db_selector_area, buf, &self.theme)
            }
            CurrentScreen::ModifyDB => {
                ModifyDBPopUp::render(&self.input_state, db_selector_area, buf, &self.theme)
            }
            CurrentScreen::Help => {
                HelpPopUp::render(area, buf, &self.theme);
            }
            CurrentScreen::LeaderHelp => {
                LeaderHelpPopUp::render(area, buf, &self.theme);
            }
            CurrentScreen::DeleteListConfirmation => {
                use crate::ui::components::popups::DeleteListConfirmationPopUp;
                if let Some(ref list_name) = self.pending_delete_list_name {
                    DeleteListConfirmationPopUp::render(lists_area, buf, &self.theme, list_name);
                }
            }
            CurrentScreen::DeleteDatabaseConfirmation => {
                use crate::ui::components::popups::DeleteDatabaseConfirmationPopUp;
                if let Some(ref db_name) = self.pending_delete_db_name {
                    DeleteDatabaseConfirmationPopUp::render(
                        db_selector_area,
                        buf,
                        &self.theme,
                        db_name,
                    );
                }
            }
            _ => {}
        }
    }
}
