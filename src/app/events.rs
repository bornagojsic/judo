use crate::app::state::{App, CurrentScreen};
use crate::ui::components::{ItemsComponent, ListsComponent};
use crate::ui::cursor::CursorState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct EventHandler;

impl EventHandler {
    pub fn format_keycode_for_buffer(key: KeyEvent) -> (String, bool) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('h') {
            return ("Ctrl + h".to_string(), true);
        }

        let (key_str, visible) = match key.code {
            KeyCode::Char(' ') => ("␣".to_string(), true),
            KeyCode::Char(c) => {
                if c.is_ascii_digit() {
                    (c.to_string(), false)
                } else {
                    (c.to_string(), true)
                }
            }
            KeyCode::Up => ("↑".to_string(), true),
            KeyCode::Down => ("↓".to_string(), true),
            KeyCode::Left => ("←".to_string(), true),
            KeyCode::Right => ("→".to_string(), true),
            KeyCode::Tab => ("Tab".to_string(), true),
            KeyCode::BackTab => ("Shift + Tab".to_string(), true),
            KeyCode::Enter => ("Enter".to_string(), true),
            KeyCode::Esc => ("Esc".to_string(), true),
            _ => (String::new(), false), // or whatever default you want
        };

        (key_str, visible)
    }

    /// Handle key events that are screen-agnostic
    pub fn matches_global_keys(app: &mut App, key: KeyEvent) -> bool {
        let main_screens = vec![
            CurrentScreen::ListSelection,
            CurrentScreen::ItemSelection,
            CurrentScreen::DBSelection,
        ];

        if main_screens.contains(&app.current_screen) {
            // Handle global key events for main screens
            if key.code == KeyCode::Tab {
                app.current_screen = match &app.current_screen {
                    CurrentScreen::ListSelection => CurrentScreen::ItemSelection,
                    CurrentScreen::ItemSelection => CurrentScreen::DBSelection,
                    CurrentScreen::DBSelection => CurrentScreen::ListSelection,
                    _ => CurrentScreen::ListSelection,
                };
            }
            if key.code == KeyCode::BackTab {
                app.current_screen = match &app.current_screen {
                    CurrentScreen::ListSelection => CurrentScreen::DBSelection,
                    CurrentScreen::ItemSelection => CurrentScreen::ListSelection,
                    CurrentScreen::DBSelection => CurrentScreen::ItemSelection,
                    _ => CurrentScreen::ListSelection,
                };
            }
            app.last_active_screen = app.current_screen.clone();
        }

        if app.awaiting_second_g {
            app.awaiting_second_g = false;
            if key.code == KeyCode::Char('g') {
                app.keys_buffer.push(("g".to_string(), false));
                return false;
            }
        }

        let (keycode_string, isnt_digit) = EventHandler::format_keycode_for_buffer(key);

        app.add_key_to_buffer(&keycode_string, !isnt_digit);

        if key.code == KeyCode::Char('g') && app.current_screen == CurrentScreen::ItemSelection {
            app.awaiting_second_g = true;
            return false;
        }

        match key.code {
            KeyCode::Char('q') => app.exit = true, // Quit application
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.current_screen = CurrentScreen::Help;
            }
            KeyCode::Esc => {
                if !main_screens.contains(&app.current_screen) {
                    return false;
                }
                app.reset_number_modifier();
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let num = if c == '0' {
                    0
                } else {
                    c.to_digit(10).unwrap() as usize
                };
                app.add_number_modifier(num);
            }
            KeyCode::Char(' ') => {
                if app.number_modifier == 0 {
                    // TODO: Remove after implementing leader help screen
                    app.leader_awaiting = true;
                    app.current_screen = CurrentScreen::LeaderHelp;
                }
            }
            _ => return false,
        }
        true
    }

    pub async fn handle_leader_help_screen_key(app: &mut App, key: KeyEvent) {
        app.leader_awaiting = false;
        match key.code {
            KeyCode::Char('q') => app.exit = true, // Quit application
            KeyCode::Char('1') => app.current_screen = CurrentScreen::ListSelection,
            KeyCode::Char('2') => app.current_screen = CurrentScreen::ItemSelection,
            KeyCode::Char('3') => app.current_screen = CurrentScreen::DBSelection,
            KeyCode::Esc => {
                app.go_back();
                app.reset_key_buffer();
            }
            _ => app.leader_awaiting = true,
        }
    }

    pub async fn handle_help_screen_key(app: &mut App, key: KeyEvent) {
        if EventHandler::matches_global_keys(app, key) {
            return;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.go_back();
            }
            _ => {}
        }
    }

    /// Handle key press from user in list selection screen
    pub async fn handle_list_selection_screen_key(app: &mut App, key: KeyEvent) {
        if EventHandler::matches_global_keys(app, key) {
            return;
        }

        let modifier_shift = key.modifiers.contains(KeyModifiers::SHIFT);

        if modifier_shift {
            match key.code {
                KeyCode::Up | KeyCode::Char('K') => {
                    // Ctrl+Up: Move selected item up
                    if let Err(e) =
                        ListsComponent::move_selected_list_up(&mut app.lists_component, &app.pool)
                            .await
                    {
                        eprintln!("Failed to move list up: {}", e);
                    }
                }
                KeyCode::Down | KeyCode::Char('J') => {
                    // Ctrl+Down: Move selected item down
                    if let Err(e) =
                        ListsComponent::move_selected_list_down(&mut app.lists_component, &app.pool)
                            .await
                    {
                        eprintln!("Failed to move list down: {}", e);
                    }
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => app.lists_component.select_next(),
            KeyCode::Up | KeyCode::Char('k') => app.lists_component.select_previous(),
            KeyCode::Right | KeyCode::Char('l') => {
                app.current_screen = CurrentScreen::ItemSelection;
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::select_first_item(selected_list);
                }
            }
            KeyCode::Char('a') => app.enter_add_list_screen(), // Add new list
            KeyCode::Char('m') => {
                if let Some(selected_list) = app.lists_component.get_selected_list() {
                    app.enter_modify_list_screen(&selected_list.list.clone())
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected_list) = app.lists_component.get_selected_list() {
                    app.pending_delete_list_name = Some(selected_list.list.name.clone());
                    app.current_screen = CurrentScreen::DeleteListConfirmation;
                }
            }
            _ => {}
        }
    }

    pub async fn handle_delete_list_confirmation_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // Actually delete the list
                if let Err(e) =
                    ListsComponent::delete_selected_list_static(&mut app.lists_component, &app.pool)
                        .await
                {
                    eprintln!("Failed to delete list: {}", e);
                }
                app.pending_delete_list_name = None;
                app.go_back();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // Cancel deletion
                app.pending_delete_list_name = None;
                app.go_back();
            }
            _ => {}
        }
    }

    /// Handle key press from user in item selection screen
    pub async fn handle_item_selection_screen_key(app: &mut App, key: KeyEvent) {
        if EventHandler::matches_global_keys(app, key) {
            return;
        }

        let modifier_shift = key.modifiers.contains(KeyModifiers::SHIFT);

        if modifier_shift {
            let amount = if app.number_modifier == 0 {
                1
            } else {
                app.number_modifier as usize
            };

            match key.code {
                KeyCode::Up | KeyCode::Char('K') => {
                    // Ctrl+Up: Move selected item up
                    if let Some(selected_list) = app.lists_component.get_selected_list_mut()
                        && let Err(e) = ItemsComponent::move_selected_item_up_by(
                            selected_list,
                            &app.pool,
                            amount,
                        )
                        .await
                    {
                        eprintln!("Failed to move item up: {}", e);
                    }
                    app.reset_number_modifier();
                }
                KeyCode::Down | KeyCode::Char('J') => {
                    // Ctrl+Down: Move selected item down
                    if let Some(selected_list) = app.lists_component.get_selected_list_mut()
                        && let Err(e) = ItemsComponent::move_selected_item_down_by(
                            selected_list,
                            &app.pool,
                            amount,
                        )
                        .await
                    {
                        eprintln!("Failed to move item down: {}", e);
                    }
                    app.reset_number_modifier();
                }
                KeyCode::Char('G') => {
                    if let Some(selected_list) = app.lists_component.get_selected_list_mut()
                        && let Some(index) = selected_list.item_state.selected()
                    {
                        ItemsComponent::scroll_down_by(
                            selected_list,
                            selected_list.items.len() - index,
                        );
                    }
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('g') => {
                if app.awaiting_second_g {
                    return;
                }
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::select_first(selected_list);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    match app.number_modifier {
                        0 => ItemsComponent::select_next_item(selected_list),
                        _ => {
                            ItemsComponent::scroll_down_by(selected_list, app.number_modifier);
                            app.reset_number_modifier();
                        }
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    match app.number_modifier {
                        0 => ItemsComponent::select_previous_item(selected_list),
                        _ => {
                            ItemsComponent::scroll_up_by(selected_list, app.number_modifier);
                            app.reset_number_modifier();
                        }
                    }
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.current_screen = CurrentScreen::ListSelection;
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::remove_item_selection(selected_list);
                }
            }
            KeyCode::Char('a') => app.enter_add_item_screen(),
            KeyCode::Char('m') => {
                if let Some(selected_list) = app.lists_component.get_selected_list() {
                    app.enter_modify_item_screen(&selected_list.clone())
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut()
                    && let Err(e) =
                        ItemsComponent::delete_selected_item(selected_list, &app.pool).await
                {
                    eprintln!("Failed to delete item: {}", e);
                }
            }
            KeyCode::Enter => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut()
                    && let Err(e) = ItemsComponent::toggle_item_done(selected_list, &app.pool).await
                {
                    eprintln!("Failed to toggle item: {}", e);
                }
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::ItemSelection;
            }
            _ => {}
        }
    }

    /// Handle key press from user in add list screen
    pub async fn handle_add_or_modify_list_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => app.exit_add_or_modify_list_without_saving(),
            KeyCode::Backspace => app.input_state.remove_char_before_cursor(),
            KeyCode::Delete => app.input_state.delete_char_after_cursor(),
            KeyCode::Char(value) => app.input_state.add_char(value),
            KeyCode::Left => app.input_state.move_cursor_left(),
            KeyCode::Right => app.input_state.move_cursor_right(),
            KeyCode::Enter => {
                let list_name = app.input_state.get_text().to_string();
                // Only do something if the list has a name
                if !list_name.trim().is_empty() {
                    if app.input_state.is_modifying {
                        if let Err(e) = ListsComponent::update_list(
                            &mut app.lists_component,
                            list_name,
                            &app.pool,
                        )
                        .await
                        {
                            eprintln!("Failed to update list: {}", e);
                        } else {
                            app.go_back();
                            app.input_state.clear();
                        }
                    } else if let Err(e) =
                        ListsComponent::create_list(&mut app.lists_component, list_name, &app.pool)
                            .await
                    {
                        eprintln!("Failed to create list: {}", e);
                    } else {
                        app.go_back();
                        app.input_state.clear();
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle key press from user in add item screen
    pub async fn handle_add_or_modify_item_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => app.exit_add_item_without_saving(),
            KeyCode::Backspace => app.input_state.remove_char_before_cursor(),
            KeyCode::Delete => app.input_state.delete_char_after_cursor(),
            KeyCode::Left => app.input_state.move_cursor_left(),
            KeyCode::Right => app.input_state.move_cursor_right(),
            KeyCode::Char(value) => app.input_state.add_char(value),
            KeyCode::Enter => {
                let item_name = app.input_state.get_text().to_string();
                if !item_name.trim().is_empty()
                    && let Some(selected_list) = app.lists_component.get_selected_list_mut()
                {
                    if app.input_state.is_modifying {
                        if let Err(e) =
                            ItemsComponent::update_item(selected_list, item_name, &app.pool).await
                        {
                            eprintln!("Failed to update item: {}", e);
                        } else {
                            app.current_screen = CurrentScreen::ItemSelection;
                            app.input_state.clear();
                        }
                    } else if let Err(e) =
                        ItemsComponent::create_item(selected_list, item_name, &app.pool).await
                    {
                        eprintln!("Failed to create item: {}", e);
                    } else {
                        app.current_screen = CurrentScreen::ItemSelection;
                        app.input_state.clear();
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle change of db
    pub async fn handle_change_db_screen_key(app: &mut App, key: KeyEvent) {
        if EventHandler::matches_global_keys(app, key) {
            return;
        }

        match key.code {
            KeyCode::Esc => app.exit_change_db_without_saving(),
            KeyCode::Up => app.select_previous_db(),
            KeyCode::Down => app.select_next_db(),
            KeyCode::Char('k') => app.select_previous_db(),
            KeyCode::Char('j') => app.select_next_db(),
            KeyCode::Enter => {
                if let Err(e) = app.switch_to_selected_db().await {
                    eprintln!("Failed to switch database: {}", e);
                }
                app.go_back();
            }
            KeyCode::Char('a') => app.enter_add_db_screen(),
            KeyCode::Char('s') => {
                // Set selected database as default
                if let Err(e) = app.switch_to_selected_db().await {
                    eprintln!("Failed to switch database: {}", e);
                }
                if let Err(e) = app.set_selected_db_as_default().await {
                    eprintln!("Failed to set database as default: {}", e);
                }
            }
            KeyCode::Char('m') => app.enter_modify_db_screen(),
            KeyCode::Char('d') => {
                let db_name = app
                    .config
                    .dbs
                    .iter()
                    .enumerate()
                    .find(|(i, _db)| *i == app.selected_db_index)
                    .map(|(_i, db)| db.name.clone());
                app.pending_delete_db_name = Some(db_name.unwrap());
                app.current_screen = CurrentScreen::DeleteDatabaseConfirmation;
            }
            _ => {}
        }
    }

    pub async fn handle_delete_database_confirmation_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // Perform the deletion
                if let Err(e) = app.delete_selected_db().await {
                    eprintln!("Failed to delete database: {}", e);
                }
                app.pending_delete_db_name = None;
                app.current_screen = CurrentScreen::DBSelection;
                let active = app.current_db_config.name.clone();
                app.selected_db_index = app
                    .config
                    .dbs
                    .iter()
                    .position(|db| db.name == active)
                    .unwrap_or(0);
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // Cancel deletion
                app.pending_delete_db_name = None;
                app.current_screen = CurrentScreen::DBSelection;
            }
            _ => {}
        }
    }

    /// Handle key press from user in add database screen
    pub async fn handle_add_db_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => app.exit_add_db_without_saving(),
            KeyCode::Backspace => app.input_state.remove_char_before_cursor(),
            KeyCode::Delete => app.input_state.delete_char_after_cursor(),
            KeyCode::Char(value) => app.input_state.add_char(value),
            KeyCode::Left => app.input_state.move_cursor_left(),
            KeyCode::Right => app.input_state.move_cursor_right(),
            KeyCode::Enter => {
                let db_name = app.input_state.get_text().to_string();
                if !db_name.trim().is_empty() {
                    if let Err(e) = app.create_new_database(db_name, false).await {
                        eprintln!("Failed to create database: {}", e);
                    } else {
                        app.current_screen = CurrentScreen::DBSelection;
                        app.input_state.clear();
                    }
                }
            }
            _ => {}
        }
    }

    pub async fn handle_modify_db_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let new_name = app.input_state.get_text().to_string();
                if !new_name.trim().is_empty() {
                    if let Err(e) = app.modify_selected_db(Some(new_name), None).await {
                        eprintln!("Failed to modify database: {}", e);
                    }
                }
                app.exit_modify_db_without_saving();
            }
            KeyCode::Esc => {
                app.exit_modify_db_without_saving();
            }
            KeyCode::Backspace => app.input_state.remove_char_before_cursor(),
            KeyCode::Delete => app.input_state.delete_char_after_cursor(),
            KeyCode::Char(value) => app.input_state.add_char(value),
            KeyCode::Left => app.input_state.move_cursor_left(),
            KeyCode::Right => app.input_state.move_cursor_right(),
            _ => {}
        }
    }
}
