#![allow(unused)]
pub struct Block {
    pub height: i32,
    pub hash: Vec<u8> 
}

pub struct  BlockScalar {
    pub id: i64,
    pub height: i32,
    pub scalars: Vec<u8>,
}


//TO-DO 
// allow option to reutn only taproot transaction for clients that want to do all parts of the scanning themselves 

