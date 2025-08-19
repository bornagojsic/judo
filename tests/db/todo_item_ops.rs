use crate::helpers::db::setup_test_db;
use anyhow::Result;
use chrono::Utc;
use td::db::models::{NewTodoItem, NewTodoList, Priority, TodoItem, TodoList};

#[tokio::test]
async fn test_todo_item_crud_operations() -> Result<()> {
    // Set up test database
    let pool = setup_test_db().await?;

    // Create a new todo list
    let new_list = NewTodoList {
        name: "Test Shopping List".to_string(),
    };

    let created_list = TodoList::create(&pool, new_list).await?;

    // Create a new todo item with all fields
    let new_item_with_all_fields = NewTodoItem {
        list_id: created_list.id,
        name: "My item".to_string(),
        priority: Some(Priority::High),
        due_date: Some(Utc::now()),
    };

    // Create a new todo item without due date
    let new_item_without_due_date = NewTodoItem {
        list_id: created_list.id,
        name: "My item without date".to_string(),
        priority: Some(Priority::Low),
        due_date: None,
    };

    let mut created_item_with_all_felds = TodoItem::create(&pool, new_item_with_all_fields).await?;

    // Verify the created list has correct properties
    assert!(created_item_with_all_felds.id > 0);
    assert_eq!(created_item_with_all_felds.list_id, created_list.id);
    assert_eq!(created_item_with_all_felds.is_done, false);
    assert_eq!(created_item_with_all_felds.name, "My item");
    assert_eq!(created_item_with_all_felds.priority, Some(Priority::High));
    assert!(created_item_with_all_felds.due_date.is_some());
    assert!(created_item_with_all_felds.created_at <= Utc::now());
    assert_eq!(
        created_item_with_all_felds.created_at,
        created_item_with_all_felds.updated_at
    );

    let created_item_without_due_date = TodoItem::create(&pool, new_item_without_due_date).await?;

    // Verify the created list has correct properties
    assert!(created_item_without_due_date.id > 0);
    assert_eq!(created_item_without_due_date.list_id, created_list.id);
    assert_eq!(created_item_without_due_date.name, "My item without date");
    assert_eq!(created_item_without_due_date.is_done, false);
    assert_eq!(created_item_without_due_date.priority, Some(Priority::Low));
    assert!(created_item_without_due_date.due_date.is_none());
    assert!(created_item_without_due_date.created_at <= Utc::now());
    assert_eq!(
        created_item_without_due_date.created_at,
        created_item_without_due_date.updated_at
    );

    // Fetch items in the list
    let fetched_items_by_list = TodoItem::get_by_list_id(&pool, created_list.id).await?;
    assert_eq!(fetched_items_by_list.len(), 2);

    // Fetch specific item
    let fetched_item = TodoItem::get_by_id(&pool, created_item_with_all_felds.id)
        .await?
        .expect("Item should exist");
    assert_eq!(fetched_item.name, "My item");

    // Update name
    let new_name = "New name".to_string();
    created_item_with_all_felds
        .update_name(&pool, new_name.clone())
        .await?;
    let fetched_item_after_name_change = TodoItem::get_by_id(&pool, created_item_with_all_felds.id)
        .await?
        .expect("Item should exist");

    assert_eq!(created_item_with_all_felds.name, new_name);
    assert_eq!(fetched_item_after_name_change.name, new_name);

    // Toggle done
    // TODO

    Ok(())
}
