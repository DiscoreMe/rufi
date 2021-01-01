use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use rufy_core::{
    Rufy,
    Result,
    Error,
};
use rufy_engine::Engine;

fn main() -> Result<(), Error> {
    if !Path::new("project").exists() {
        create_dir_all("project")?;
        let mut file = File::create(Path::new("project/main.lua"))?;
        file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls at startup
end")?;
    }

    let engine = Engine::new();
    engine.init();
    let rufy = Rufy::new(String::from("project"))?;
    rufy.run()
}