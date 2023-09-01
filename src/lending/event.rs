use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;
use sha2::{Sha256, Digest};
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, sign, Signature};
use nostr::{serialize, deserialize};

fn create_event(
    pubkey_hex: &str,
    kind: u16,
    content: &str,
    tags: Vec<String>,
    secret_key_hex: &str,
) -> Result<MyEvent, nostr::Error> {
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();

    let event = MyEvent {
        id: calculate_id(pubkey_hex, created_at, kind, content),
        pubkey: pubkey_hex.to_owned(),
        created_at,
        kind,
        tags,
        content: content.to_owned(),
        sig: sign_event(secret_key_hex, pubkey_hex, created_at, kind, content),
    };

    Ok(event)
}

fn calculate_id(
    pubkey_hex: &str,
    created_at: u64,
    kind: u16,
    content: &str,
) -> String {
    let serialized_data = format!(
        "{{\"pubkey\":\"{}\",\"created_at\":{},\"kind\":{},\"content\":\"{}\"}}",
        pubkey_hex, created_at, kind, content
    );

    let mut hasher = Sha256::new();
    hasher.update(serialized_data);
    let result = hasher.finalize();

    hex::encode(result)
}

fn sign_event(
    secret_key_hex: &str,
    pubkey_hex: &str,
    created_at: u64,
    kind: u16,
    content: &str,
) -> String {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_str(secret_key_hex).expect("Invalid secret key");
    let public_key = PublicKey::from_str(pubkey_hex).expect("Invalid public key");

    let serialized_data = format!(
        "{{\"pubkey\":\"{}\",\"created_at\":{},\"kind\":{},\"content\":\"{}\"}}",
        pubkey_hex, created_at, kind, content
    );

    let message = Message::from_slice(serialized_data.as_bytes()).expect("Invalid message");

    let signature = secp.sign(&message, &secret_key);
    let signature_compact = signature.serialize_compact();

    hex::encode(signature_compact)
}


