use std::fmt::Debug;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process::exit;

use crate::error::{EmuError, handle_result};
use crate::rom::mapper::NRom;

use super::mapper::{Mapper, NROM};

#[derive(Debug, Clone, Copy)]
pub enum MirrorType {
    Horizontal,
    Vertical,
    FourScreen,
}

impl From<MirrorType> for bool {
    fn from(data: MirrorType) -> Self {
        !matches!(data, MirrorType::Horizontal)
    }
}

impl From<bool> for MirrorType {
    fn from(data: bool) -> Self {
        if data {
            MirrorType::Vertical
        } else {
            MirrorType::Horizontal
        }
    }
}

pub struct CartridgeInfo {
    pub prg: usize,
    pub chr: usize,
    pub mapper: usize,
    pub mirror_type: MirrorType,
    pub has_backed: bool,
    pub data_start: usize,
}

impl CartridgeInfo {
    pub fn parse_header(buf: [u8; 16]) -> Result<Self, EmuError> {
        let mut magic = [0u8; 4];
        magic.copy_from_slice(&buf[0..4]);
        if magic != [0x4e, 0x45, 0x53, 0x1a] {
            return Err(EmuError::new(
                "magic check failed ,invalid ines file".to_string(),
                file!().to_string(),
                line!(),
            ));
        }
        
        let prg = buf[4] as usize;
        let chr = buf[5] as usize;
        let mirror = if buf[6] & 0b1 != 0 {
            MirrorType::Vertical
        } else if buf[6] & 0b1000 != 0 {
            MirrorType::FourScreen
        } else {
            MirrorType::Horizontal
        };
        let backed = buf[6] & 0b10 != 0;
        let mapper = buf[7] & 0xf0 | ((buf[6] & 0xf0) >> 4);
        let data_start = if buf[6] & 0b100 != 0 { 16 + 512 } else { 16 };
        
        Ok(Self {
            prg,
            chr,
            mapper: mapper as usize,
            mirror_type: mirror,
            has_backed: backed,
            data_start,
        })
    }
}

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
    info: CartridgeInfo,
}

impl Cartridge {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image_file = handle_result(
            File::open(path)
                .map_err(|e| EmuError::new(e.to_string(), file!().to_string(), line!())),
        );
        
        let mut buf = [0u8; 16];
        let _ = image_file.read_at(&mut buf, 0).unwrap();
        let info = handle_result(CartridgeInfo::parse_header(buf));
        
        //read prg and chr data from ines
        let mut prg = vec![0u8; info.prg * 0x4000];
        let mut chr = vec![0u8; info.chr * 0x2000];
        let mut ptr = info.data_start;
        let _ = image_file.read_at(&mut prg, ptr as u64);
        ptr += prg.len();
        let _ = image_file.read_at(&mut chr, ptr as u64);
        
        match info.mapper {
            NROM => {
                let nrom = NRom::new(prg, chr, info.mirror_type.into());
                Self {
                    mapper: Box::new(nrom),
                    info,
                }
            }
            _ => {
                println!("unsupported mapper type {}", info.mapper);
                exit(0)
            }
        }
    }
    
    pub fn read(&self, address: u16) -> u8 {
        self.mapper.read(address)
    }
    
    pub fn write(&mut self, address: u16, data: u8) {
        self.mapper.write(address, data)
    }
}

#[test]
fn test() {
    let cart = Cartridge::new("./test/nestest.nes");
    println!("trest");
}
