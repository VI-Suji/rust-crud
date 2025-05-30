use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::model::{User, UserInfo};

pub struct UserService {
    pool: PgPool
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres:abc123@localhost/postgres")
            .await?;
        Ok(Self {pool})
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn get_users_by_id(&self, id: i32) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserInfo) -> Result<(), Error> {
        sqlx::query("INSERT INTO users(name, occupation, email, phone) VALUES ($1, $2, $3, $4)")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_user(&self, user: UserInfo) -> Result<(), Error> {
        sqlx::query("UPDATE users SET name = $1, occupation = $2, email = $3, phone = $4")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}