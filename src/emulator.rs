use crate::bus::{CPUBus, PPUBus};
use crate::ram::{CPURam, VRam};
use crate::rom::Cartridge;

pub struct Emulator {
    cpu: (),
    ppu:(),
    cpu_bus: CPUBus,
    ppu_bus: PPUBus,
    ram: CPURam,
    vram: VRam,
    cartridge: Cartridge,
}

impl Emulator {

}
