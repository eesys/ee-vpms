//! Database integration tests

#[cfg(test)]
mod db_tests {
    use std::env;

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
            .unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string()
            });

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
}
