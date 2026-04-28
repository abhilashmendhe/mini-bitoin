pub struct Satoshis(u64);

impl Satoshis {
    pub fn new(btc: f64) -> Self {
        let amt = 100_000_000.0 * btc;
        Self(amt as u64)
    }
    pub fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

pub struct TxOut {
    pub satoshis: Satoshis,
    pub script_pub_key: (),
}
