extern crate lightning;
extern crate dlc;
extern crate nostr;
extern crate env_logger;

fn main() {
    env_logger::init();
    lightning::run();
}
