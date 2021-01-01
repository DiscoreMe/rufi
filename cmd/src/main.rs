use rufy_core::{
    Rufy,
    Result,
    Error,
};
use rufy_engine::Engine;

fn main() -> Result<(), Error> {
    let engine = Engine::new();
    engine.init();
    let rufy = Rufy::new(String::from("project"))?;
    rufy.run()
}