use bitcoin::secp256k1::{PublicKey, SecretKey};
use bitcoin::{Address, Network};
use miniscript::bitcoin::PublicKey as MiniscriptPublicKey;

pub fn get_address(secret_key: &SecretKey, network: Network) -> Address {
    let public_key = PublicKey::from_secret_key(secret_key);
    Address::p2pkh(&public_key, network)
}


pub fn get_miniscript_address(secret_key: &SecretKey, network: Network) -> MiniscriptPublicKey {
    let public_key = PublicKey::from_secret_key(secret_key);
    MiniscriptPublicKey::from_public_key(&public_key);
}