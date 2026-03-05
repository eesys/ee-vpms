//! Entry point for ee-vpms API server

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ee_vpms_api::run().await
}
