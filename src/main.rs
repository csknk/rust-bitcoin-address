#![allow(unused)]
use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, CompressedPublicKey, Network, PublicKey};
use std::error::Error;

fn main() {
    /// gen_new_key is just sample code
    // gen_new_key();
    let public_key_fixed =
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_string();
    match p2wpkh_address(&public_key_fixed) {
        Ok(address) => {
            println!("p2wpkh address: {}", address);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    match p2pkh_address(&public_key_fixed) {
        Ok(address) => {
            println!("p2pkh address (compressed): {}", address);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn p2wpkh_address(public_key: &String) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = hex::decode(public_key)?;
    let public_key = PublicKey::from_slice(&decoded)?;
    let comp_pk = CompressedPublicKey(public_key.inner); // p2wpkh requires compressed public key
    let address_p2wpkh = Address::p2wpkh(&comp_pk, Network::Bitcoin);

    Ok(address_p2wpkh.to_string())
}

fn p2pkh_address(public_key: &String) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = hex::decode(public_key)?;
    let public_key = PublicKey::from_slice(&decoded)?;
    let address_p2pkh = Address::p2pkh(&public_key, Network::Bitcoin);

    Ok(address_p2pkh.to_string())
}

/// gen_new_key show code for creating keys and deriving addresses
fn gen_new_key() {
    let s = Secp256k1::new();
    let (secret_key, pub_key) = s.generate_keypair(&mut rand::thread_rng());
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    //
    // Generate pay-to-pubkey-hash address.
    let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("p2pkh address:\t{}", p2pkh_address);
    let comp_key = CompressedPublicKey(public_key.inner);

    let address_p2wpkh = Address::p2wpkh(&comp_key, Network::Bitcoin);
    println!("p2wpkh address:\t{}", address_p2wpkh);
}
