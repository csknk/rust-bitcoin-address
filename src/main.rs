#![allow(unused)]
use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, CompressedPublicKey, Network, PublicKey};
use std::error::Error;

/// The different types of addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AddressType {
    /// Pay to pubkey hash.
    P2pkh,
    /// Pay to script hash.
    P2sh,
    /// Pay to witness pubkey hash.
    P2wpkh,
    /// Pay to witness script hash.
    P2wsh,
    /// Pay to taproot.
    P2tr,
}

fn main() {
    let s = Secp256k1::new();
    let (secret_key, pub_key) = s.generate_keypair(&mut rand::thread_rng());
    //
    // println!("Public Key:");
    // for byte in public_key.serialize_uncompressed().iter() {
    //     print!("{:02x}", byte);
    // }
    // println!("\n---\n");
    //
    // println!("Compressed Public Key:");
    // for byte in public_key.serialize().iter() {
    //     print!("{:02x}", byte);
    // }
    //
    // Generate random key pair.
    // let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    //
    // Generate pay-to-pubkey-hash address.
    let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("p2pkh address:\t{}", p2pkh_address);
    let comp_key = CompressedPublicKey(public_key.inner);

    let address_p2wpkh = Address::p2wpkh(&comp_key, Network::Bitcoin);
    println!("p2wpkh address:\t{}", address_p2wpkh);

    let public_key_fixed =
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_string();
    match address(&public_key_fixed) {
        Ok(address) => {
            println!("Address: {}", address);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn address(public_key: &String) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = hex::decode(public_key)?;
    let public_key = PublicKey::from_slice(&decoded)?;
    let comp_pk = CompressedPublicKey(public_key.inner);
    let address_p2wpkh = Address::p2wpkh(&comp_pk, Network::Bitcoin);

    Ok(address_p2wpkh.to_string())
}
