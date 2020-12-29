use std::io::Result;

use super::lua::VM;

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
}

