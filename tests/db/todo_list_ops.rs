use crate::helpers::db::setup_test_db;
use anyhow::Result;
use chrono::Utc;
use judo::db::models::{NewTodoList, TodoList};

#[tokio::test]
async fn test_todo_list_crud_operations() -> Result<()> {
    // Set up test database
    let pool = setup_test_db().await?;

    // Create a new todo list
    let new_list = NewTodoList {
        name: "Test Shopping List".to_string(),
    };

    let created_list = TodoList::create(&pool, new_list).await?;

    // Verify the created list has correct properties
    assert_eq!(created_list.name, "Test Shopping List");
    assert!(created_list.id > 0);
    assert!(created_list.created_at <= Utc::now());
    assert_eq!(created_list.created_at, created_list.updated_at);

    // Test 2: Get the list by ID
    let fetched_list = TodoList::get_by_id(&pool, created_list.id)
        .await?
        .expect("List should exist");

    assert_eq!(fetched_list.id, created_list.id);
    assert_eq!(fetched_list.name, created_list.name);
    assert_eq!(fetched_list.created_at, created_list.created_at);

    // Test 3: Update the list name
    let mut mutable_list = fetched_list;
    let new_name = "Updated Shopping List".to_string();
    mutable_list.update_name(&pool, new_name.clone()).await?;

    assert_eq!(mutable_list.name, new_name);
    assert!(mutable_list.updated_at > mutable_list.created_at);

    // Test 4: Verify the update persisted in database
    let updated_from_db = TodoList::get_by_id(&pool, mutable_list.id)
        .await?
        .expect("List should still exist");

    assert_eq!(updated_from_db.name, new_name);
    assert_eq!(updated_from_db.updated_at, mutable_list.updated_at);

    // Test 5: Get all lists (should contain our list)
    let all_lists = TodoList::get_all(&pool).await?;
    assert_eq!(all_lists.len(), 1);
    assert_eq!(all_lists[0].name, new_name);

    // Test 6: Delete the list
    updated_from_db.delete(&pool).await?;

    // Test 7: Verify deletion
    let deleted_list = TodoList::get_by_id(&pool, mutable_list.id).await?;
    assert!(deleted_list.is_none());

    let empty_lists = TodoList::get_all(&pool).await?;
    assert_eq!(empty_lists.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_get_nonexistent_todo_list() -> Result<()> {
    // Arrange
    let pool = setup_test_db().await?;

    // Act: Try to get a list that doesn't exist
    let result = TodoList::get_by_id(&pool, 999).await?;

    // Assert: Should return None, not an error
    assert!(result.is_none());

    Ok(())
}

#[tokio::test]
async fn test_get_all_empty_database() -> Result<()> {
    // Arrange
    let pool = setup_test_db().await?;

    // Get all lists from empty database
    let lists = TodoList::get_all(&pool).await?;

    // Should return empty vector
    assert_eq!(lists.len(), 0);

    Ok(())
}
