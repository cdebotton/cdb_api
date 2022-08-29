use super::user::User;

#[derive(Default)]
pub struct Auth {
    pub user: Option<User>,
}

impl Auth {
    pub async fn authorize(&mut self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let rows = sqlx::query_as!(User, "SELECT * FROM \"user\"")
            .fetch_all(db)
            .await?;

        Ok(())
    }
}
