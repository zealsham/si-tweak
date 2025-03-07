use crate::configuration::btc;
use bitcoincore_rpc::{Auth,Client,RpcApi};
use anyhow::{Ok, Result};



pub fn connect_rpc() -> Result<Client> {
    let config = btc::load_config("../CONFIG.YAML").unwrap();

    let rpc_con  = Client::new(&format!("http://{}:{}",config.rpc_url,config.rpc_port),
    Auth::UserPass(config.rpc_password,
                   config.rpc_user)).unwrap();

        Ok(rpc_con)   
}

pub fn fetch_blocks() -> Result<()> {
    let con = connect_rpc().unwrap();
    let count= con.get_block_count()?;

    Ok(())
}