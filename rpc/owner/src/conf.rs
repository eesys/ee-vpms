#[derive(serde::Deserialize)]
pub struct RpcConfig {
    pub name: String,
    pub listen_on: String,
    pub data_source: String,
    pub etcd: EtcdConfig,
}

#[derive(serde::Deserialize)]
pub struct EtcdConfig {
    pub hosts: Vec<String>,
    pub key: String,
}
