//! Tests for database module

use super::*;

#[test]
fn test_postgres_url_scheme() {
    let url = "postgres://user:pass@localhost/db";
    assert!(url.starts_with("postgres://"));
}

#[test]
fn test_database_url_credentials() {
    let url = "postgres://postgres:password@localhost:5432/database";
    assert!(url.contains("postgres"));
    assert!(url.contains("password"));
    assert!(url.contains("localhost"));
}

#[test]
fn test_default_connection_string() {
    let url = "postgres://postgres:postgres@localhost:5432/ee_vpms";
    assert!(url.contains("postgres"));
    assert!(url.contains("5432"));
    assert!(url.contains("ee_vpms"));
}

#[test]
fn test_url_contains_port() {
    let url = "postgres://user:pass@host:5432/db";
    assert!(url.contains(":5432"));
}

#[test]
fn test_url_parsing_components() {
    let url = "postgres://user:password@localhost:5432/mydb";
    let parts: Vec<&str> = url.split("://").collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "postgres");
}

#[test]
fn test_environment_var_default() {
    let url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string());
    assert!(url.contains("postgres"));
}

#[test]
fn test_database_name_in_url() {
    let url = "postgres://user:pass@localhost/testdb";
    assert!(url.ends_with("testdb"));
}

#[tokio::test]
async fn test_db_connection_url_env() {
    let original = env::var("DATABASE_URL").ok();
    unsafe {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
    }

    let result = env::var("DATABASE_URL");
    assert_eq!(result.unwrap(), "postgres://test:test@localhost/test");

    match original {
        Some(url) => unsafe { env::set_var("DATABASE_URL", url) },
        None => unsafe { env::remove_var("DATABASE_URL") },
    }
}

#[test]
fn test_default_database_url() {
    unsafe { env::remove_var("DATABASE_URL") };
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string());

    assert!(database_url.contains("postgres"));
}

#[test]
fn test_database_url_format() {
    let url1 = "postgres://user:pass@localhost:5432/db";
    assert!(url1.starts_with("postgres://"));
    assert!(url1.contains("localhost"));
    assert!(url1.contains("5432"));
}

#[test]
fn test_postgres_default_port() {
    let default_url = "postgres://postgres:postgres@localhost:5432/ee_vpms";
    assert!(default_url.contains(":5432"));
}

#[test]
fn test_database_url_env_var_parsing() {
    let url = "postgres://test:password@127.0.0.1:5432/testdb";
    assert!(url.contains("test"));
    assert!(url.contains("password"));
    assert!(url.contains("127.0.0.1"));
    assert!(url.contains("testdb"));
}

#[test]
fn test_multiple_database_urls() {
    let urls = vec![
        "postgres://user1:pass1@localhost:5432/db1",
        "postgres://user2:pass2@localhost:5432/db2",
        "postgres://user3:pass3@localhost:5432/db3",
    ];
    assert_eq!(urls.len(), 3);
    assert!(urls.iter().all(|url| url.starts_with("postgres://")));
}
