use crate::script::script::Script;

pub struct TxIn {
    pub prev_tx: Vec<u8>,
    pub prev_ind: usize,
    pub script_sig: Script,
    pub sequence: u64,
}

impl TxIn {
    pub fn new(
        prev_tx: Vec<u8>,
        prev_ind: usize,
        script_sig: Option<Script>,
        sequence: u64,
    ) -> Self {
        let script_sig = if let Some(sc) = script_sig {
            sc
        } else {
            Script::new(None)
        };
        Self {
            prev_tx,
            prev_ind,
            script_sig,
            sequence,
        }
    }
}
