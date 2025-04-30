use std::sync::Arc;

use crate::Channel;


use lightning::chain::chaininterface::{BroadcasterInterface, FeeEstimator};
use lightning::ln::channelmanager::{ChannelManager, ChannelManagerReadArgs};
use lightning::ln::peer_handler::{MessageHandler, PeerHandler, SocketDescriptor};
use lightning::util::config::UserConfig;
use lightning::ln::BOLT12;
use ldk_node::lightning::ln::msgs::SocketAddress;
use lightning_liquidity::LiquidityProvider;
use lightning_liquidity::LSPS0Client;
use ark_core::CoinSelect;

use bitcoin::network::constants::Network;

fn main() {
    let network = Network::Testnet; // You can switch between testnet and mainnet here
    let config = UserConfig::default();
    let coin_select = CoinSelect::new();

    let channel_manager = ChannelManager::new(
        Arc::new(chain_source),
        Arc::new(keys_manager.clone()), // Clone because it's used in multiple places
        Arc::new(fee_estimator.clone()),
        Arc::new(tx_broadcaster.clone()),
        Arc::new(LSPS0Client.clone()),
        Arc::new(logger.clone()),
        Arc::new(coin_select),
        config,
        network,
        Arc::new(ChainMonitor::new(Arc::new(chain_source), Arc::new(tx_broadcaster), Arc::new(logger), Arc::new(keys_manager.clone()), Arc::new(fee_estimator.clone()))),
    );

    let peer_handler = PeerHandler::new(
        channel_manager.clone(),
        Arc::new(keys_manager.clone()),
        Arc::new(logger.clone()),
        Arc::new(tx_broadcaster.clone()),
        Arc::new(fee_estimator.clone()),
        Arc::new(coin_select),
        config,
        network,
        message_handler,
    );

    peer_handler.connect_peer(peer_address, None).unwrap()
}