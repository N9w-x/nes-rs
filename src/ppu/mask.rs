use bitflags::bitflags;
bitflags! {
    pub struct Mask: u8 {
        const GrayScale             = 1 << 0;
        const ShowBackGroundLeft8px = 1 << 1;
        const ShowSpriteLeft8px     = 1 << 2;
        const ShowBackGround        = 1 << 3;
        const ShowSprite            = 1 << 4;
        const EmphasizeRed          = 1 << 5;
        const EmphasizeGreen        = 1 << 6;
        const EmphasizeBlue         = 1 << 7;
    }
}

impl Mask {
    pub fn is_grayscale(&self) -> bool {
        self.bits & 0x1 != 0
    }

    pub fn is_show_background_left_8px(&self) -> bool {
        self.bits & 0x2 != 0
    }

    pub fn is_show_sprite_left_8px(&self) -> bool {
        self.bits & 0x4 != 0
    }

    pub fn is_show_background(&self) -> bool {
        self.contains(Mask::ShowBackGround)
    }

    pub fn is_show_sprite(&self) -> bool {
        self.contains(Mask::ShowSprite)
    }

    pub fn is_emphasize_red(&self) -> bool {
        self.contains(Mask::EmphasizeRed)
    }

    pub fn is_emphasize_green(&self) -> bool {
        self.contains(Mask::EmphasizeGreen)
    }

    pub fn is_emphasize_blue(&self) -> bool {
        self.contains(Mask::EmphasizeBlue)
    }

    pub fn update(&mut self, val: u8) {
        self.bits = val
    }
}
