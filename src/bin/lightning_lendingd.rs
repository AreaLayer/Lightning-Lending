extern crate lightning;
extern crate dlc;
extern crate nostr;
extern crate env_logger;

use lightning::{util::logger::Logger as LightningLogger}; // Renamed to avoid conflict
use dlc::{Contract, PayoutCurve}; // Assuming these are directly available
use nostr::NodeClient;
use log::info;

struct AppLogger;

impl AppLogger {
    fn new() -> Self {
        AppLogger
    }
}

fn main() {
    env_logger::init();

    info!("Logger initialized");

    let logger = AppLogger::new();
    // Use logger with your specific implementation
    // Assuming new methods accept a reference to AppLogger

    let lightning = Lightning::new(&logger);
    let bitcoin = Bitcoin::new(&logger);
    let dlc = Dlc::new(&logger);
    let nostr = Nostr::new(&logger);
    let contract = Contract::new(&logger);
    let payout_curve = PayoutCurve::new(&logger);
    let node_client = NodeClient::new(&logger);
}


