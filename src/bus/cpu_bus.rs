use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::ram::CPURam;
use crate::rom::Cartridge;

pub struct CPUBus {
    cartridge_port: Rc<RefCell<Cartridge>>,
    ram_port: Rc<RefCell<CPURam>>,
}

impl CPUBus {
    pub fn connect(cartridge_port: Rc<RefCell<Cartridge>>, ram_port: Rc<RefCell<CPURam>>) -> Self {
        Self {
            cartridge_port,
            ram_port,
        }
    }

    #[inline]
    fn cartridge_port(&self) -> RefMut<Cartridge> {
        (*self.cartridge_port).borrow_mut()
    }

    #[inline]
    fn ram_port(&self) -> RefMut<CPURam> {
        (*self.ram_port).borrow_mut()
    }

    pub fn read(&self, address: usize) -> u8 {
        match address {
            0x0..0x6000 => self.ram_port().read(address),
            0x6000..0x100000 => self.cartridge_port().read(address as u16),
            _ => unreachable!(),
        }
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        match address {
            0x0..0x6000 => {
                if address & 0xff == 0xff {
                    self.read(address) as u16 | (self.read(address & 0xff00) as u16) << 8
                } else {
                    self.read(address) as u16 | (self.read(address + 1) as u16) << 8
                }
            }
            0x6000..0x100000 => self.read(address) as u16 | (self.read(address + 1) as u16) << 8,
            _ => unreachable!(),
        }
    }

    pub fn write(&self, address: usize, val: u8) {
        match address {
            0x0..0x6000 => self.ram_port().write(address, val),
            0x6000..0x100000 => self.cartridge_port().write(address as u16, val),
            _ => unimplemented!(),
        }
    }
}
