use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::ram::VRam;
use crate::rom::Cartridge;

pub struct PPUBus {
    cartridge_port: Rc<RefCell<Cartridge>>,
    vram_port: Rc<RefCell<VRam>>,
}

impl PPUBus {
    pub fn connect(cartridge_port: Rc<RefCell<Cartridge>>, vram_port: Rc<RefCell<VRam>>) -> Self {
        Self {
            cartridge_port,
            vram_port,
        }
    }
    
    #[inline]
    fn cartridge_port(&self) -> RefMut<'_, Cartridge> {
        //self.cartridge_port
        (*self.cartridge_port).borrow_mut()
    }
    
    #[inline]
    fn vram_port(&self) -> RefMut<VRam> {
        (*self.vram_port).borrow_mut()
    }
    
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0x0..0x2000 => self.cartridge_port().read(address as u16),
            0x2000..0x4000 => self.vram_port().read(address),
            _ => unimplemented!(),
        }
    }
    
    pub fn write(&self, address: usize, val: u8) {
        match address {
            0x0..0x2000 => self.cartridge_port().write(address as u16, val),
            0x2000..0x4000 => self.vram_port().write(address, val),
            _ => unimplemented!(),
        }
    }
}
