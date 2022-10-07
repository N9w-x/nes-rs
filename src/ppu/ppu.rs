use crate::ppu::controller::Controller;
use crate::ppu::mask::Mask;
use crate::ppu::status::Status;
use crate::ram::VRam;
use bitflags::bitflags;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    vram_port: Rc<RefCell<VRam>>,
    controller: Controller,
    mask: Mask,
    status: Status,
}

impl PPU {
    pub fn write_controller(&mut self, val: u8) {
        self.controller.update(val)
    }

    pub fn write_mask(&mut self, val: u8) {
        self.mask.update(val)
    }

    pub fn read_status(&self) -> u8 {
        self.status.bits()
    }
}

#[derive(Debug, Clone)]
pub enum IORegisters {
    Controller = 0x2000,
    Mask = 0x2001,
    Status = 0x2002,
    OamAddress = 0x2003,
    OamData = 0x2004,
    Scroll = 0x2005,
    Address = 0x2006,
    Data = 0x2007,
    OamDma = 0x4014,
}

impl From<usize> for IORegisters {
    fn from(address: usize) -> Self {
        match address {
            0x2000 => Self::Controller,
            0x2001 => Self::Mask,
            0x2002 => Self::Status,
            0x2003 => Self::OamAddress,
            0x2004 => Self::OamData,
            0x2005 => Self::Scroll,
            0x2006 => Self::Address,
            0x2007 => Self::Data,
            0x4014 => Self::OamDma,
            _ => unreachable!(),
        }
    }
}
