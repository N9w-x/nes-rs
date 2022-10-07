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
mod trace;
mod ppu;

fn main() {
    trace::trace("./test/nestest.log");
}
