use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::db::models::{NewTodoItem, NewTodoList, Priority, TodoItem, TodoList, UIItem, UIList};
use ratatui::widgets::ListState;

impl TodoList {
    /// Create a new todo list
    pub async fn create(pool: &SqlitePool, new_list: NewTodoList) -> Result<TodoList> {
        let now = Utc::now();

        // Use query_as to map results to a struct
        let row = sqlx::query_as::<_, TodoList>(
            r#"
            INSERT INTO todo_lists (name, created_at, updated_at)
            VALUES (?1, ?2, ?3)
            RETURNING id, name, created_at, updated_at
            "#,
        )
        .bind(&new_list.name)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .with_context(|| "Failed to create todo list")?;

        Ok(row)
    }

    /// Get all todo lists
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<TodoList>> {
        let lists = sqlx::query_as::<_, TodoList>(
            "SELECT id, name, created_at, updated_at FROM todo_lists ORDER BY created_at",
        )
        .fetch_all(pool)
        .await
        .with_context(|| "Failed to fetch all todo lists")?;

        Ok(lists)
    }

    /// Get a specific todo list by ID
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<TodoList>> {
        let list = sqlx::query_as::<_, TodoList>(
            "SELECT id, name, created_at, updated_at FROM todo_lists WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .with_context(|| "Failed to fetch todo list by id")?;

        Ok(list)
    }

    /// Update todo list name
    pub async fn update_name(&mut self, pool: &SqlitePool, new_name: String) -> Result<()> {
        let now = Utc::now();

        sqlx::query("UPDATE todo_lists SET name = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(&new_name)
            .bind(now)
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to update todo list")?;

        self.name = new_name;
        self.updated_at = now;
        Ok(())
    }

    /// Delete todo list (and all its items due to CASCADE)
    pub async fn delete(self, pool: &SqlitePool) -> Result<()> {
        sqlx::query("DELETE FROM todo_lists WHERE id = ?1")
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to delete todo list")?;

        Ok(())
    }
}

impl TodoItem {
    /// Create a new todo item
    pub async fn create(pool: &SqlitePool, new_item: NewTodoItem) -> Result<TodoItem> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, TodoItem>(
            r#"
            INSERT INTO todo_items (list_id, name, is_done, priority, due_date, created_at, updated_at)
            VALUES (?1, ?2, FALSE, ?3, ?4, ?5, ?6)
            RETURNING id, list_id, name, is_done, priority, due_date, created_at, updated_at
            "#,
        )
        .bind(new_item.list_id)
        .bind(&new_item.name)
        .bind(&new_item.priority)
        .bind(new_item.due_date)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .with_context(|| "Failed to create todo item")?;

        Ok(row)
    }

    /// Get all items for a specific list
    pub async fn get_by_list_id(pool: &SqlitePool, list_id: i64) -> Result<Vec<TodoItem>> {
        let items = sqlx::query_as::<_, TodoItem>(
            r#"
            SELECT id, list_id, name, is_done, priority, due_date, created_at, updated_at
            FROM todo_items 
            WHERE list_id = ?1 
            ORDER BY created_at
            "#,
        )
        .bind(list_id)
        .fetch_all(pool)
        .await
        .with_context(|| "Failed to fetch todo items")?;

        Ok(items)
    }

    /// Get item with a specific id
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<TodoItem>> {
        let item = sqlx::query_as::<_, TodoItem>(
            r#"
            SELECT id, list_id, name, is_done, priority, due_date, created_at, updated_at
            FROM todo_items 
            WHERE id = ?1 
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .with_context(|| "Failed to fetch todo item")?;

        Ok(item)
    }

    /// Update to-do item name
    pub async fn update_name(&mut self, pool: &SqlitePool, new_name: String) -> Result<()> {
        let now = Utc::now();

        sqlx::query("UPDATE todo_items SET name = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(&new_name)
            .bind(now)
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to update todo item name")?;

        self.name = new_name;
        self.updated_at = now;

        Ok(())
    }

    /// Toggle item completion status (from false to true or from true to false)
    pub async fn toggle_done(&mut self, pool: &SqlitePool) -> Result<()> {
        let now = Utc::now();
        let new_status = !self.is_done;

        sqlx::query("UPDATE todo_items SET is_done = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(new_status)
            .bind(now)
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to update todo item status")?;

        self.is_done = new_status;
        self.updated_at = now;

        Ok(())
    }

    /// Update item priority
    pub async fn update_priority(
        &mut self,
        pool: &SqlitePool,
        new_priority: Priority,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query("UPDATE todo_items SET priority = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(&new_priority)
            .bind(now)
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to update todo item priority")?;

        self.priority = Some(new_priority);
        self.updated_at = now;

        Ok(())
    }

    /// Update item due date
    pub async fn update_due_date(
        &mut self,
        pool: &SqlitePool,
        new_due_date: DateTime<Utc>,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query("UPDATE todo_items SET due_date = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(new_due_date)
            .bind(now)
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to update todo item priority")?;

        self.due_date = Some(new_due_date);
        self.updated_at = now;
        Ok(())
    }

    /// Delete todo item
    pub async fn delete(self, pool: &SqlitePool) -> Result<()> {
        sqlx::query("DELETE FROM todo_items WHERE id = ?1")
            .bind(self.id)
            .execute(pool)
            .await
            .with_context(|| "Failed to delete todo item")?;

        Ok(())
    }
}

impl UIList {
    /// Get all lists in db already attached to their items
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<UIList>> {
        // Fetch all lists
        let lists = TodoList::get_all(pool)
            .await
            .with_context(|| "Failed to fetch lists from db")?;

        let mut ui_lists = Vec::new();

        // For each list, fetch its items and create a UIList
        for list in lists {
            let items = TodoItem::get_by_list_id(pool, list.id)
                .await
                .with_context(|| format!("Failed to fetch items for list {}", list.id))?
                .iter()
                .map(|i| UIItem {
                    item: i.clone(),
                    state: ListState::default(),
                })
                .collect();

            ui_lists.push(UIList {
                list,
                item_state: ListState::default(),
                items,
            });
        }

        Ok(ui_lists)
    }

    /// Update items when something changes (new item, deleted item).
    /// Keeps the same list state instead of reinitializing it
    pub async fn update_items(&mut self, pool: &SqlitePool) -> Result<()> {
        // Re-fetch the items but don't change the list state
        let items = TodoItem::get_by_list_id(pool, self.list.id)
            .await
            .with_context(|| "Failed to fetch items for list")?
            .iter()
            .map(|i| UIItem {
                item: i.clone(),
                state: self.item_state.clone(),
            })
            .collect();

        // Update the items
        self.items = items;

        Ok(())
    }
}
