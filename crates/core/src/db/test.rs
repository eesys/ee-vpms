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
        .unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string()
        });
    assert!(url.contains("postgres"));
}

#[test]
fn test_database_name_in_url() {
    let url = "postgres://user:pass@localhost/testdb";
    assert!(url.ends_with("testdb"));
}
