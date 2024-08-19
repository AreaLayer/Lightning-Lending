use std::sync::Arc;

use crate::Channel;


use lightning::chain::chaininterface::{BroadcasterInterface, FeeEstimator};
use lightning::ln::channelmanager::{ChannelManager, ChannelManagerReadArgs};
use lightning::ln::peer_handler::{MessageHandler, PeerHandler, SocketDescriptor};
use lightning::util::config::UserConfig;
use lightning::ln::BOLT12;
use ldk_node::lightning::ln::msgs::SocketAddress;

use bitcoin::network::constants::Network;

let network = Network::Testnet; // You can switch between testnet and mainnet here
let config = UserConfig::default();
let channel_manager = ChannelManager::new(
    chain_source: Arc::new(chain_source),
    keys_manager: Arc::new(keys_manager), // Your keys manager implementation    fee_estimator: Arc::new(fee_estimator.clone()), // Your fee estimator implementation    tx_broadcaster, // Your broadcaster interface implementation
    logger: Arc::new(logger.clone()),
    config,



    Some(ChannelManagerReadArgs::new(
        Arc::new(keys_manager.clone()),
        Arc::new(fee_estimator.clone()),
    )),    message_handler, // Your message handler implementation
    Some(socket_descriptor!), // Your socket descriptor implementation);

let peer_handler = PeerHandler::new(
    channel_manager.clone(),
    Arc::new(keys_manager.clone()),
    Arc::new(logger.clone()),
    Arc::new(peer_handler_constructor),
);
peer_handler.connect_peer(peer_address, None).unwrap();