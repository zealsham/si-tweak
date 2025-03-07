use config::{Config,File};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Ok, Result};


#[derive(Debug, Deserialize, Serialize)]
#[serde(default)] 
pub struct BitcoinNode {
    pub rpc_url: String,
    pub rpc_port: u32,
    pub rpc_user: String,
    pub rpc_password: String,
    pub network: String ,
}

impl Default for  BitcoinNode {
    fn default() -> Self {
        Self {
            rpc_url: "127.0.0.1".to_string(),
            rpc_port: 8832,
            rpc_user: "user".to_string(),
            rpc_password: "password".to_string(),
            network: "mainnet".to_string(),
            
            
        }
    }
}



pub fn load_config(path: &str) -> Result<BitcoinNode> {
    let cfg = Config::builder().add_source(File::with_name(path).required(false))
    .build()?;

    let conf : BitcoinNode = cfg.try_deserialize()?;
    Ok(conf)

    
}