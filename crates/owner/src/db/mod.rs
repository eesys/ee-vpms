use sea_orm::{Database, DbConn};

pub async fn init() -> crate::Result<DbConn> {
    let database_url = "postgres://postgres:Aa123456@localhost:5432/eevpms?sslmode=disable";

    let db = Database::connect(database_url)
        .await
        .map_err(|e| crate::Error::Database(e.to_string()))?;

    Ok(db)
}
