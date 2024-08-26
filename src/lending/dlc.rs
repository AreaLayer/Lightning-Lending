use crate::dlc::DlcHandler;
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use dlc::{BtcDlc, ContractInfo, Dlc, DlcManager, OracleInfo, OfferChannel, AcceptChannel, SignOffer};
use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    let oracle_info = OracleInfo {
        pub_key: oracle_pub_key,
        offerchannel: pub_key_peer,
        oracle_event: "Open channel with other peer",
        maturity_time: 1651339200,
        num_ticks: 1000,
        tick_size: 1000,
    };

    let contract_info = ContractInfo {
        local_amount: 1000000,
        oracle_r_value: Some(oracle_r_value),
        local_collateral: 2000000,
        lock_time: 1648704000,
        refund_lock_time: 1649790400,
        oracle_pub_key: oracle_pub_key,
    };

    let dlc_manager = DlcManager::new();

    let dlc = BtcDlc::new(
        contract_info,
        oracle_info,
        dlc_manager.get_random_bytes(32),
        Some(dlc_manager.get_random_bytes(32)),
    );

    let (funding_tx, funding_address, funding_amount) = get_funding_info();
    let mut psbt_map = HashMap::new();
    let psbt = get_signed_psbt();
    psbt_map.insert(funding_address, psbt);
    let fund_result = dlc.fund_contract(psbt_map, funding_address, psbt);

    // Define the network you want to use
    let network = bitcoin::network::constants::Network::Testnet;
}
