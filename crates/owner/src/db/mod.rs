use sea_orm::{Database, DbConn};
use std::env;

pub async fn init() -> crate::Result<DbConn> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string());

    let db = Database::connect(&database_url)
        .await
        .map_err(|e| crate::Error::Database(e.to_string()))?;

    Ok(db)
}

#[cfg(test)]
mod test;
