use std::fs::File;
use std::io::Read;

use rlua::{
  Lua,
  Result,
  Function,
};

fn main() -> Result<()> {
  let mut file = File::open("project/main.lua").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents);

  let lua: Lua = Lua::new();
  lua.context(|lua_ctx| {
    let globals = lua_ctx.globals();
    lua_ctx.load(contents.as_str()).exec()?;

    let init_function: Function = globals.get("onInit")?;
    init_function.call::<_, ()>(())?;
    init_function.call::<_, ()>(())?;
    Ok(())
  })?;

  Ok(())
}