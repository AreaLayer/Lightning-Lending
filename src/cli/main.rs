extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate secp256k1;
extern crate hex;
extern crate nostr;
extern crate rbf;

use clap::{App, Arg};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use nostr::{Serialize, Deserialize};
use nostr::{EventKind0};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: String,
    pubkey: String,
    created_at: u64,
    kind: u16,
    tags: Vec<String>,
    content: String,
    sig: String,
}

fn main() {
    let matches = App::new("Event Generator CLI")
        .version("1.0.12-beta")
        .author("Area Layer")
        .about("Open/close channels and make payments using Nostr, DLCs and Lightning Network via CLI")
        .arg(
            Arg::with_name("pubkey")
                .short("p")
                .long("pubkey")
                .value_name("PUBKEY_HEX")
                .help("Public key in hexadecimal format")
                .required(true),
        )
        .arg(
            Arg::with_name("kind")
                .short("k")
                .long("kind")
                .value_name("KIND")
                .help("Event kind (integer between 0 and 65535)")
                .required(true),
        )
        .arg(
            Arg::with_name("content")
                .short("c")
                .long("content")
                .value_name("CONTENT")
                .help("Event content")
                .required(true),
        )
        .arg(
            Arg::with_name("tags")
                .short("t")
                .long("tags")
                .value_name("TAGS")
                .help("Event tags, comma-separated")
                .required(true),
        )
        .arg(
            Arg::with_name("secret_key")
                .short("s")
                .long("secret-key")
                .value_name("SECRET_KEY_HEX")
                .help("Secret key in hexadecimal format")
                .required(true),
        )
        .get_matches();

    let pubkey_hex = matches.value_of("pubkey").unwrap();
    let kind = u16::from_str(matches.value_of("kind").unwrap()).expect("Invalid kind value");
    let content = matches.value_of("content").unwrap();
    let tags: Vec<String> = matches
        .value_of("tags")
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let secret_key_hex = matches.value_of("secret_key").unwrap();

    let event = create_event(pubkey_hex, kind, content, tags, secret_key_hex)
        .expect("Failed to create event");

    println!("{:#?}", event);
}

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
