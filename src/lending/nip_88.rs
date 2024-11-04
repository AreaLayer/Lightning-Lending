use crate::models::Nip;
use nostr::PubKey;
use nostr::Sign;
use nostr::Event;
use nostr::Relay;

use nip88::Eventkid_88;
use nip88::Eventkind_89;
use nip88::Eventkind_90;

pub (crate) fn NP_88() {
    let nip = Nip::new(PubKey::from_str("pubkey").unwrap());
    let event = Event::from(nip);
    let relay = Relay::new("wss://relay.example.com");
    let Eventkid_88 = nip.sign(&event);
    let Eventkind_89 = nip.sign(&event);
    let Eventkind_90 = nip.sign(&event);
    relay.publish(event);
}
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

impl From<Oracle> for Event {
    fn from(oracle: Oracle) -> Self {
        Event::Oracle(oracle)
    }
}

pub (crate) fn Eventkind8_888(event: Event) -> Event {
    match event {
        Event::Nip(nip) => Event::Nip(nip),
        Event::Relay(relay) => Event::Relay(relay),
        Event::Oracle(oracle) => Event::Oracle(oracle),
        _ => Event::Nip(Nip::new(PubKey::from(""))),
    }
}

pub (crate) fn Evetkind_30_088 (event: Event) -> Event {
    match event {
        Event::Nip(nip) => Event::Nip(nip),
        Event::Relay(relay) => Event::Relay(relay),
        Event::Oracle(oracle) => Event::Oracle(oracle),
        _ => Event::Nip(Nip::new(PubKey::from(""))),
    }
}

pub (crate) fn Eventkid_88(event: Event) -> Event {
    match event {
        Event::Nip(nip) => Event::Nip(nip),
        Event::Relay(relay) => Event::Relay(relay),
        Event::Oracle(oracle) => Event::Oracle(oracle),
        _ => Event::Nip(Nip::new(PubKey::from(""))),
    }
}

pub (crate) fn Eventkind_89(event: Event) -> Event {
    match event {
        Event::Nip(nip) => Event::Nip(nip),
        Event::Relay(relay) => Event::Relay(relay),
        Event::Oracle(oracle) => Event::Oracle(oracle),
        _ => Event::Nip(Nip::new(PubKey::from(""))),
    }
}

pub (crate) fn Eventkind_90(event: Event) -> Event {
    match event {
        Event::Nip(nip) => Event::Nip(nip),
        Event::Relay(relay) => Event::Relay(relay),
        Event::Oracle(oracle) => Event::Oracle(oracle),
        _ => Event::Nip(Nip::new(PubKey::from(""))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr::Event;
    use nostr::Relay;

    pub (crate) fn test_nip88() {
        let nip = Nip::new(PubKey::from_str("pubkey").unwrap());
        let event = Event::from(nip);
        let relay = Relay::new("wss://relay.damus.io");
        let Eventkid_88 = nip.sign(&event);
        let Eventkind_89 = nip.sign(&event);
        let Eventkind_90 = nip.sign(&event);
        relay.publish(event);
    }
}