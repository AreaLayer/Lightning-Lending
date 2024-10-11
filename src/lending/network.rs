use bdk::bitcoin::Transaction;
use bdk::bticoin::BumpFee;
use bdk::bitcoin::Network;
use bdk::bitcoin::Network::{Bitcoin, Testnet, Regtest, Signet};
use bitcoin::Network::Testnet4;

fn main() {
    let tx: Transaction = Transaction::default();
    let new_tx = tx.bump_fee(BumpFee::RBF);
    println!("{:?}", new_tx);
    assert!(new_tx.is_rbf());
    assert!(new_tx.is_final());
    assert!(new_tx.is_coinbase());
    assert!(new_tx.is_empty());
    assert!(new_tx.is_coinbase_or_empty());
    assert!(new_tx.is_standard());
    assert!(new_tx.is_standard_or_coinbase());
    assert!(new_tx.is_standard_or_empty());
    assert!(new_tx.is_witness());
    assert!(new_tx.is_witness_or_coinbase());
    assert!(new_tx.is_witness_or_empty());
    assert!(new_tx.is_provably_unspendable());
    assert!(new_tx.is_witness_v0());
    assert!(new_tx.is_witness_v1());
    assert!(new_tx.is_witness_unknown());
    assert!(new_ tx.is_p2sh());
    assert!(new_tx.is_p2pkh());
    assert!(new_tx.is_p2pkh_or_p2sh());
}

impl BumpFee {
    fn is_rbf(&self) -> bool {
        self.0.is_rbf()
    }
}

impl BumpFee {
    fn is_final(&self) -> bool {
        self.0.is_final()
    }
}

impl Network {
    fn is_signet(&self) -> bool {
        match self {
            Signet => true,
            _ => false,
        }
    }
}

fn is_testnet(network: Network) -> bool {
    match network {
        Testnet => true,
        _ => false,
    }
}

fn is_regtest(network: Network) -> bool {
    match network {
        Regtest => true,
        _ => false,
        }
    }

fn is_bitcoin(network: Network) -> bool {
    match network {
        Bitcoin => true,
        _ => false,
        }
        }

fn is_testnet4(network: Network) -> bool {
    match network {
        Testnet4 => true,
        _ => false,
        }
        }