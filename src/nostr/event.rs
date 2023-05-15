use nostr::prelude::*;
use tungstenite::{Message as WsMessage};

fn main() -> Result<()> {
    // Generate new random keys
    let my_keys = Keys::generate();

    // or use your already existing
    //
    // From HEX or Bech32
    // let my_keys = Keys::from_sk_str("hex-or-bech32-secret-key")?;

    // Show bech32 public key
    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    let metadata = Metadata::new()
        .name("username")
        .display_name("My Username")
        .about("Description")
        .picture(Url::parse("https://example.com/avatar.png")?)
        .banner(Url::parse("https://example.com/banner.png")?)
        .nip05("username@example.com")
        .lud16("yuki@getalby.com");

    let event: Event = EventBuilder::set_metadata(metadata).to_event(&my_keys)?;

    // New text note
    let event: Event = EventBuilder::new_text_note("Hello from Nostr SDK", &[]).to_event(&my_keys)?;

    // New POW text note
    let event: Event = EventBuilder::new_text_note("My first POW text note from Nostr SDK", &[]).to_pow_event(&my_keys, 20)?;

    // Connect to relay
    let (mut socket, _) = tungstenite::connect("wss://relay.damus.io").expect("Can't connect to relay");

    // Send msg
    let msg = ClientMessage::new_event(event).as_json();
    socket.write_message(WsMessage::Text(msg)).expect("Impossible to send message");

    Ok(())
}
