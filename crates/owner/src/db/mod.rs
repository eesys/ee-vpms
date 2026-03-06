use ee_vpms_shared::{defaults, get_database_url, services};
use sea_orm::{Database, DbConn};

pub async fn init() -> crate::Result<DbConn> {
    let database_url = get_database_url(services::OWNER, "config.toml")
        .unwrap_or_else(|_| defaults::DEFAULT_DATABASE_URL.to_string());

    let db = Database::connect(&database_url)
        .await
        .map_err(|e| crate::Error::Database(e.to_string()))?;

    Ok(db)
}

#[cfg(test)]
mod test;
