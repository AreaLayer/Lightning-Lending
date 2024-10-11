use crate::P2P;
use crate::Kind38383;
use crate::tags::{D, K, F, S, ATM, Fa, Pm, Premium, Source, Network, Layer, Name, G , Bond, Experiration, Y , Z};
use nostr::Relay;
use nostr::Client;

fn main () {
    let mut p2p = P2P::new();
    let mut relay = Relay::new();
    let mut tags = Vec::new();
}

pub struct P2P {
    client: Client,
}

impl P2P {
    pub fn new() -> Self {
        P2P {
            client: Client::new(),
        }
    }
}

impl P2P {
    pub fn connect(&mut self, addr: &str) {
        self.client.connect(addr);
        self.client.send(Kind38383::new(tags));
        self.client.url("relay.damus.io");
        self.client.port(443);
    }
}
pub struct Kind38383 {
    tags: Vec<Tag>,
    pub(crate) kind: Kind,
    pub(crate) version: u8,
}

pub struct Tag {
    pub(crate) kind: Kind,
    pub(crate) name: String,
    pub(crate) value: String,
}