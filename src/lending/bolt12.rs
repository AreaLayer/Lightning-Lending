use lightning::ln::msgs::{ChannelUpdate, NodeId};
use lightning::util::ser::{Reader, Writer};
use lightning::ln::PaymentHash;
use lightning::ln::PaymentSecret;
use lightning::ln::BOLT12;

impl BOLT12 {
    pub fn new(
        node_id: NodeId,
        channel_id: u64,
        timestamp: u64,
        payment_hash: PaymentHash,
        payment_secret: PaymentSecret,
        channel_update: ChannelUpdate,
    ) -> Self {
        let mut writer = Writer::new();
        writer.write_node_id(&node_id);
        writer.write_u64(channel_id);
        writer.write_u64(timestamp);
        writer.write_bytes(&payment_hash.0);
        writer.write_bytes(&payment_secret.0);
        writer.write_bytes(&channel_update.data);
        Self {
            data: writer.into_vec(),
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut reader = Reader::new(bytes);
        let node_id = reader.read_node_id();
        let channel_id = reader.read_u64();
        let timestamp = reader.read_u64();
        let payment_hash = PaymentHash(reader.read_bytes());
        let payment_secret = PaymentSecret(reader.read_bytes());
        let channel_update_data = reader.read_bytes();
        let channel_update = ChannelUpdate::from_bytes(&channel_update_data);
        Self {
            data: bytes.to_vec(),
        }
    }
}