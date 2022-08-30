use chrono::Utc;
use sqlx::FromRow;
use uuid::Uuid;
#[derive(FromRow, Default, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl User {
    pub fn find() {}
    pub fn find_all() {}
    pub fn create() {}
    pub fn destroy() {}
    pub fn update() {}
}
