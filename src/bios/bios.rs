use std::path::Path;
use std::fs::File;
use std::io::{Read,Error,ErrorKind};
use crate::prelude::*;

pub const BIOS: Range = Range(0xbfc00000,512*1024);


pub struct Bios{
    data: Vec<u8>
}


impl Bios{

pub fn new(path: &Path) -> Result<Bios,Error>{

let file = File::open(path)?;


let mut data = Vec::new();

file.take(BIOS_SIZE).read_to_end(&mut data)?;

if data.len() == BIOS_SIZE as usize{
Ok(Self{
    data
})
}else{
    Err(Error::new(ErrorKind::InvalidInput, "invalid bios size"))
}
}



pub fn load32(&self, offset: u32) -> u32{
    let offset = offset as usize;
    
    let b0 = self.data[offset + 0] as u32;
    let b1 = self.data[offset + 1] as u32;
    let b2 = self.data[offset + 2] as u32;
    let b3 = self.data[offset + 3] as u32;


    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}


}// impl Bios
 



pub struct Interconnect{
    bios: Bios
}



impl Interconnect{

pub fn new(bios: Bios) -> Self{
    Self{
        bios
    }
}

pub fn load32(&self, addr: u32) -> u32{
    if let Some(value) = BIOS.contains(addr){
        return self.bios.load32(value)
    }
    panic!("unhandled fetch32 at address {:08x}",addr);
}

}//impl Interconnect




pub struct Range(u32,u32);


impl Range{
    
    pub fn contains(self,addr: u32) -> Option<u32>{
        let Range(start,length) = self;

        if addr>= start && addr < start + length{
            Some(addr - start)
        }else{
            None
        }

    }


} // impl range
