use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use bitflags::bitflags;

use crate::bus::CPUBus;
use crate::cpu::opcode::{AddressingType, Inst, Opcode, INST_TABLE};
use crate::ram::CPURam;
use crate::rom::Cartridge;

bitflags! {
    #[derive(PartialEq,Eq)]
    pub struct Flags:u8  {
        const C  = 1 << 0;
        const Z  = 1 << 1;
        const I  = 1 << 2;
        const D  = 1 << 3;
        const B  = 1 << 4;
        const U  = 1 << 5;
        const V  = 1 << 6;
        const N  = 1 << 7;
    }
}

impl Flags {
    pub fn new() -> Self {
        Flags::U | Flags::I
    }
}

#[derive(Eq)]
pub struct Regs {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub SP: u8,
    pub P: Flags,
    pub PC: u16,
}

impl PartialEq for Regs {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A && self.X == other.X
            && self.Y == other.Y && self.SP == other.SP &&
            self.P == other.P && self.PC == other.PC
    }
}

impl Default for Regs {
    fn default() -> Self {
        Self {
            A: 0,
            X: 0,
            Y: 0,
            SP: 0xfd,
            P: Flags::new(),
            PC: 0,
        }
    }
}

impl Debug for Regs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A:{:#x} X:{:#x} Y:{:#x} P:{:#x} SP:{:#x} PC:{:#x}",
            self.A, self.X, self.Y, self.P, self.SP, self.PC
        )
    }
}

pub struct CPU {
    regs: Regs,
    bus_port: Rc<RefCell<CPUBus>>,
    clock: (),
}

impl Debug for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.regs)
    }
}

enum AccessType {
    Read,
    Write,
}

// when execute inst
// assume that pc is pointer to the start of inst
// update pc after inst execute
impl CPU {
    pub fn new(port: Rc<RefCell<CPUBus>>, pc: usize) -> Self {
        let regs = Regs {
            PC: pc as u16,
            ..Regs::default()
        };
        Self {
            regs,
            bus_port: port,
            clock: (),
        }
    }
    
    #[inline]
    fn bus_port(&self) -> RefMut<CPUBus> {
        (*self.bus_port).borrow_mut()
    }
    
