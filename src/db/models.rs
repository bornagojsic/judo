use chrono::{DateTime, Utc};
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph, Widget,
};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, FromRow)]
pub struct TodoList {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Clone)]
pub struct TodoItem {
    pub id: i64,
    pub list_id: i64,
    pub name: String,
    pub is_done: bool,
    pub priority: Option<Priority>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Structs for creating new records (without id and timestamps)
#[derive(Debug)]
pub struct NewTodoList {
    pub name: String,
}

#[derive(Debug)]
pub struct NewTodoItem {
    pub list_id: i64,
    pub name: String,
    pub priority: Option<Priority>,
    pub due_date: Option<DateTime<Utc>>,
}

// Convenient repackaging of DB items to cache reads from DB
#[derive(Debug)]
pub struct UIList {
    pub list: TodoList,
    pub item_state: ListState,
    pub items: Vec<UIItem>,
}

#[derive(Debug)]
pub struct UIItem {
    pub item: TodoItem,
    pub state: ListState,
}
