pub use NROM::NRom;

mod MMC3;
mod NROM;

pub const NROM: usize = 0;
pub const MMC3: usize = 4;

pub enum AccessArea {
    ChrRom,
    PrgRom,
    
}

pub trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
    fn read_word(&self, address: u16) -> u16;
    fn write_word(&mut self, address: u16, data: u16);
    fn data_ref(&self, address: u16) -> &u8;
    fn data_ref_mut(&mut self, address: u16) -> &mut u8;
}

/// return (high byte,low byte)
pub fn split_word(data: u16) -> (u8, u8) {
    (((data & 0xff00) >> 8) as u8, (data & 0x00ff) as u8)
}
