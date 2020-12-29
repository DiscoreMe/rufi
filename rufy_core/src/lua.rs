use std::io::{Result as IOResult, Read};
use std::fs::File as File;
use std::path::Path;

use rlua::{
    Lua,
};

pub struct VM {
    lua: Lua,
    dir: String,
}

impl VM {
    pub fn new(dir: String) -> VM {
        VM {
            lua: Lua::new(),
            dir: dir.clone(),
        }
    }

    pub fn init(&self) -> IOResult<()> {
        self.load_file("main.lua")?;
        Ok(())
    }

    pub fn load_file(&self, filepath: &str) -> IOResult<()> {
        let mut file = File::open(Path::new(self.dir.as_str()).join(filepath))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(())
    }
}
