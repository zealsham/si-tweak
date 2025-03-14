use std::hash::Hash;

use crate::configuration::btc::{self, BitcoinNode};
use bitcoincore_rpc::{Auth,Client,RpcApi};
use anyhow::{Ok, Result,Error};
use bitcoin::{blockdata::transaction::{Transaction,TxIn,TxOut},script::Script,witness::Witness, Block, OutPoint, Txid};
use bitcoin::consensus::deserialize;
const OP_0: u8 = 0x00;
const OP_1: u8 = 0x51;
const OP_DUP: u8 = 0x76;
const OP_HASH160: u8 = 0xa9;
const OP_EQUALVERIFY: u8 = 0x88;
const OP_CHECKSIG: u8 = 0xac;
const OP_PUSHBYTES_20: u8 = 0x14;
const OP_PUSHBYTES_32: u8 = 0x20;




struct InputData {
    outpoint: OutPoint,
    pubkey: Vec<u8>
}



pub fn connect_rpc(config_path: &str) -> Result<(Client,BitcoinNode)> {
    let config = btc::load_config(config_path).unwrap();
    println!("{}",config.rpc_url);
        println!("{}",config.rpc_user);

    let rpc_con  = Client::new(&format!("http://{}:{}",&config.rpc_url,&config.rpc_port),
    Auth::UserPass(config.rpc_password.clone(),
                   config.rpc_user.clone())).unwrap();

        

        Ok((rpc_con,config))   
}

pub fn fetch_blocks(config_path: &str,block_height :Option<u32>) -> Result<()> {
    //no nee to scan blocks before silent payment was accepted
    // we still want the user to parse in a block number too 
    let height = block_height.unwrap_or(0);
    
    let (con,config) = connect_rpc(config_path).unwrap();
    
    
    let bb = con.get_block_hash(300).unwrap();
    let tx: Block = con.get_block(&bb).unwrap();

    let transactions: Vec<Transaction> = deserialize(&bitcoin::consensus::serialize(&tx.txdata))?;  

    //worst case , every transaction 
    let mut relevant_input: Vec<InputData> = Vec::new();

    for tx in transactions {
        let mut relevant_ouput = false ;

        for output in tx.output {
            let scrip_pub = &output.script_pubkey.to_bytes();
            if is_p2wpkh(scrip_pub) || is_p2pkh(scrip_pub) || is_p2tr(scrip_pub){
            relevant_ouput = true;
            //println!("{:?}",scrip_pub)
            
            }
        }

        if relevant_ouput {
            
            for (i,input) in tx.input.iter().enumerate() {
                //skip coinbase tx inputs as we dont need it for silent payment scanning . 
                if input.previous_output.txid.to_string() == "0000000000000000000000000000000000000000000000000000000000000000" {
                    continue;
                }

                


                extract_pubkey_from_input(&input.script_sig.to_bytes(), &input.witness.to_vec());
                
            }

        }



       
    }

    
   
    

    Ok(())
}

pub fn extract_pubkey_from_input(script_sig: &[u8],txinwittness:&Vec<Vec<u8>>,spk: &[u8]) {
    if is_p2pkh(spk) {
        println!("true for p2pkh");

    }

    if is_p2tr(spk) {
        println!("true for p2tr");

    }

    if is_p2wpkh(spk){
        println!("true for p2wpkh");
    }
    
}


pub fn is_p2tr(spk: &[u8]) -> bool {
    matches!(spk, [OP_1, OP_PUSHBYTES_32, ..] if spk.len() == 34)
}


pub fn is_p2wpkh(spk: &[u8]) -> bool {
    matches!(spk, [OP_0, OP_PUSHBYTES_20, ..] if spk.len() == 22)
}

pub fn is_p2pkh(spk: &[u8]) -> bool {
    matches!(spk, [OP_DUP, OP_HASH160, OP_PUSHBYTES_20, .., OP_EQUALVERIFY, OP_CHECKSIG] if spk.len() == 25)
}
