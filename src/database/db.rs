#![allow(unused)]
use rusqlite::{Connection,Result};
use crate::database::model::{Block,BlockScalar};

pub struct Db {
    conn: Connection
}

impl Db {
    pub fn new(&self, path: &str) -> Result<(Self)>{
        let conn = Connection::open(path)?;
        init_database(&conn)?;
        Ok(Db { conn })
        
    }
    pub fn insert_block(){}
    pub fn insert_scalars(){}
    pub fn get_scalar_by_height(){}
    pub fn get_scalar_by_hash(){}

}


fn init_database(conn: &Connection) -> Result<()>{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blocks (
            height INTEGER PRIMARY KEY,
            hash BLOB NOT NULL
        )",
         []
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS block_scalars (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                height INTEGER NOT NULL,
                scalars BLOB NOT NULL,
                FOREIGN KEY (height) REFERENCES blocks(height) ON DELETE CASCADE
            )",
            [],
        )?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_height ON block_scalars(height)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_hash ON blocks(hash)", [])?;
    
        Ok(())


}