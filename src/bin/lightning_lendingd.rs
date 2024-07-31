extern crate lightning;
extern crate dlc;
extern crate nostr;
extern crate env_logger;

pub struct AppLogger;

impl AppLogger {
    fn new() -> Self {
        AppLogger
    }
}