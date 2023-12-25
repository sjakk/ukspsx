mod cpu;
mod bios;

use crate::prelude::*;
use std::path::Path;


mod prelude{
pub use crate::cpu::*;
pub use crate::bios::*;
pub const BIOS_SIZE: u64 = 512 * 1024;
}



fn main() {

let bios = Bios::new(&Path::new("roms/SCPH1001.BIN")).unwrap();
let inter = Interconnect::new(bios);
let mut cpu = Processor::new(inter);

loop{
    cpu.run_next_instruction();
}


}