    #[inline]
    pub(crate) fn get_next_inst(&self) -> &'static Inst {
        let opcode = self.bus_port().read(self.regs.PC as usize);
        &INST_TABLE[&opcode]
    }
    
    #[inline]
    fn get_pc(&self) -> usize {
        self.regs.PC as usize
    }
    
    fn accumulator(&self) -> u16 {
        self.regs.A as u16
    }
    
    fn immediate(&self) -> u16 {
        (self.get_pc() + 1) as u16
    }
    
    fn zero_page(&self) -> u16 {
        let bus_port = self.bus_port();
        bus_port.read(self.get_pc() + 1) as u16
    }
    
    fn zero_page_x(&self) -> u16 {
        self.zero_page() + self.regs.X as u16
    }
    
    fn zero_page_y(&self) -> u16 {
        self.zero_page() + self.regs.Y as u16
    }
    
    fn realtive(&mut self) {
        let imm = self.bus_port().read(self.get_pc() + 1);
        self.regs.PC = (self.get_pc() as i16 + imm as i16) as u16;
    }
    
    fn absolute(&self) -> u16 {
        self.bus_port().read_u16(self.get_pc() + 1)
    }
    
    fn absolute_x(&self) -> u16 {
        self.absolute() + self.regs.X as u16
    }
    
    fn absolute_y(&self) -> u16 {
        self.absolute() + self.regs.Y as u16
    }
    
    fn indirect(&self) -> u16 {
        //handle nes indirect bug
        if (self.get_pc() + 1) & 0x00ff == 0xff {
            let port = self.bus_port();
            let mask = (self.get_pc() + 1) & 0xff00;
            (port.read(mask) as u16) << 8 | port.read(self.get_pc() + 1) as u16
        } else {
            self.bus_port().read_u16(self.get_pc() + 1) as u16
        }
    }
    
    fn indexed_indirect(&self) -> u16 {
        (self.bus_port().read((self.get_pc() + 1) as usize) + self.regs.X) as u16
    }
    
    fn indirect_indexed(&self) -> u16 {
        let port = self.bus_port();
        let address = port.read(self.get_pc() + 1) as usize;
        port.read(address) as u16 + self.regs.Y as u16
    }
    
    fn increase_pc(&mut self, val: usize) {
        self.regs.PC += val as u16;
    }
    
    fn handle_mem_read(&mut self, address_type: &AddressingType) -> u16 {
        let handle_read = |address: usize| self.bus_port().read(address) as u16;
        
        match address_type {
            AddressingType::Accumulator => handle_read(self.accumulator() as usize),
            AddressingType::Immediate => handle_read(self.immediate() as usize),
            AddressingType::ZeroPage => handle_read(self.zero_page() as usize),
            AddressingType::ZeroPageX => handle_read(self.zero_page_x() as usize),
            AddressingType::ZeroPageY => handle_read(self.zero_page_y() as usize),
            AddressingType::Absolute => self.absolute(),
            AddressingType::AbsoluteX => handle_read(self.absolute_x() as usize),
            AddressingType::AbsoluteY => handle_read(self.absolute_y() as usize),
            AddressingType::Indirect => handle_read(self.indirect() as usize),
            AddressingType::IndirectX => handle_read(self.indexed_indirect() as usize),
            AddressingType::IndirectY => handle_read(self.indirect_indexed() as usize),
            AddressingType::Implied => 0,
            AddressingType::Relative => {
                self.realtive();
                0
            }
        }
    }
    
    fn handle_mem_write(&mut self, address_type: &AddressingType, val: u8) {
        let handle_write = |address: u16, val: u8| {
            self.bus_port().write(address as usize, val);
        };
        match address_type {
            AddressingType::Accumulator => self.regs.A = val,
            AddressingType::Immediate => {}
            AddressingType::ZeroPage => handle_write(self.zero_page(), val),
            AddressingType::ZeroPageX => handle_write(self.zero_page_x(), val),
            AddressingType::ZeroPageY => handle_write(self.zero_page_y(), val),
            AddressingType::Absolute => handle_write(self.absolute(), val),
            AddressingType::AbsoluteX => handle_write(self.absolute_x(), val),
            AddressingType::AbsoluteY => handle_write(self.absolute_y(), val),
            AddressingType::Indirect => {}
            AddressingType::IndirectX => handle_write(self.indexed_indirect(), val),
            AddressingType::IndirectY => handle_write(self.indirect_indexed(), val),
            //AddressingType::Implied => {}
            //AddressingType::Relative => {}
            _ => unreachable!(),
        }
    }
    
    #[allow(unused)]
    pub fn handle_flag_update(&mut self, val: u8) {
        if val == 0 {
            self.regs.P.insert(Flags::Z);
        }
        
        if val & 0b1000_0000 != 0 {
            self.regs.P.insert(Flags::N);
        }
    }
    
    fn get_real_sp(&self) -> usize {
        self.regs.SP as usize + 0x100
    }
    
    pub fn push_stack(&mut self, val: u8) {
        self.bus_port().write(self.get_real_sp(), val);
        self.regs.SP -= 1;
    }
    
    pub fn pop_stack(&mut self) -> u8 {
        let val = self.bus_port().read(self.get_real_sp());
        self.regs.SP += 1;
        val
    }
    
    pub fn exec_once(&mut self, inst: &Inst) {
        match inst.opcode {
            Opcode::ADC => {
                let val = self.handle_mem_read(&inst.address_type);
                let res = val + self.regs.A as u16 + self.regs.P.contains(Flags::C) as u16;
                self.regs.P.set(Flags::C, res & 0x100 != 0);
                self.regs.P.set(Flags::Z, res == 0);
                self.regs.P.set(Flags::N, res & 0x80 != 0);
                self.regs.P.set(
                    Flags::V,
                    (self.regs.A ^ res as u8) & (val ^ res) as u8 & 0x80 == 0,
                );
            }
            Opcode::AND => {
                let imm = self.handle_mem_read(&inst.address_type);
                self.regs.A &= imm as u8;
                self.handle_flag_update(self.regs.A);
            }
            Opcode::ASL => {
                let imm = self.handle_mem_read(&inst.address_type);
                self.regs.P.bits |= imm as u8 & 0x80;
                self.regs.A = (self.regs.A as u16 * 2) as u8;
                self.handle_flag_update(self.regs.A);
            }
            Opcode::BCC => {
                if !self.regs.P.contains(Flags::C) {
                    self.handle_mem_read(&inst.address_type);
                }
            }
            Opcode::BCS => {
                if self.regs.P.contains(Flags::C) {
                    self.handle_mem_read(&inst.address_type);
                }
            }
            Opcode::BEQ => {
                if self.regs.P.contains(Flags::Z) {
                    self.handle_mem_read(&inst.address_type);
                }
            }
            Opcode::BIT => {
                let imm = self.handle_mem_read(&inst.address_type);
                if (imm as u8 & self.regs.A) == 0 {
                    self.regs.P.insert(Flags::Z);
                }
                
                //update overflow flag
                self.regs.P.bits |= imm as u8 & 0b0100_0000;
                //update overflow flag
                self.regs.P.bits |= imm as u8 & 0b1000_0000;
            }
            Opcode::BMI => {
                if self.regs.P.contains(Flags::N) {
                    self.handle_mem_read(&inst.address_type);
                    return;
                }
            }
            Opcode::BNE => {
                if !self.regs.P.contains(Flags::Z) {
                    self.handle_mem_read(&inst.address_type);
                    return;
                }
            }
            Opcode::BPL => {
                if !self.regs.P.contains(Flags::N) {
                    self.handle_mem_read(&inst.address_type);
                    return;
                }
            }
            Opcode::BRK => {}
            Opcode::BVC => {
                if !self.regs.P.contains(Flags::V) {
                    self.handle_mem_read(&inst.address_type);
                    return;
                }
            }
            Opcode::BVS => {
                if self.regs.P.contains(Flags::V) {
                    self.handle_mem_read(&inst.address_type);
                    return;
                }
            }
            Opcode::CLC => {
                self.regs.P.remove(Flags::C);
            }
            Opcode::CLD => {
                self.regs.P.remove(Flags::D);
            }
            Opcode::CLI => {
                self.regs.P.remove(Flags::I);
            }
            Opcode::CLV => {
                self.regs.P.remove(Flags::V);
            }
            Opcode::CMP => {
                let imm = self.handle_mem_read(&inst.address_type) as u8;
                if self.regs.A as i8 - imm as i8 >= 0 {
                    self.regs.P.insert(Flags::C);
                }
                self.handle_flag_update(self.regs.A - imm);
            }
            Opcode::CPX => {
                let imm = self.handle_mem_read(&inst.address_type) as u8;
                if self.regs.X as i8 - imm as i8 >= 0 {
                    self.regs.P.insert(Flags::C)
                };
                self.handle_flag_update(self.regs.X - imm);
            }
            Opcode::CPY => {
                let imm = self.handle_mem_read(&inst.address_type) as u8;
                if self.regs.Y as i8 - imm as i8 >= 0 {
                    self.regs.P.insert(Flags::C)
                };
                self.handle_flag_update(self.regs.Y - imm);
            }
            Opcode::DEC => {
                let imm = self.handle_mem_read(&inst.address_type);
                self.handle_flag_update(imm as u8 - 1);
                self.handle_mem_write(&inst.address_type, imm as u8 - 1);
            }
            Opcode::DEX => {
                self.regs.X -= 1;
                self.handle_flag_update(self.regs.X);
            }
            Opcode::DEY => {
                self.regs.Y -= 1;
                self.handle_flag_update(self.regs.Y);
            }
            Opcode::EOR => {
                let imm = self.handle_mem_read(&inst.address_type) as u8;
                self.regs.A ^= imm;
                self.handle_flag_update(self.regs.A);
            }
            Opcode::INC => {
                let imm = self.handle_mem_read(&inst.address_type);
                self.handle_flag_update(imm as u8 + 1);
                self.handle_mem_write(&inst.address_type, imm as u8 + 1);
            }
            Opcode::INX => {
                self.regs.X += 1;
                self.handle_flag_update(self.regs.X);
            }
            Opcode::INY => {
                self.regs.Y += 1;
                self.handle_flag_update(self.regs.Y);
            }
            Opcode::JMP => {
                let target_address = self.handle_mem_read(&inst.address_type);
                self.regs.PC = target_address;
                return;
            }
            Opcode::JSR => {
                let target_address = self.handle_mem_read(&inst.address_type) as usize;
                let ret_address = self.get_pc() as u16 + inst.inst_len as u16;
                self.push_stack((ret_address >> 8) as u8);
                self.push_stack(ret_address as u8);
                self.regs.PC = target_address as u16;
                return;
            }
            Opcode::LDA => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                self.handle_flag_update(val);
                self.regs.A = val;
            }
            Opcode::LDX => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                self.handle_flag_update(val);
                self.regs.X = val;
            }
            Opcode::LDY => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                self.handle_flag_update(val);
                self.regs.Y = val;
            }
            Opcode::LSR => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                self.regs.P.bits |= val & 0x1;
                let res = val / 2;
                self.handle_mem_write(&inst.address_type, res);
                self.handle_flag_update(res);
            }
            Opcode::NOP => {}
            Opcode::ORA => {
                let imm = self.handle_mem_read(&inst.address_type) as u8;
                self.regs.A |= imm;
                self.handle_flag_update(self.regs.A);
            }
            Opcode::PHA => {
                self.push_stack(self.regs.A);
            }
            Opcode::PHP => {
                self.push_stack(self.regs.P.bits());
            }
            Opcode::PLA => {
                self.regs.A = self.pop_stack();
                self.handle_flag_update(self.regs.A);
            }
            Opcode::PLP => self.regs.P = unsafe { Flags::from_bits_unchecked(self.pop_stack()) },
            Opcode::ROL => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                let mask = if val & 0b1000_0000 == 0 { 0x0 } else { 0x1 };
                //set negative flag
                self.regs.P.bits |= val & 0b1000_0000;
                self.handle_mem_write(&inst.address_type, (val << 1) & mask);
                if self.regs.A == 0 {
                    self.regs.P.insert(Flags::Z);
                }
            }
            Opcode::ROR => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                let mask = if val & 0b0000_0001 == 0 { 0x00 } else { 0x80 };
                //set negative flag
                self.regs.P.bits |= val & 0b0000_0001;
                self.handle_mem_write(&inst.address_type, (val >> 1) & mask);
                if self.regs.A == 0 {
                    self.regs.P.insert(Flags::Z);
                }
            }
            Opcode::RTI => {
                self.regs.P = unsafe { Flags::from_bits_unchecked(self.pop_stack()) };
                self.regs.PC = self.pop_stack() as u16;
                self.regs.PC |= (self.pop_stack() as u16) << 8;
                return;
            }
            Opcode::RTS => {
                self.regs.PC = self.pop_stack() as u16;
                self.regs.PC |= (self.pop_stack() as u16) << 8;
                return;
            }
            Opcode::SBC => {
                let val = self.handle_mem_read(&inst.address_type) as u8;
                let res = self.regs.A - val - !self.regs.P.contains(Flags::C) as u8;
                
                self.regs.P.set(Flags::C, res as u16 & 0x100 == 0);
                self.regs
                    .P
                    .set(Flags::V, (self.regs.A ^ res) & (!val ^ res) & 0x80 != 0);
                self.regs.P.set(Flags::Z, res == 0);
                self.regs.A = res;
            }
            Opcode::SEC => {
                self.regs.P.insert(Flags::C);
            }
            Opcode::SED => {
                self.regs.P.insert(Flags::D);
            }
            Opcode::SEI => self.regs.P.insert(Flags::I),
            Opcode::STA => {
                self.handle_mem_write(&inst.address_type, self.regs.A);
            }
            Opcode::STX => {
                self.handle_mem_write(&inst.address_type, self.regs.X);
            }
            Opcode::STY => {
                self.handle_mem_write(&inst.address_type, self.regs.Y);
            }
            Opcode::TAX => {
                self.regs.X = self.regs.A;
                self.handle_flag_update(self.regs.X);
            }
            Opcode::TAY => {
                self.regs.Y = self.regs.A;
                self.handle_flag_update(self.regs.Y);
            }
            Opcode::TSX => {
                self.regs.X = self.regs.SP;
                self.handle_flag_update(self.regs.SP);
            }
            Opcode::TXA => {
                self.regs.A = self.regs.X;
                self.handle_flag_update(self.regs.A);
            }
            Opcode::TXS => {
                self.regs.SP = self.regs.X;
                self.handle_flag_update(self.regs.SP);
            }
            Opcode::TYA => {
                self.regs.A = self.regs.Y;
                self.handle_flag_update(self.regs.A);
            }
        }
        self.increase_pc(inst.inst_len)
    }
}

#[test]
fn cpu_test() {
    
    //let cart = Cartridge::new("./test/nestest.nes");
    //let ram = CPURam::default();
    //let bus = CPUBus::connect(Rc::new(RefCell::new(cart)), Rc::new(RefCell::new(ram)));
    //let mut cpu = CPU::new(Rc::new(RefCell::new(bus)), 0xc000);
    //
    //for _ in 0..20 {
    //    let inst = cpu.get_next_inst();
    //    cpu.exec_once(inst);
    //    println!("{:#?}", cpu);
    //}
}
