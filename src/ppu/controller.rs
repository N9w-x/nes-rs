use bitflags::bitflags;

pub enum SpriteSize {
    SIZE8X8,
    SIZE8X16,
}

bitflags! {
    pub struct Controller:u8 {
        const NAME_TABLE1               = 1 << 0;
        const NAME_TABLE2               = 1 << 1;
        const ADD_INCREMENT             = 1 << 2;
        const SPRITE_PATTERN_ADDR       = 1 << 3;
        const BACKGROUD_PATTERN_ADDR    = 1 << 4;
        const SPRITE_SIZE               = 1 << 5;
        const MASTER_SLAVE_SELECT       = 1 << 6;
        const GENERATE_NMI              = 1 << 7;
    }
}

impl Controller {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn nametable_baseaddr(&self) -> usize {
        let val = self.bits & 0x3;
        match val {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => unreachable!(),
        }
    }

    pub fn sprite_pattern_address(&self) -> usize {
        if self.bits & 0x8 != 0 {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn background_pattern_address(&self) -> usize {
        if self.bits & 0x10 != 0 {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn vram_add_increment(&self) -> usize {
        if self.bits & 0x4 != 0 {
            32
        } else {
            1
        }
    }

    pub fn sprite_size(&self) -> SpriteSize {
        if 0x20 != 0 {
            SpriteSize::SIZE8X16
        } else {
            SpriteSize::SIZE8X8
        }
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data
    }
}
