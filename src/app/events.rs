use crate::app::state::{App, CurrentScreen};
use crate::ui::components::ItemsComponent;
use crate::ui::cursor::CursorState;
use crossterm::event::{KeyCode, KeyEvent};

pub struct EventHandler;

impl EventHandler {
    /// Handle key press from user in main screen
    pub async fn handle_main_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => app.exit = true, // Quit application
            KeyCode::Char('s') => app.lists_component.select_next(), // Navigate down in lists
            KeyCode::Char('w') => app.lists_component.select_previous(), // Navigate up in lists
            KeyCode::Char('A') => app.enter_add_list_screen(), // Add new list
            KeyCode::Char('a') => app.enter_add_item_screen(), // Add new item
            KeyCode::Char('D') => {
                if let Err(e) = app.lists_component.delete_selected_list(&app.pool).await {
                    // Log error but don't crash the application
                    eprintln!("Failed to delete list: {}", e);
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
            KeyCode::Down => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::select_next_item(selected_list);
                }
            }
            KeyCode::Up => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::select_previous_item(selected_list);
                }
            }
            KeyCode::Left => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::remove_item_selection(selected_list);
                }
            }
            KeyCode::Right => {
                if let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                    ItemsComponent::select_first_item(selected_list);
                }
            }
            _ => {}
        }
    }

    /// Handle key press from user in add list screen
    pub async fn handle_add_list_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => app.exit_add_list_without_saving(),
            KeyCode::Backspace => app.new_list_state.remove_char_before_cursor(),
            KeyCode::Delete => app.new_list_state.delete_char_after_cursor(),
            KeyCode::Char(value) => app.new_list_state.add_char(value),
            KeyCode::Left => app.new_list_state.move_cursor_left(),
            KeyCode::Right => app.new_list_state.move_cursor_right(),
            KeyCode::Enter => {
                let list_name = app.new_list_state.get_text().to_string();
                if !list_name.trim().is_empty() {
                    if let Err(e) = app.lists_component.create_list(list_name, &app.pool).await {
                        eprintln!("Failed to create list: {}", e);
                    } else {
                        app.current_screen = CurrentScreen::Main;
                        app.new_list_state.clear();
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle key press from user in add item screen
    pub async fn handle_add_item_screen_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => app.exit_add_item_without_saving(),
            KeyCode::Backspace => app.new_item_state.remove_char_before_cursor(),
            KeyCode::Delete => app.new_item_state.delete_char_after_cursor(),
            KeyCode::Left => app.new_item_state.move_cursor_left(),
            KeyCode::Right => app.new_item_state.move_cursor_right(),
            KeyCode::Char(value) => app.new_item_state.add_char(value),
            KeyCode::Enter => {
                let item_name = app.new_item_state.get_text().to_string();
                if !item_name.trim().is_empty()
                    && let Some(selected_list) = app.lists_component.get_selected_list_mut() {
                        if let Err(e) =
                            ItemsComponent::create_item(selected_list, item_name, &app.pool).await
                        {
                            eprintln!("Failed to create item: {}", e);
                        } else {
                            app.current_screen = CurrentScreen::Main;
                            app.new_item_state.clear();
                        }
                    }
            }
            _ => {}
        }
    }
}
