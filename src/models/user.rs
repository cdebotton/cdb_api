use sqlx::FromRow;
use uuid::Uuid;
#[derive(FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn find() {}
    pub fn find_all() {}
    pub fn create() {}
    pub fn destroy() {}
    pub fn update() {}
}
