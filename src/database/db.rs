#![allow(unused)]
use rusqlite::{Connection,Result,params};
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
    pub fn insert_block(&self, block: &Block)-> Result<()> {
        self.conn.execute(
            "INSERT INTO blocks  (height, hash) VALUES (?1, ?2)", 
            params![block.height,block.hash])?;

            Ok(())

    }

    pub fn insert_scalars(&self,scalar: &BlockScalar)-> Result<()>{
        self.conn.execute(
            "INSERT INTO block_scalars (height,scalars) VALUES (?1, ?2",
             params![scalar.height,scalar.scalars])?;
             Ok(())
    }
    pub fn get_scalar_by_height(&self,height:i64)-> Result<Option<BlockScalar>>{
        let mut stmt = self.conn.prepare(
            "SELECT id, height, scalars FROM block_scalars WHERE height = ?1"
        )?;
        let scalar_iter = stmt.query_map(params![height], |row| {
            Ok(BlockScalar {
                id: row.get(0)?,
                height: row.get(1)?,
                scalars: row.get(2)?,
            })
        })?;

        let result = scalar_iter.collect::<Result<Vec<_>>>()?;
        Ok(result.into_iter().next())

    }
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