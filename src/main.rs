use memory_map::cpu::CPU;
use memory_map::memory::Parm;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let mut x_1_cpu = CPU::new();
    // Allocate space for data
    x_1_cpu.malloc(Parm::READ | Parm::WRITE);

    // Allocate space for Instructions
    x_1_cpu.malloc(Parm::RWE);

    // Load data into memory
    x_1_cpu.write_byte_into_region(0x1, 0, 0)?;
    x_1_cpu.write_byte_into_region(0x1, 0, 1)?;
    x_1_cpu.write_byte_into_region(0x4, 0, 3)?;

    // Load Instructions into memory
    x_1_cpu.write_byte_into_region(0x50, 1, 0)?; // FETCH
    x_1_cpu.write_byte_into_region(0x51, 1, 1)?; // FETCH
    x_1_cpu.write_byte_into_region(0x10, 1, 2)?; // ADD
    x_1_cpu.write_byte_into_region(0x60, 1, 3)?; // STORE
    x_1_cpu.write_byte_into_region(0xFA, 1, 4)?; // HALT

    // Run processor
    x_1_cpu.execute(1)?;

    println!("{:#X?}", x_1_cpu);

    Ok(())
}
