use std::process::*;

pub struct processWrapper {
    command: Command,
    child: Child
}

impl processWrapper {
    
    pub fn new(command: &str) -> processWrapper {
        processWrapper { command: Command::new(command), child: None }
    }

    pub fn commence(&self) {
        self.child = self.command.spawn().expect("Command failed to start!");
    }
}