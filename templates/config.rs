//! Configuration interface for Bitcoin RPC clients

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}
