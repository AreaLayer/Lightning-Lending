use bitcoin::secp256k1::{PublicKey, SecretKey};
use bitcoin::{Address, Network};
use miniscript::bitcoin::PublicKey as MiniscriptPublicKey;
use bitcoin::taproot::{TapTweak, TapTweakExt};

pub fn get_address(secret_key: &SecretKey, network: Network) -> Address {
    let public_key = PublicKey::from_secret_key(secret_key);
    Address::p2pkh(&public_key, network)
}


pub fn get_miniscript_address(secret_key: &SecretKey, network: Network) -> MiniscriptPublicKey {
    let public_key = PublicKey::from_secret_key(secret_key);
    MiniscriptPublicKey::from_public_key(&public_key);
}

pub fn get_miniscript_address_from_string(address: &str, network: Network) -> MiniscriptPublicKey {
    let public_key = PublicKey::from_str(address).unwrap();
    MiniscriptPublicKey::from_public_key(&public_key);
}
pub fn get_taproot_address(secret_key: &SecretKey, network: Network) -> Address {
    let public_key = PublicKey::from_secret_key(secret_key);
    let taproot_tweak = TapTweak::from_secret_key(secret_key);
    let taproot_public_key = public_key.taproot_tweak(&taproot_tweak);
    Address::p2tr(&taproot_public_key, network)
}

