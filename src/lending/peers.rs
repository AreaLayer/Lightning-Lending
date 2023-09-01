extern crate lightning;
extern crate secp256k1;
extern crate hex;
extern crate nostr;

use lightning::ln::chan_utils::TxCreationKeys;
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use nostr::{serialize, deserialize};
use std::str::FromStr;

fn main() {
    // Generate random private and public keys for Alice and Bob
    let secp = Secp256k1::new();
    let alice_secret_key = SecretKey::new(&secp, &mut rand::thread_rng());
    let alice_public_key = PublicKey::from_secret_key(&secp, &alice_secret_key);

    let bob_secret_key = SecretKey::new(&secp, &mut rand::thread_rng());
    let bob_public_key = PublicKey::from_secret_key(&secp, &bob_secret_key);

    // Create a payment channel between Alice and Bob
    let channel_funding_txid = "tx123"; // Replace with an actual transaction ID
    let channel_funding_output_index = 0; // Replace with the output index
    let channel_value_satoshis = 1000000; // Replace with the channel value

    let keys = TxCreationKeys {
        per_commitment_point: alice_public_key.clone(),
        revocation_key: bob_public_key.clone(),
        broadcaster_htlc_key: alice_public_key.clone(),
        countersignatory_htlc_key: bob_public_key.clone(),
        broadcaster_delayed_payment_key: alice_public_key.clone(),
        countersignatory_delayed_payment_key: bob_public_key.clone(),
        revocation_hash: [0; 32], // Replace with actual revocation hash
        contest_delay: 144, // Replace with actual contest delay
        commitment_feerate: 253, // Replace with actual feerate
    };

    // Serialize and hash the channel data for Nostr
    let channel_data = serialize(&keys).expect("Failed to serialize channel data");
    let mut hasher = Sha256::new();
    hasher.update(&channel_data);
    let channel_id = hex::encode(hasher.finalize());

    // Sign the channel data with Alice's secret key using Rust-nostr
    let channel_signature = sign_channel_data(&alice_secret_key, &channel_data);

    // Print the channel ID and Alice's signature
    println!("Channel ID: {}", channel_id);
    println!("Alice's Signature: {}", channel_signature);
}

fn sign_channel_data(secret_key: &SecretKey, data: &[u8]) -> String {
    let secp = Secp256k1::new();
    let message = secp256k1::Message::from_slice(data).expect("Invalid message");
    let signature = secp.sign(&message, secret_key);
    let signature_compact = signature.serialize_compact();
    hex::encode(signature_compact)
}

