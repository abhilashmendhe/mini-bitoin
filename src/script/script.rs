#[derive(Debug)]
pub struct Script {
    pub cmds: Vec<u8>
}

impl Script {
    pub fn new(cmds: Option<Vec<u8>>) -> Self {
        let n_cmds = if let Some(c) = cmds {
            c
        } else {
            vec![]
        };
        Self { cmds: n_cmds }
    }
}