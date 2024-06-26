use bitcoin::BumpFee;
use bitcoin::Transaction;
use bitcoin::TX;
use bitcoin::BumpFee::RBF;

fn main {
    let tx: TX = bitcoin::Transaction::default();
    let new_tx = tx.bump_fee(RBF);
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
    assert!(new_tx.is_p2sh());
    assert!(new_tx.is_p2pkh());
    assert!(new_tx.is_p2pkh_or_p2sh());
    assert!(new_tx.is_p2wpkh());
    assert!(new_tx.is_p2wpkh_p2sh());
    assert!(new_tx.is_p2wsh());
    assert!(new_tx.is_p2wsh_p2sh());
    assert!(new_tx.is_p2sh_p2wsh());
    assert!(new_tx.is_p2tr());
    assert!(new_tx.is_p2sh_p2tr());
    assert!(new_tx.is_p2wsh_p2tr());
    assert!(new_tx.is_p2sh_p2wsh_p2tr());
    assert!(new_.tx.is_hex_serialized());
    assert!(new_tx.is_minable());
    assert!(new_tx.is_mined());
    assert!(new_tx.is_final());

}
impl bump_fee {
    fn is_rbf(&self) -> bool {
        self.0.is_rbf()
    }
}

impl bump_fee {
    fn is_final(&self) -> bool {
        self.0.is_final()
    }
}

impl bump_fee {
    fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }
}


impl bump_fee {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}


impl bump_fee {
    fn is_coinbase_or_empty(&self) -> bool {
        self.0.is_coinbase_or_empty()
    }
}


impl bump_fee {
    fn is_standard(&self) -> bool {
        self.0.is_standard()
    }
}

impl bump_fee {
    fn is_standard_or_coinbase(&self) -> bool {
        self.0.is_standard_or_coinbase()
    }
}

impl tx {
    fn is_standard_or_empty(&self) -> bool {
        self.0.is_standard_or_empty()
    }
}

impl tx {
    fn is_witness(&self) -> bool {
        self.0.is_witness()
    }
}

impl tx {
    fn is_witness_or_coinbase(&self) -> bool {
        self.0.is_witness_or_coinbase()
    }
}
