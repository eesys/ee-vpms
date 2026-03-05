//! Binary entry point for ee-vpms API

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ee_vpms_api::run().await
}
