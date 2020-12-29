use super::lua::VM;
use crate::result::Result;

pub struct Rufy {
    vm: VM,
}

impl Rufy {
    pub fn new(path: String) -> Result<Rufy> {
        let vm = VM::new(path.clone());
        vm.init()?;

        Ok(Rufy {
            vm,
        })
    }

    pub fn run(&self) {
        match self.vm.run() {
            Err(e) => println!("Error: {:?}", e),
            _ => {}
        }
    }
}

