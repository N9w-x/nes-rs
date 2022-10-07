use bitflags::bitflags;
bitflags! {
    pub struct Status: u8 {
        const SpriteOverflow    = 1 << 5;
        const Sprite0Hit        = 1 << 6;
        const VBlankStart       = 1 << 7;
    }
}