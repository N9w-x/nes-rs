use std::collections::HashMap;

use lazy_static::lazy_static;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    //invalid,
    LAX,
    SAX,
    DCP,
    ISC,
    SLO,
    RLA,
    SRE,
    RRA,
}

#[derive(Debug, Clone)]
pub enum AddressingType {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
    Relative,
}

#[allow(unused)]
pub struct Inst {
    pub address_type: AddressingType,
    pub opcode: Opcode,
    pub inst_len: usize,
    pub cycles: usize,
    pub page_crossed_add: bool,
}

impl Inst {
    pub fn new(
        address_type: AddressingType,
        opcode: Opcode,
        inst_len: usize,
        cycles: usize,
        page_crossed_add: bool,
    ) -> Self {
        Self {
            address_type,
            opcode,
            inst_len,
            cycles,
            page_crossed_add,
        }
    }
}

lazy_static! {
    pub static ref INST_TABLE: HashMap<u8, Inst> = { generate_inst_table() };
}

fn generate_inst_table() -> HashMap<u8, Inst> {
    let mut hash = HashMap::new();
    hash.insert(
        0x00,
        Inst::new(AddressingType::Implied, Opcode::BRK, 2, 7, false),
    );
    hash.insert(
        0x01,
        Inst::new(AddressingType::IndirectX, Opcode::ORA, 2, 6, false),
    );
    hash.insert(
        0x05,
        Inst::new(AddressingType::ZeroPage, Opcode::ORA, 2, 3, false),
    );
    hash.insert(
        0x06,
        Inst::new(AddressingType::ZeroPage, Opcode::ASL, 2, 5, false),
    );
    hash.insert(
        0x08,
        Inst::new(AddressingType::Implied, Opcode::PHP, 1, 3, false),
    );
    hash.insert(
        0x09,
        Inst::new(AddressingType::Immediate, Opcode::ORA, 2, 2, false),
    );
    hash.insert(
        0x0a,
        Inst::new(AddressingType::Accumulator, Opcode::ASL, 1, 2, false),
    );
    hash.insert(
        0x0d,
        Inst::new(AddressingType::Absolute, Opcode::ORA, 3, 4, false),
    );
    hash.insert(
        0x0e,
        Inst::new(AddressingType::Absolute, Opcode::ASL, 3, 6, false),
    );
    hash.insert(
        0x10,
        Inst::new(AddressingType::Relative, Opcode::BPL, 2, 2, true),
    );
    hash.insert(
        0x11,
        Inst::new(AddressingType::IndirectY, Opcode::ORA, 2, 5, true),
    );
    hash.insert(
        0x15,
        Inst::new(AddressingType::ZeroPageX, Opcode::ORA, 2, 4, false),
    );
    hash.insert(
        0x16,
        Inst::new(AddressingType::ZeroPageX, Opcode::ASL, 2, 6, false),
    );
    hash.insert(
        0x18,
        Inst::new(AddressingType::Implied, Opcode::CLC, 1, 2, false),
    );
    hash.insert(
        0x19,
        Inst::new(AddressingType::AbsoluteY, Opcode::ORA, 3, 4, true),
    );
    hash.insert(
        0x1d,
        Inst::new(AddressingType::AbsoluteX, Opcode::ORA, 3, 4, true),
    );
    hash.insert(
        0x1e,
        Inst::new(AddressingType::AbsoluteX, Opcode::ASL, 3, 7, false),
    );
    hash.insert(
        0x20,
        Inst::new(AddressingType::Absolute, Opcode::JSR, 3, 6, false),
    );
    hash.insert(
        0x21,
        Inst::new(AddressingType::IndirectX, Opcode::AND, 2, 6, false),
    );
    hash.insert(
        0x24,
        Inst::new(AddressingType::ZeroPage, Opcode::BIT, 2, 3, false),
    );
    hash.insert(
        0x25,
        Inst::new(AddressingType::ZeroPage, Opcode::AND, 2, 3, false),
    );
    hash.insert(
        0x26,
        Inst::new(AddressingType::ZeroPage, Opcode::ROL, 2, 5, false),
    );
    hash.insert(
        0x28,
        Inst::new(AddressingType::Implied, Opcode::PLP, 1, 4, false),
    );
    hash.insert(
        0x29,
        Inst::new(AddressingType::Immediate, Opcode::AND, 2, 2, false),
    );
    hash.insert(
        0x2a,
        Inst::new(AddressingType::Accumulator, Opcode::ROL, 1, 2, false),
    );
    hash.insert(
        0x2c,
        Inst::new(AddressingType::Absolute, Opcode::BIT, 3, 4, false),
    );
    hash.insert(
        0x2d,
        Inst::new(AddressingType::Absolute, Opcode::AND, 3, 4, false),
    );
    hash.insert(
        0x2e,
        Inst::new(AddressingType::Absolute, Opcode::ROL, 3, 6, false),
    );
    hash.insert(
        0x30,
        Inst::new(AddressingType::Relative, Opcode::BMI, 2, 2, true),
    );
    hash.insert(
        0x31,
        Inst::new(AddressingType::IndirectY, Opcode::AND, 2, 5, true),
    );
    hash.insert(
        0x35,
        Inst::new(AddressingType::ZeroPageX, Opcode::AND, 2, 4, false),
    );
    hash.insert(
        0x36,
        Inst::new(AddressingType::ZeroPageX, Opcode::ROL, 2, 6, false),
    );
    hash.insert(
        0x38,
        Inst::new(AddressingType::Implied, Opcode::SEC, 1, 2, false),
    );
    hash.insert(
        0x39,
        Inst::new(AddressingType::AbsoluteY, Opcode::AND, 3, 4, true),
    );
    hash.insert(
        0x3d,
        Inst::new(AddressingType::AbsoluteX, Opcode::AND, 3, 4, true),
    );
    hash.insert(
        0x3e,
        Inst::new(AddressingType::AbsoluteX, Opcode::ROL, 3, 7, false),
    );
    hash.insert(
        0x40,
        Inst::new(AddressingType::Implied, Opcode::RTI, 1, 6, false),
    );
    hash.insert(
        0x41,
        Inst::new(AddressingType::IndirectX, Opcode::EOR, 2, 6, false),
    );
    hash.insert(
        0x45,
        Inst::new(AddressingType::ZeroPage, Opcode::EOR, 2, 3, false),
    );
    hash.insert(
        0x46,
        Inst::new(AddressingType::ZeroPage, Opcode::LSR, 2, 5, false),
    );
    hash.insert(
        0x48,
        Inst::new(AddressingType::Implied, Opcode::PHA, 1, 3, false),
    );
    hash.insert(
        0x49,
        Inst::new(AddressingType::Immediate, Opcode::EOR, 2, 2, false),
    );
    hash.insert(
        0x4a,
        Inst::new(AddressingType::Accumulator, Opcode::LSR, 1, 2, false),
    );
    hash.insert(
        0x4c,
        Inst::new(AddressingType::Absolute, Opcode::JMP, 3, 3, false),
    );
    hash.insert(
        0x4d,
        Inst::new(AddressingType::Absolute, Opcode::EOR, 3, 4, false),
    );
    hash.insert(
        0x4e,
        Inst::new(AddressingType::Absolute, Opcode::LSR, 3, 6, false),
    );
    hash.insert(
        0x50,
        Inst::new(AddressingType::Relative, Opcode::BVC, 2, 2, true),
    );
    hash.insert(
        0x51,
        Inst::new(AddressingType::IndirectY, Opcode::EOR, 2, 5, true),
    );
    hash.insert(
        0x55,
        Inst::new(AddressingType::ZeroPageX, Opcode::EOR, 2, 4, false),
    );
    hash.insert(
        0x56,
        Inst::new(AddressingType::ZeroPageX, Opcode::LSR, 2, 6, false),
    );
    hash.insert(
        0x58,
        Inst::new(AddressingType::Implied, Opcode::CLI, 1, 2, false),
    );
    hash.insert(
        0x59,
        Inst::new(AddressingType::AbsoluteY, Opcode::EOR, 3, 4, true),
    );
    hash.insert(
        0x5d,
        Inst::new(AddressingType::AbsoluteX, Opcode::EOR, 3, 4, true),
    );
    hash.insert(
        0x5e,
        Inst::new(AddressingType::AbsoluteX, Opcode::LSR, 3, 7, false),
    );
    hash.insert(
        0x60,
        Inst::new(AddressingType::Implied, Opcode::RTS, 1, 6, false),
    );
    hash.insert(
        0x61,
        Inst::new(AddressingType::IndirectX, Opcode::ADC, 2, 6, false),
    );
    hash.insert(
        0x65,
        Inst::new(AddressingType::ZeroPage, Opcode::ADC, 2, 3, false),
    );
    hash.insert(
        0x66,
        Inst::new(AddressingType::ZeroPage, Opcode::ROR, 2, 5, false),
    );
    hash.insert(
        0x68,
        Inst::new(AddressingType::Implied, Opcode::PLA, 1, 4, false),
    );
    hash.insert(
        0x69,
        Inst::new(AddressingType::Immediate, Opcode::ADC, 2, 2, false),
    );
    hash.insert(
        0x6a,
        Inst::new(AddressingType::Accumulator, Opcode::ROR, 1, 2, false),
    );
    hash.insert(
        0x6c,
        Inst::new(AddressingType::Indirect, Opcode::JMP, 3, 5, false),
    );
    hash.insert(
        0x6d,
        Inst::new(AddressingType::Absolute, Opcode::ADC, 3, 4, false),
    );
    hash.insert(
        0x6e,
        Inst::new(AddressingType::Absolute, Opcode::ROR, 3, 6, false),
    );
    hash.insert(
        0x70,
        Inst::new(AddressingType::Relative, Opcode::BVS, 2, 2, true),
    );
    hash.insert(
        0x71,
        Inst::new(AddressingType::IndirectY, Opcode::ADC, 2, 5, true),
    );
    hash.insert(
        0x75,
        Inst::new(AddressingType::ZeroPageX, Opcode::ADC, 2, 4, false),
    );
    hash.insert(
        0x76,
        Inst::new(AddressingType::ZeroPageX, Opcode::ROR, 2, 6, false),
    );
    hash.insert(
        0x78,
        Inst::new(AddressingType::Implied, Opcode::SEI, 1, 2, false),
    );
    hash.insert(
        0x79,
        Inst::new(AddressingType::AbsoluteY, Opcode::ADC, 3, 4, true),
    );
    hash.insert(
        0x7d,
        Inst::new(AddressingType::AbsoluteX, Opcode::ADC, 3, 4, true),
    );
    hash.insert(
        0x7e,
        Inst::new(AddressingType::AbsoluteX, Opcode::ROR, 3, 7, false),
    );
    hash.insert(
        0x81,
        Inst::new(AddressingType::IndirectX, Opcode::STA, 2, 6, false),
    );
    hash.insert(
        0x84,
        Inst::new(AddressingType::ZeroPage, Opcode::STY, 2, 3, false),
    );
    hash.insert(
        0x85,
        Inst::new(AddressingType::ZeroPage, Opcode::STA, 2, 3, false),
    );
    hash.insert(
        0x86,
        Inst::new(AddressingType::ZeroPage, Opcode::STX, 2, 3, false),
    );
    hash.insert(
        0x88,
        Inst::new(AddressingType::Implied, Opcode::DEY, 1, 2, false),
    );
    hash.insert(
        0x8a,
        Inst::new(AddressingType::Implied, Opcode::TXA, 1, 2, false),
    );
    hash.insert(
        0x8c,
        Inst::new(AddressingType::Absolute, Opcode::STY, 3, 4, false),
    );
    hash.insert(
        0x8d,
        Inst::new(AddressingType::Absolute, Opcode::STA, 3, 4, false),
    );
    hash.insert(
        0x8e,
        Inst::new(AddressingType::Absolute, Opcode::STX, 3, 4, false),
    );
    hash.insert(
        0x90,
        Inst::new(AddressingType::Relative, Opcode::BCC, 2, 2, true),
    );
    hash.insert(
        0x91,
        Inst::new(AddressingType::IndirectY, Opcode::STA, 2, 6, false),
    );
    hash.insert(
        0x94,
        Inst::new(AddressingType::ZeroPageX, Opcode::STY, 2, 4, false),
    );
    hash.insert(
        0x95,
        Inst::new(AddressingType::ZeroPageX, Opcode::STA, 2, 4, false),
    );
    hash.insert(
        0x96,
        Inst::new(AddressingType::ZeroPageY, Opcode::STX, 2, 4, false),
    );
    hash.insert(
        0x98,
        Inst::new(AddressingType::Implied, Opcode::TYA, 1, 2, false),
    );
    hash.insert(
        0x99,
        Inst::new(AddressingType::AbsoluteY, Opcode::STA, 3, 5, false),
    );
    hash.insert(
        0x9a,
        Inst::new(AddressingType::Implied, Opcode::TXS, 1, 2, false),
    );
    hash.insert(
        0x9d,
        Inst::new(AddressingType::AbsoluteX, Opcode::STA, 3, 5, false),
    );
    hash.insert(
        0xa0,
        Inst::new(AddressingType::Immediate, Opcode::LDY, 2, 2, false),
    );
    hash.insert(
        0xa1,
        Inst::new(AddressingType::IndirectX, Opcode::LDA, 2, 6, false),
    );
    hash.insert(
        0xa2,
        Inst::new(AddressingType::Immediate, Opcode::LDX, 2, 2, false),
    );
    hash.insert(
        0xa4,
        Inst::new(AddressingType::ZeroPage, Opcode::LDY, 2, 3, false),
    );
    hash.insert(
        0xa5,
        Inst::new(AddressingType::ZeroPage, Opcode::LDA, 2, 4, false),
    );
    hash.insert(
        0xa6,
        Inst::new(AddressingType::ZeroPage, Opcode::LDX, 2, 3, false),
    );
    hash.insert(
        0xa8,
        Inst::new(AddressingType::Implied, Opcode::TAY, 1, 2, false),
    );
    hash.insert(
        0xa9,
        Inst::new(AddressingType::Immediate, Opcode::LDA, 2, 2, false),
    );
    hash.insert(
        0xaa,
        Inst::new(AddressingType::Implied, Opcode::TAX, 1, 2, false),
    );
    hash.insert(
        0xac,
        Inst::new(AddressingType::Absolute, Opcode::LDY, 3, 4, false),
    );
    hash.insert(
        0xad,
        Inst::new(AddressingType::Absolute, Opcode::LDA, 3, 4, false),
    );
    hash.insert(
        0xae,
        Inst::new(AddressingType::Absolute, Opcode::LDX, 3, 4, false),
    );
    hash.insert(
        0xb0,
        Inst::new(AddressingType::Relative, Opcode::BCS, 2, 2, true),
    );
    hash.insert(
        0xb1,
        Inst::new(AddressingType::IndirectY, Opcode::LDA, 2, 5, true),
    );
    hash.insert(
        0xb4,
        Inst::new(AddressingType::ZeroPageX, Opcode::LDY, 2, 4, false),
    );
    hash.insert(
        0xb5,
        Inst::new(AddressingType::ZeroPageX, Opcode::LDA, 2, 4, false),
    );
    hash.insert(
        0xb6,
        Inst::new(AddressingType::ZeroPageY, Opcode::LDX, 2, 4, false),
    );
    hash.insert(
        0xb8,
        Inst::new(AddressingType::Implied, Opcode::CLV, 1, 2, false),
    );
    hash.insert(
        0xb9,
        Inst::new(AddressingType::AbsoluteY, Opcode::LDA, 3, 4, true),
    );
    hash.insert(
        0xba,
        Inst::new(AddressingType::Implied, Opcode::TSX, 1, 2, false),
    );
    hash.insert(
        0xbc,
        Inst::new(AddressingType::AbsoluteX, Opcode::LDY, 3, 4, true),
    );
    hash.insert(
        0xbd,
        Inst::new(AddressingType::AbsoluteX, Opcode::LDA, 3, 4, true),
    );
    hash.insert(
        0xbe,
        Inst::new(AddressingType::AbsoluteY, Opcode::LDX, 3, 4, true),
    );
    hash.insert(
        0xc0,
        Inst::new(AddressingType::Immediate, Opcode::CPY, 2, 2, false),
    );
    hash.insert(
        0xc1,
        Inst::new(AddressingType::IndirectX, Opcode::CMP, 2, 6, false),
    );
    hash.insert(
        0xc4,
        Inst::new(AddressingType::ZeroPage, Opcode::CPY, 2, 3, false),
    );
    hash.insert(
        0xc5,
        Inst::new(AddressingType::ZeroPage, Opcode::CMP, 2, 3, false),
    );
    hash.insert(
        0xc6,
        Inst::new(AddressingType::ZeroPage, Opcode::DEC, 2, 5, false),
    );
    hash.insert(
        0xc8,
        Inst::new(AddressingType::Implied, Opcode::INY, 1, 2, false),
    );
    hash.insert(
        0xc9,
        Inst::new(AddressingType::Immediate, Opcode::CMP, 2, 2, false),
    );
    hash.insert(
        0xca,
        Inst::new(AddressingType::Implied, Opcode::DEX, 1, 2, false),
    );
    hash.insert(
        0xcc,
        Inst::new(AddressingType::Absolute, Opcode::CPY, 3, 4, false),
    );
    hash.insert(
        0xcd,
        Inst::new(AddressingType::Absolute, Opcode::CMP, 3, 4, false),
    );
    hash.insert(
        0xce,
        Inst::new(AddressingType::Absolute, Opcode::DEC, 3, 6, false),
    );
    hash.insert(
        0xd0,
        Inst::new(AddressingType::Relative, Opcode::BNE, 2, 2, true),
    );
    hash.insert(
        0xd1,
        Inst::new(AddressingType::IndirectY, Opcode::CMP, 2, 5, true),
    );
    hash.insert(
        0xd5,
        Inst::new(AddressingType::ZeroPageX, Opcode::CMP, 2, 4, false),
    );
    hash.insert(
        0xd6,
        Inst::new(AddressingType::ZeroPageX, Opcode::DEC, 2, 6, false),
    );
    hash.insert(
        0xd8,
        Inst::new(AddressingType::Implied, Opcode::CLD, 1, 2, false),
    );
    hash.insert(
        0xd9,
        Inst::new(AddressingType::AbsoluteY, Opcode::CMP, 3, 4, true),
    );
    hash.insert(
        0xdd,
        Inst::new(AddressingType::AbsoluteX, Opcode::CMP, 3, 4, true),
    );
    hash.insert(
        0xde,
        Inst::new(AddressingType::AbsoluteX, Opcode::DEC, 3, 7, false),
    );
    hash.insert(
        0xe0,
        Inst::new(AddressingType::Immediate, Opcode::CPX, 2, 2, false),
    );
    hash.insert(
        0xe1,
        Inst::new(AddressingType::IndirectX, Opcode::SBC, 2, 6, false),
    );
    hash.insert(
        0xe4,
        Inst::new(AddressingType::ZeroPage, Opcode::CPX, 2, 3, false),
    );
    hash.insert(
        0xe5,
        Inst::new(AddressingType::ZeroPage, Opcode::SBC, 2, 3, false),
    );
    hash.insert(
        0xe6,
        Inst::new(AddressingType::ZeroPage, Opcode::INC, 2, 5, false),
    );
    hash.insert(
        0xe8,
        Inst::new(AddressingType::Implied, Opcode::INX, 1, 2, false),
    );
    hash.insert(
        0xe9,
        Inst::new(AddressingType::Immediate, Opcode::SBC, 2, 2, false),
    );
    hash.insert(
        0xea,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 2, false),
    );
    hash.insert(
        0xec,
        Inst::new(AddressingType::Absolute, Opcode::CPX, 3, 4, false),
    );
    hash.insert(
        0xed,
        Inst::new(AddressingType::Absolute, Opcode::SBC, 3, 4, false),
    );
    hash.insert(
        0xee,
        Inst::new(AddressingType::Absolute, Opcode::INC, 3, 6, false),
    );
    hash.insert(
        0xf0,
        Inst::new(AddressingType::Relative, Opcode::BEQ, 2, 2, true),
    );
    hash.insert(
        0xf1,
        Inst::new(AddressingType::IndirectY, Opcode::SBC, 2, 5, true),
    );
    hash.insert(
        0xf5,
        Inst::new(AddressingType::ZeroPageX, Opcode::SBC, 2, 4, false),
    );
    hash.insert(
        0xf6,
        Inst::new(AddressingType::ZeroPageX, Opcode::INC, 2, 6, false),
    );
    hash.insert(
        0xf8,
        Inst::new(AddressingType::Implied, Opcode::SED, 1, 2, false),
    );
    hash.insert(
        0xf9,
        Inst::new(AddressingType::AbsoluteY, Opcode::SBC, 3, 4, true),
    );
    hash.insert(
        0xfd,
        Inst::new(AddressingType::AbsoluteX, Opcode::SBC, 3, 4, true),
    );
    hash.insert(
        0xfe,
        Inst::new(AddressingType::AbsoluteX, Opcode::INC, 3, 7, false),
    );
    //invalid inst
    hash.insert(
        0x04,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x44,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x64,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x0c,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0x14,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x34,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x54,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x74,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0xd4,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0xf4,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x80,
        Inst::new(AddressingType::Implied, Opcode::NOP, 2, 1, true),
    );
    hash.insert(
        0x1c,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0x3c,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0x5c,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0x7c,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0xdc,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0xfc,
        Inst::new(AddressingType::Implied, Opcode::NOP, 3, 1, true),
    );
    hash.insert(
        0x1a,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    hash.insert(
        0x3a,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    hash.insert(
        0x5a,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    hash.insert(
        0x7a,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    hash.insert(
        0xda,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    hash.insert(
        0xfa,
        Inst::new(AddressingType::Implied, Opcode::NOP, 1, 1, true),
    );
    //invalid but useful (fuck stupid 6502)
    hash.insert(
        0xa3,
        Inst::new(AddressingType::IndirectX, Opcode::LAX, 2, 1, true),
    );
    hash.insert(
        0xa7,
        Inst::new(AddressingType::ZeroPage, Opcode::LAX, 2, 1, true),
    );
    hash.insert(
        0xaf,
        Inst::new(AddressingType::Absolute, Opcode::LAX, 3, 1, true),
    );
    hash.insert(
        0xb3,
        Inst::new(AddressingType::IndirectY, Opcode::LAX, 2, 1, true),
    );
    hash.insert(
        0xb7,
        Inst::new(AddressingType::ZeroPageY, Opcode::LAX, 2, 1, true),
    );
    hash.insert(
        0xbf,
        Inst::new(AddressingType::AbsoluteY, Opcode::LAX, 3, 1, true),
    );
    hash.insert(
        0x83,
        Inst::new(AddressingType::IndirectX, Opcode::SAX, 2, 1, true),
    );
    hash.insert(
        0x87,
        Inst::new(AddressingType::ZeroPage, Opcode::SAX, 2, 1, true),
    );
    hash.insert(
        0x97,
        Inst::new(AddressingType::ZeroPageY, Opcode::SAX, 2, 1, true),
    );
    hash.insert(
        0x8f,
        Inst::new(AddressingType::Absolute, Opcode::SAX, 3, 1, true),
    );
    hash.insert(
        0xeb,
        Inst::new(AddressingType::Immediate, Opcode::SBC, 2, 1, true),
    );
    hash.insert(
        0xc7,
        Inst::new(AddressingType::ZeroPage, Opcode::DCP, 2, 1, true),
    );
    hash.insert(
        0xd7,
        Inst::new(AddressingType::ZeroPageX, Opcode::DCP, 2, 1, true),
    );
    hash.insert(
        0xc3,
        Inst::new(AddressingType::IndirectX, Opcode::DCP, 2, 1, true),
    );
    hash.insert(
        0xd3,
        Inst::new(AddressingType::IndirectY, Opcode::DCP, 2, 1, true),
    );
    hash.insert(
        0xcf,
        Inst::new(AddressingType::Absolute, Opcode::DCP, 3, 1, true),
    );
    hash.insert(
        0xdf,
        Inst::new(AddressingType::AbsoluteX, Opcode::DCP, 3, 1, true),
    );
    hash.insert(
        0xdb,
        Inst::new(AddressingType::AbsoluteY, Opcode::DCP, 3, 1, true),
    );
    hash.insert(
        0xe7,
        Inst::new(AddressingType::ZeroPage, Opcode::ISC, 2, 1, true),
    );
    hash.insert(
        0xf7,
        Inst::new(AddressingType::ZeroPageX, Opcode::ISC, 2, 1, true),
    );
    hash.insert(
        0xe3,
        Inst::new(AddressingType::IndirectX, Opcode::ISC, 2, 1, true),
    );
    hash.insert(
        0xf3,
        Inst::new(AddressingType::IndirectY, Opcode::ISC, 2, 1, true),
    );
    hash.insert(
        0xef,
        Inst::new(AddressingType::Absolute, Opcode::ISC, 3, 1, true),
    );
    hash.insert(
        0xff,
        Inst::new(AddressingType::AbsoluteX, Opcode::ISC, 3, 1, true),
    );
    hash.insert(
        0xfb,
        Inst::new(AddressingType::AbsoluteY, Opcode::ISC, 3, 1, true),
    );
    hash.insert(
        0x07,
        Inst::new(AddressingType::ZeroPage, Opcode::SLO, 2, 1, true),
    );
    hash.insert(
        0x17,
        Inst::new(AddressingType::ZeroPageX, Opcode::SLO, 2, 1, true),
    );
    hash.insert(
        0x03,
        Inst::new(AddressingType::IndirectX, Opcode::SLO, 2, 1, true),
    );
    hash.insert(
        0x13,
        Inst::new(AddressingType::IndirectY, Opcode::SLO, 2, 1, true),
    );
    hash.insert(
        0x0f,
        Inst::new(AddressingType::Absolute, Opcode::SLO, 3, 1, true),
    );
    hash.insert(
        0x1f,
        Inst::new(AddressingType::AbsoluteX, Opcode::SLO, 3, 1, true),
    );
    hash.insert(
        0x1b,
        Inst::new(AddressingType::AbsoluteY, Opcode::SLO, 3, 1, true),
    );
    hash.insert(
        0x27,
        Inst::new(AddressingType::ZeroPage, Opcode::RLA, 2, 1, true),
    );
    hash.insert(
        0x37,
        Inst::new(AddressingType::ZeroPageX, Opcode::RLA, 2, 1, true),
    );
    hash.insert(
        0x23,
        Inst::new(AddressingType::IndirectX, Opcode::RLA, 2, 1, true),
    );
    hash.insert(
        0x33,
        Inst::new(AddressingType::IndirectY, Opcode::RLA, 2, 1, true),
    );
    hash.insert(
        0x2f,
        Inst::new(AddressingType::Absolute, Opcode::RLA, 3, 1, true),
    );
    hash.insert(
        0x3f,
        Inst::new(AddressingType::AbsoluteX, Opcode::RLA, 3, 1, true),
    );
    hash.insert(
        0x3b,
        Inst::new(AddressingType::AbsoluteY, Opcode::RLA, 3, 1, true),
    );
    hash.insert(
        0x47,
        Inst::new(AddressingType::ZeroPage, Opcode::SRE, 2, 1, true),
    );
    hash.insert(
        0x57,
        Inst::new(AddressingType::ZeroPageX, Opcode::SRE, 2, 1, true),
    );
    hash.insert(
        0x43,
        Inst::new(AddressingType::IndirectX, Opcode::SRE, 2, 1, true),
    );
    hash.insert(
        0x53,
        Inst::new(AddressingType::IndirectY, Opcode::SRE, 2, 1, true),
    );
    hash.insert(
        0x4f,
        Inst::new(AddressingType::Absolute, Opcode::SRE, 3, 1, true),
    );
    hash.insert(
        0x5f,
        Inst::new(AddressingType::AbsoluteX, Opcode::SRE, 3, 1, true),
    );
    hash.insert(
        0x5b,
        Inst::new(AddressingType::AbsoluteY, Opcode::SRE, 3, 1, true),
    );
    hash.insert(
        0x67,
        Inst::new(AddressingType::ZeroPage, Opcode::RRA, 2, 1, true),
    );
    hash.insert(
        0x77,
        Inst::new(AddressingType::ZeroPageX, Opcode::RRA, 2, 1, true),
    );
    hash.insert(
        0x63,
        Inst::new(AddressingType::IndirectX, Opcode::RRA, 2, 1, true),
    );
    hash.insert(
        0x73,
        Inst::new(AddressingType::IndirectY, Opcode::RRA, 2, 1, true),
    );
    hash.insert(
        0x6f,
        Inst::new(AddressingType::Absolute, Opcode::RRA, 3, 1, true),
    );
    hash.insert(
        0x7f,
        Inst::new(AddressingType::AbsoluteX, Opcode::RRA, 3, 1, true),
    );
    hash.insert(
        0x7b,
        Inst::new(AddressingType::AbsoluteY, Opcode::RRA, 3, 1, true),
    );

    hash
}

#[test]
fn test() {
    println!("{:#}", INST_TABLE.len());
}
