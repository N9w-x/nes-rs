pub struct CPURam {
    ram: Vec<u8>,
    io_registers: Vec<u8>,
}

impl Default for CPURam {
    fn default() -> Self {
        Self {
            ram: vec![0; 0x2000],
            io_registers: vec![0; 0x2020],
        }
    }
}

impl CPURam {
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0x0..=0x1fff => self.ram[address],
            0x2000..=0x401f => self.io_registers[address - 0x2000],
            _ => unimplemented!(),
        }
    }
    
    pub fn write(&mut self, address: usize, val: u8) {
        match address {
            0x0..=0x1fff => self.ram[address] = val,
            0x2000..=0x401f => self.io_registers[address - 0x2000] = val,
            _ => unimplemented!(),
        }
    }
}

pub struct VRam {
    name_tables: Vec<u8>,
    palettes: Vec<u8>,
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            name_tables: vec![0; 0x1f00],
            palettes: vec![0; 0x0100],
        }
    }
}

impl VRam {
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0x2000..0x3f00 => self.name_tables[address],
            0x3f00..0x4000 => self.palettes[address],
            _ => unimplemented!(),
        }
    }
    
    pub fn write(&mut self, address: usize, val: u8) {
        match address {
            0x2000..0x3f00 => self.name_tables[address] = val,
            0x3f00..0x4000 => self.palettes[address] = val,
            _ => unimplemented!(),
        }
    }
}
