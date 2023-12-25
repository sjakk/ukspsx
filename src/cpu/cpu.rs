use crate::prelude::*;

pub struct Processor{

pc: u32,

inter: Interconnect,

}



impl Processor{

    pub fn new(inter: Interconnect) -> Self{
        Self{
            pc: 0xbfc00000,// look at playstation memory map bios section
            inter,
        }
    }

fn load32(&self, addr:u32) -> u32{
    self.inter.load32(addr)
}



pub fn run_next_instruction(&mut self) {
    let pc = self.pc;
    let instruction = self.load32(pc);
    self.pc = pc.wrapping_add(4);
    self.decode_and_execute(instruction);
}

fn decode_and_execute(&mut self, instruction: u32){
    panic!("unhandled instruction - {:08x}",instruction);
}



} // impl Processor
