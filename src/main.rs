#![feature(associated_type_defaults)]
#![feature(exclusive_range_pattern)]
#![allow(unused)]

use crate::bus::CPUBus;
use crate::cpu::CPU;
use crate::ram::CPURam;
use crate::rom::Cartridge;
use std::cell::RefCell;
use std::env::current_dir;
use std::rc::Rc;

mod bus;
mod cpu;
mod emulator;
mod error;
//mod log;
mod ram;
mod rom;
mod test;

fn main() {
    let cart = Cartridge::new("./test/nestest.nes");
    let ram = CPURam::default();
    let bus = CPUBus::connect(Rc::new(RefCell::new(cart)), Rc::new(RefCell::new(ram)));
    let mut cpu = CPU::new(Rc::new(RefCell::new(bus)), 0xc000);
    
    for _ in 0..20 {
        println!("{:#?}", cpu);
        let inst = cpu.get_next_inst();
        cpu.exec_once(inst);
    }
}
