use uuid::Uuid;

use crate::models::user::schema::User;

pub mod schema;

pub async fn exists(
    email: &String,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<Option<bool>> {
    let a: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(email.to_owned().to_ascii_lowercase())
        .fetch_one(db)
        .await?;
    Ok(a)
}

pub async fn new(
    name: &String,
    email: &String,
    hashed_password: &String,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<User> {
    let user: User = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        name,
        email.to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

pub async fn get_by_email(
    email: &String,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        email.to_ascii_lowercase()
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}

pub async fn get_by_id(id: &Uuid, db: &sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(db)
        .await?;
    Ok(user)
}
