use sea_orm::{Database, DbConn};

pub async fn init(ds: String) -> crate::Result<DbConn> {
    let db = Database::connect(ds)
        .await
        .map_err(|e| crate::Error::Database(e.to_string()))?;

    Ok(db)
}
