mod common;
use common::*;

#[tokio::test]
async fn list_users_returns_list() {
    let result = client().users().list().send().await.unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn get_current_user() {
    let me = client().users().get_current().send().await.unwrap();
    assert!(!me.id.is_empty());
    assert!(!me.email.is_empty());
}
