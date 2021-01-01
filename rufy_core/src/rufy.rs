use super::lua::VM;
use crate::error::{
    Result,
    Error
};

pub struct Rufy {
    vm: VM,
}

impl Rufy {
    pub fn new(path: String) -> Result<Rufy, Error> {
        let vm = VM::new(path.clone());
        vm.init()?;

        Ok(Rufy {
            vm,
        })
    }

    pub fn run(&self) -> Result<(), Error>{
        self.vm.run()?;
        Ok(())
    }
}

