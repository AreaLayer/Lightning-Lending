use crate::models::Nip;
use nostr::PubKey;
use nostr::Sign;
use nostr::Event;
use nostr::Relay;

impl Nip {
    pub fn new(pubkey: PubKey) -> Self {
        Self {
            pubkey,
        }
    }
}

impl Sign for Nip {
    fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.pubkey.sign(msg)
    }
}

impl From<PubKey> for Nip {
    fn from(pubkey: PubKey) -> Self {
        Self {
            pubkey,
        }
    }
}

impl From<&PubKey> for Nip {
    fn from(pubkey: &PubKey) -> Self {
        Self {
            pubkey: pubkey.clone(),
        }
    }
}

impl From<Nip> for PubKey {
    fn from(nip: Nip) -> Self {
        nip.pubkey
    }
}

impl From<&Nip> for PubKey {
    fn from(nip: &Nip) -> Self {
        nip.pubkey.clone()
    }
}

impl From<Nip> for Event {
    fn from(nip: Nip) -> Self {
        Event::Nip(nip)
    }
}

impl From<Relay> for Event {
    fn from(relay: Relay) -> Self {
        Event::Relay(relay)
    }
}