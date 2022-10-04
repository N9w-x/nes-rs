use crate::rom::mapper::{Mapper, split_word};

pub struct NRom {
    prg_rom: Vec<u8>,
    chr: Vec<u8>,
    prg_ram: [u8; 1024 * 8],
    is_mirroring: bool,
}

impl NRom {
    pub fn new(mut prg_rom: Vec<u8>, mut chr_rom: Vec<u8>, is_mirroring: bool) -> Self {
        if chr_rom.is_empty() {
            chr_rom.resize(0x2000, 0);
        }
        
        if prg_rom.len() <= 1024 * 16 {
            let len = prg_rom.len();
            let bak = prg_rom.clone();
            prg_rom.resize(32 * 1024, 0);
            prg_rom[16 * 1024..16 * 1024 + len].copy_from_slice(&bak);
        }
        
        Self {
            prg_rom,
            chr: chr_rom,
            prg_ram: [0; 1024 * 8],
            is_mirroring,
        }
    }
    
    fn parse_address(&self, mut address: u16) -> u16 {
        address &= 0xffff;
        
        if address <= 0x2000 {
            address
        } else if self.is_mirroring {
            (address & 0b1011_1111_1111_1111) - 0x8000
        } else {
            address - 0x8000
        }
    }
}

impl Mapper for NRom {
    fn read(&self, address: u16) -> u8 {
        *self.data_ref(address)
    }
    
    fn write(&mut self, address: u16, data: u8) {
        *self.data_ref_mut(address) = data;
    }
    
    fn read_word(&self, address: u16) -> u16 {
        (self.read(address + 1) as u16) << 8 | (self.read(address) as u16)
    }
    
    fn write_word(&mut self, address: u16, data: u16) {
        let (high, low) = split_word(data);
        self.write(address, low);
        self.write(address + 1, high);
    }
    
    fn data_ref(&self, address: u16) -> &u8 {
        if address <= 0x2000 {
            self.chr.get(address as usize)
        } else if address >= 0x8000 {
            self.prg_rom.get(self.parse_address(address) as usize)
        } else if address >= 0x6000 {
            self.prg_ram.get((address - 0x6000) as usize)
        } else {
            panic!("invalid address {:#x}", address)
        }
            .unwrap()
    }
    
    fn data_ref_mut(&mut self, address: u16) -> &mut u8 {
        if address <= 0x2000 {
            self.chr.get_mut(address as usize)
        } else if address >= 0x8000 {
            let address = self.parse_address(address) as usize;
            self.prg_rom.get_mut(address)
        } else if address >= 0x6000 {
            self.prg_ram.get_mut((address - 0x6000) as usize)
        } else {
            panic!("invalid address {:#x}", address)
        }
            .unwrap()
    }
}
