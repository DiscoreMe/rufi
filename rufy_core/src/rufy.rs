use super::lua::VM;
use crate::error::{
    Result,
    Error
};

use crate::channel::{
    Sender,
    Receiver,
    create_channel,
};


pub struct Rufy {
    vm: VM,
    recipient: Option<Receiver>,
}

impl Rufy {
    pub fn new(path: String) -> Result<Rufy, Error> {
        let vm = VM::new(path.clone());
        vm.init()?;

        Ok(Rufy {
            vm,
            recipient: None,
        })
    }

    pub fn run(&self) -> Result<(), Error>{
        self.vm.run()?;
        Ok(())
    }

    pub fn init_signal(&mut self) -> Sender {
        let (sx, mut rx) = create_channel();
        self.recipient = Some(rx);
        sx
    }

    pub async fn listen_signal(self) {
        match self.recipient {
            None => println!("listen signal is none"),
            Some(mut rc) => {
                tokio::spawn(async move {
                    while let Some(msg) = rc.recv().await {
                        println!("GOT = {}", msg);
                    }
                });
            },
        }
    }
}

