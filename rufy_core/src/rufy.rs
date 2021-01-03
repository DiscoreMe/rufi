use super::lua::VM;

use crate::channel::{
    Sender,
    Receiver,
    create_channel,
};
use crate::engine::Engine;

pub struct Rufy {
    vm: VM,
    sender: Option<Sender>,
    engine: Option<Engine>,
}

use std::thread;

impl Rufy {
    pub fn init(path: String) {
        let vm = VM::new(path.clone());
        vm.init().unwrap();
        vm.run().unwrap();

        let mut rufy = Rufy {
            vm,
            sender: None,
            engine: None,
        };
        rufy.run();
    }

    pub fn run(&mut self) {
        let (sx, rx) = create_channel();
        self.sender = Some(sx.clone());

        let engine = Engine::init(sx.clone());
        self.engine = Some(engine);
        let engine_clone = self.engine.clone().unwrap();

        thread::spawn(move || {
            engine_clone.init_sdl();
        });

        loop {
            while let Ok(message) = rx.try_recv() {
                println!("GOT = {}", message);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

