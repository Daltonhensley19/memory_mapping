#![allow(dead_code)]

use crate::memory::MemError;
use crate::memory::MemoryMap;
use crate::memory::Parm;

#[derive(Debug, Clone, Copy, Default)]
struct Registers {
    rax: u8,
    rbx: u8,
    rcx: u8,
    rdx: u8,
    r1: u8,
    r2: u8,
    r3: u8,
    r4: u8,
    pc: u8,
    ir: u8,
}

#[derive(Clone, Copy, Debug)]
enum ModeKind {
    Arth,
    IO,
    Halt,
}

#[derive(Clone, Debug)]
pub struct CPU {
    pub memory: MemoryMap,
    regs: Registers,
    mode: ModeKind,
}

impl CPU {
    pub fn new() -> Self {
        let regs = Default::default();
        let memory = MemoryMap::new();
        let mode = ModeKind::Halt;

        CPU { regs, memory, mode }
    }

    pub fn malloc(&mut self, parms: Parm) {
        let memory_area = [0u8; 255];

        self.memory.memory_region.push((memory_area, parms));
    }

    pub fn dealloc_region(&mut self, region_index: usize) {
        self.memory.memory_region.remove(region_index);
    }

    pub fn dealloc_all(&mut self) {
        self.memory.memory_region.clear();
    }

    pub fn chmod(&mut self, region_index: usize, parms: Parm) {
        self.memory.memory_region[region_index].1 = parms;
    }

    pub fn read_region(&self, region_index: usize) -> Result<(), MemError> {
        if Parm::contains(&self.memory.memory_region[region_index].1, Parm::READ) {
            println!(
                "Reading from region {region_index}: {:?}",
                &self.memory.memory_region[region_index].0
            );
        } else {
            return Err(MemError::MissingParms("READ".to_string()));
        }

        Ok(())
    }

    pub fn write_into_region(
        &mut self,
        data_buffer: [u8; 255],
        region_index: usize,
    ) -> Result<(), MemError> {
        if Parm::contains(&self.memory.memory_region[region_index].1, Parm::WRITE) {
            for idx in 0..data_buffer.len() {
                self.memory.memory_region[region_index].0[idx] = data_buffer[idx];
            }
        } else {
            return Err(MemError::MissingParms("WRITE".to_string()));
        }

        Ok(())
    }

    pub fn write_byte_into_region(
        &mut self,
        byte: u8,
        region_index: usize,
        byte_index: usize,
    ) -> Result<(), MemError> {
        if Parm::contains(&self.memory.memory_region[region_index].1, Parm::WRITE) {
            self.memory.memory_region[region_index].0[byte_index] = byte;
        } else {
            return Err(MemError::MissingParms("WRITE".to_string()));
        }

        Ok(())
    }

    pub fn execute(&mut self, region_index: usize) -> Result<(), MemError> {
        if Parm::contains(&self.memory.memory_region[region_index].1, Parm::EXEC) {
            self.regs.pc = 0;
            self.regs.ir = self.memory.memory_region[1].0[0];
            loop {
                match self.regs.ir {
                    0x50 => {
                        self.regs.rax = self.memory.memory_region[0].0[self.regs.pc as usize];
                        self.regs.pc += 1;
                    }
                    0x51 => {
                        self.regs.rbx = self.memory.memory_region[0].0[self.regs.pc as usize];
                        self.regs.pc += 1;
                    }
                    0x10 => {
                        self.regs.rax += self.regs.rbx;
                        self.regs.pc += 1;
                    }
                    0x60 => {
                        let index = self.memory.memory_region[0].0[self.regs.pc as usize] as usize;

                        dbg!(&self.regs.pc);
                        dbg!(&index);
                        self.memory.memory_region[0].0[index] = self.regs.rax;
                        self.regs.pc += 1;
                    }
                    0xFF => {
                        println!("HALTING CPU!!!");
                        break;
                    }
                    0x00 => {
                        self.regs.pc += 1;
                    }
                    instr => panic!("PANIC!!! UNKNOWN INSTR: {instr:#X}"),
                }

                self.regs.ir = self.memory.memory_region[1].0[self.regs.pc as usize];
            }
        } else {
            return Err(MemError::MissingParms("EXEC".to_string()));
        }

        Ok(())
    }
}
