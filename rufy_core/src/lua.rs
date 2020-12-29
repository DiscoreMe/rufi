use std::io::{Result as IOResult, Read};
use std::fs::File as File;
use std::path::Path;

use crate::Result;

use rlua::{
    Lua,
    Result as LuaResult,
    Function,
};
use rlua::prelude::LuaError;

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

    pub fn init(&self) -> Result<()> {
        self.load_file("main.lua")?;
        Ok(())
    }

    pub fn load_file(&self, filepath: &str) -> LuaResult<()> {
        // todo: implement custom error to support LuaResult and IOResult
        let mut file = File::open(Path::new(self.dir.as_str()).join(filepath)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        self.lua.context(|ctx| {
            ctx.load(content.as_str()).exec()?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn run(&self) -> LuaResult<()> {
        self.lua.context(|ctx|{
            let global = ctx.globals();
            let init_function: Function = global.get("onInit")?;
            init_function.call::<_, ()>(())?;
            Ok(())
        })?;

        Ok(())
    }
}
