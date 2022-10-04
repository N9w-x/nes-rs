const LAST_BANK: i8 = -1;
const SECOND_LAST_BANK: i8 = -2;

const PRG_BANK_VALUE: [[i8; 4]; 2] = [
    [6, 7, -2, -1],
    [-2, 7, 6, -1]
];

const CHR_BANK_VALUE: [[i8; 8]; 2] = [
    [0, 0, 1, 1, 2, 3, 4, 5],
    [2, 3, 4, 5, 0, 0, 1, 1]
];

pub struct MMC3 {
    regs: [i8; 8],
    
}