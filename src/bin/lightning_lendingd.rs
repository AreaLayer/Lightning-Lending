extern crate lightning;
extern crate dlc;
extern crate nostr;

use lightning::{ln::peer_channel::ChannelDetails, util::logger::Logger};
use dlc::{contract::Contract, payout_curve::PayoutCurve};
use nostr::{self, NodeClient};

use crate::lightning_lending::{
    bitcoin::Bitcoin,
    nostr::Nostr,
    dlc::Dlc,
    lightning::Lightning,
};

struct Logger;

impl Logger {
    fn new() -> Self {
        Logger
    }
}
fn main() {


  env_logger::init();

  info!("Logger initialized");
  
  // Assuming these `new` methods exist and accept a logger
  let lightning = Lightning::new(&logger);
  let bitcoin = Bitcoin::new(&logger);
  let dlc = Dlc::new(&logger);
  let nostr = Nostr::new(&logger);
  let contract = Contract::new(&logger);
  let payout_curve = PayoutCurve::new(&logger);
  let node_client = NodeClient::new(&logger);

}

