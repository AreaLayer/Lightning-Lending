use esplora_client::TX;
use esplora_client::TX_HASH;
use esplora_client::TX_INDEX;

pub mod esplora_client;

fn main() {
    let client = esplora_client::Client::new("https://blockstream.info/api");
    let tx = client.get_tx("00000000000000000000000000000000000000000000000000000000000000000");
    println!("{:?}", tx);
    let tx = client.get_tx_with_metadata("0000000000000000000000000000000000000000000000000000000000000000");
    println!("{:?}", tx);
    let tx = client.get_tx_with_metadata_and_mempool("0000000000000000000000000000000000000000000000000000000000000000");
    println!("{:?}", tx);
}

impl esplora_client::Client {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
        }
    pub fn get_tx(&self, tx_hash: TX_HASH) -> Result<TX, reqwest::Error> {
        let url = format!("{}/tx/{}", self.url, tx_hash);
        let res = reqwest::blocking::get(&url)?;
        let tx: TX = res.json()?;
        Ok(tx)
        }
        pub fn get_tx_with_metadata(&self, tx_hash: TX_HASH) -> Result<TX, reqwest::Error> {
            let url = format!("{}/tx/{}/metadata", self.url, tx_hash);
            let res = reqwest::blocking::get(&url)?;
            let tx: TX = res.json()?;
            Ok(tx)
            }
}

impl esplora_client::TX {
    pub fn get_tx_index(&self) -> TX_INDEX {
        self.txid.to_string()
        }
        pub fn get_tx_hash(&self) -> TX_HASH {
            self.txid.to_string()
            }
            pub fn get_block_hash(&self) -> BLOCK_HASH {
                self.blockhash.to_string()
                }
                pub fn get_block_height(&self) -> BLOCK_HEIGHT {
                    self.blockheight.to_string()
                    }
                }