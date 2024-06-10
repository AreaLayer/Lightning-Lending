use std::sync::Arc;

use crate::Channel;


use lightning::chain::chaininterface::{BroadcasterInterface, FeeEstimator};
use lightning::ln::channelmanager::{ChannelManager, ChannelManagerReadArgs};
use lightning::ln::peer_handler::{MessageHandler, PeerHandler, SocketDescriptor};
use lightning::util::config::UserConfig;
use lightning::ln::BOLT12;
use ldk_node::lightning::ln::msgs::SocketAddress;

let confing = BitcoinTestnet;

let config = UserConfig::default(); // Set your Lightning node configuration here
let channel_manager = ChannelManager::new(
    chain_source, // Your chain interface implementation
    keys_manager, // Your keys manager implementation
    fee_estimator, // Your fee estimator implementation
    tx_broadcaster, // Your broadcaster interface implementation
    logger, // Your logging implementation
    config,
    Some(ChannelManagerReadArgs {
        keys_manager_arc: Arc::new(keys_manager.clone()),
        fee_estimator_arc: Arc::new(fee_estimator.clone()),
    }),
    message_handler, // Your message handler implementation
    Some(socket_descriptor), // Your socket descriptor implementation
);
/ Define the network you want to use, e.g. testnet or mainnet
let network = bitcoin::network::constants::Network::Testnet;

let peer_handler = PeerHandler::new(channel_manager);
peer_handler.connect_to_peer(peer_address); // Connect to your counterparty's Lightning node
