#![allow(unused)]

mod database;
use crate::database::{Block,BlockScalar,db};
fn main() {
    let see = db::Db::new("./db.db").unwrap();
    let hash =vec![0u8; 32];
    let data = Block {
        height: 1,
        hash: hash.clone()
    };

    let scalar = BlockScalar{
        id:0,
        height:1,
        scalars: hash

    };

    see.insert_block(&data).expect("Failed to insert block");
    see.insert_scalars(&scalar).expect("Failed to insert scalar");
    
}


