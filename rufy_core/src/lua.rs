use std::io::{Result as IOResult, Read};
use std::fs::File as File;
use std::path::Path;

use walkdir::{WalkDir, DirEntry};

use crate::Result;

use rlua::{Lua, Result as LuaResult, Function, Table};
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
        self.load_files()?;
        Ok(())
    }

    pub fn load_file(&self, filepath: &str) -> LuaResult<()> {
        // todo: implement custom error to support LuaResult and IOResult
        let mut file = File::open(Path::new(self.dir.as_str()).join(filepath)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        self.lua.context(|ctx| {
            ctx.load(content.as_str()).set_name(filepath)?.exec()?;
            Ok(())
        })?;

        Ok(())
    }

    fn load_files(&self) -> IOResult<()> {
        for entry in WalkDir::new(Path::new(self.dir.as_str())) {
            let entry: DirEntry = entry?;
            if !entry.file_type().is_dir() {
                let filename: &Path  = entry.file_name().as_ref();
                let mut file = "";
                match filename.to_str() {
                    None => eprintln!("Error load file: {:#?}", file),
                    Some(f) => file = f,
                }
                let file = str::replace(file, "", "");

                match self.load_file(file.as_str()) {
                    Err(err) => eprintln!("Error load file: {:#?}: {:#?}", file, err),
                    _ => {},
                }
            }
        }

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

