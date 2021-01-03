use std::thread;

use super::lua::VM;
use crate::channel::{
    Sender,
    create_channel,
};
use crate::engine::Engine;

pub struct Rufy {
    vm: VM,
    sender: Option<Sender>,
}

impl Rufy {
    pub fn init(path: String) {
        let vm = VM::new(path.clone());
        vm.init().unwrap();
        vm.run().unwrap();

        let mut rufy = Rufy {
            vm,
            sender: None,
        };
        rufy.run();
    }

    pub fn run(&mut self) {
        let (sx, rx) = create_channel();
        self.sender = Some(sx.clone());

        thread::spawn(move || {
            let mut engine = Engine::init();
            engine.init_sdl();
        });

        sx.send("Init signal recv".parse().unwrap()).unwrap();

        loop {
            while let Ok(message) = rx.try_recv() {
                println!("GOT = {}", message);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

