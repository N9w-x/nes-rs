use crate::cpu::{Flags, Regs};
use crate::{CPUBus, CPURam, Cartridge, CPU};
use regex::Regex;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::rc::Rc;

struct Trace {
    regs: Regs,
}

impl Debug for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.regs)
    }
}

impl Trace {
    fn generate_all_trace<P: AsRef<Path>>(path: P) -> Vec<Trace> {
        let mut standard_log = File::open(path).unwrap();
        let mut buf = String::new();
        let _ = standard_log.read_to_string(&mut buf).unwrap();
        let line_buf = buf
            .split('\n')
            .filter(|t| !t.is_empty())
            .collect::<Vec<&str>>();
        let mut vec = vec![];
        let regex =
            Regex::new(r"A:(?P<A>\w+)\sX:(?P<X>\w+)\sY:(?P<Y>\w+)\sP:(?P<P>\w+)\sSP:(?P<SP>\w+)")
                .unwrap();

        for line in line_buf {
            vec.push(Trace::generate_one_trace(line, &regex));
        }

        vec
    }

    fn generate_one_trace(line: &str, regex: &Regex) -> Trace {
        let split = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let pc = u16::from_str_radix(split[0], 16).unwrap();
        let caps = regex.captures(line).unwrap();

        Trace {
            regs: Regs {
                A: u8::from_str_radix(&caps["A"], 16).unwrap(),
                X: u8::from_str_radix(&caps["X"], 16).unwrap(),
                Y: u8::from_str_radix(&caps["Y"], 16).unwrap(),
                SP: u8::from_str_radix(&caps["SP"], 16).unwrap(),
                P: unsafe {
                    Flags::from_bits_unchecked(u8::from_str_radix(&caps["P"], 16).unwrap())
                },
                PC: pc,
            },
        }
    }
}

pub fn trace<P: AsRef<Path>>(path: P) {
    let cart = Cartridge::new("./test/nestest.nes");
    let ram = CPURam::default();
    let bus = CPUBus::connect(Rc::new(RefCell::new(cart)), Rc::new(RefCell::new(ram)));
    let mut cpu = CPU::new(Rc::new(RefCell::new(bus)), 0xc000);

    let trace_vec = Trace::generate_all_trace(path);
    for trace in trace_vec {
        if trace.regs == cpu.get_regs() {
            let inst = cpu.get_next_inst();
            cpu.exec_once(inst);
        } else {
            println!(
                "Error occur at {:#x}\nregs: {:#?}\ntrace:{:#?}",
                cpu.get_regs().PC,
                cpu.get_regs(),
                trace
            );
            exit(-1);
        }
    }
    println!("finish all test!");
}

#[test]
fn test_gen_trace() {
    let trace = Trace::generate_all_trace("./test/nestest.log");
    println!("{:#?}", trace);
}

#[test]
fn test_trace() {
    trace("./test/nestest.log");
}
