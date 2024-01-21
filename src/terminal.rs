use std;

pub(crate) struct Terminal {
    working_dir: String,
    history: Vec<String>,
    aliases: Vec<String>,
}

impl Terminal {
    pub fn new(working_dir: String) -> Terminal {
        Terminal {
            working_dir,
            history: Vec::new(),
            aliases: Vec::new(),
        }
    }
}