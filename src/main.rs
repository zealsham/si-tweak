#![allow(unused)]

mod database;
mod configuration;
mod indexer;
use configuration::btc::load_config;
use indexer::blockchain::fetch_blocks;

use database::{db,model};  

fn main() {
   /*let see = db::Db::new("./db.db").unwrap();
    let hash =vec![0u8; 32];
    let data = model::Block {
        height: 1,
        hash: hash.clone()
    };

    let scalar = model::BlockScalar{
        id:0,
        height:1,
        scalars: hash

    };


    see.insert_block(&data).expect("Failed to insert block");
    see.insert_scalars(&scalar).expect("Failed to insert scalar");
     */
    
    let configff =load_config("./config.yaml").unwrap();
    println!("{:?}",configff);
   
    let b =fetch_blocks("./config.yaml",Some(50000)).unwrap();

  
    
}


