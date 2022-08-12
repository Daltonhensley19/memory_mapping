use bitflags::bitflags;
use std::cell::RefCell;
use thiserror::Error;

#[derive(Debug)]
pub struct Dog {
    pub name: RefCell<i32>,
}

#[derive(Error, Debug)]
pub enum MemError {
    #[error("You are accessing memory which needs to be set to {0}")]
    MissingParms(String),
}

bitflags! {
    pub struct Parm: u8 {
        const READ = 1;
        const WRITE = 2;
        const EXEC = 4;
        const RWE = Self::READ.bits | Self::WRITE.bits | Self::EXEC.bits;
    }
}

#[derive(Debug, Default, Clone)]
pub struct MemoryMap {
    pub memory_region: Vec<([u8; 255], Parm)>,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            memory_region: Default::default(),
        }
    }
}
