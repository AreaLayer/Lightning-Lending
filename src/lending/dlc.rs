use crate::dlc::DlcHandler;
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use dlc::{BtcDlc, ContractInfo, Dlc, DlcManager, OracleInfo, OfferChannel, AcceptChannel, SignOffer}
use std::collections::HashMap;
use std::sync::Arc;

let oracle_info = OracleInfo {
    pub_key: oracle_pub_key,
    offerchannel: pub_key_peer
    oracle_event: "Open channel with other peer",
    maturity_time: 1651339200, // Timestamp for 12:00 UTC on April 30, 2023
    num_ticks: 1000, // Number of price ticks
    tick_size: 1000, // Size of each price tick (in sats)
};
let contract_info = ContractInfo {
    local_amount: 1000000, // Amount of the loan (in sats)
    oracle_r_value: Some(oracle_r_value), // Oracle's R value (if already known)
    local_collateral: 2000000, // Collateral amount (in sats)
    lock_time: 1648704000, // Timestamp for May 1, 2023 (when the contract becomes active)
    refund_lock_time: 1649790400, // Timestamp for May 12, 2023 (when the refund can be claimed)
    oracle_pub_key: oracle_pub_key, // Oracle's public key
};
let dlc_manager = DlcManager::new();
let dlc = BtcDlc::new(
    contract_info,
    oracle_info,
    dlc_manager.get_random_bytes(32),
    Some(dlc_manager.get_random_bytes(32)),
);
let (funding_tx, funding_address, funding_amount) = get_funding_info(); // Get the funding information for the Lightning channel
let mut psbt_map = HashMap::new();
let psbt = get_signed_psbt(); // Get a signed PSBT for the funding transaction
psbt_map.insert(funding_address, psbt);
let fund_result = dlc.fund_contract(psbt_map,funding_address, psbt);

};
/ Define the network you want to use, e.g. testnet or mainnet
let network = bitcoin::network::constants::Network::Testnet;
let network = bitcoin::network::constants::Network::Signet;
