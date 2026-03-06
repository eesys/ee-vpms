pub mod services {
    pub const OWNER: &str = "owner";
    pub const API: &str = "api";
}

pub mod defaults {
    pub const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/ee_vpms";
    pub const API_SERVICE_ADDR: &str = "http://[::1]:8080";
    pub const OWNER_SERVICE_ADDR: &str = "http://[::1]:50051";
}
