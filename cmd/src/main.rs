use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::thread;

use rufy_core::{
    Rufy,
    Result,
    Error,
    create_channel,
};
use rufy_engine::Engine;

#[tokio::main]
async fn main() {
    if !Path::new("project").exists() {
        create_dir_all("project").unwrap();
        let mut file = File::create(Path::new("project/main.lua")).unwrap();
        file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls at startup
end").unwrap();
    }


    let mut engine = Engine::new();
    let engine_sx = engine.init_signal();
    engine.listen_signal().await;
    engine_sx.send("init engine".parse().unwrap()).await;

    let mut rufy = Rufy::new(String::from("project")).unwrap();
    let rufy_sx = rufy.init_signal();
    rufy.listen_signal().await;
    rufy_sx.send("init rufy".parse().unwrap()).await;

    // rufy.run();
    // engine.init();
}