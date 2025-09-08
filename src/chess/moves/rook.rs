const NOTBORDER: u64 = 0x7e7e7e7e7e7e00;

const FILE :u64 = 0x0101010101010101;
const RANK :u64 = 0xff;
pub fn generateMask(index: u64) -> u64 {
    let table :u64 = 1 << index;
    let rank = index / 8;
    let file = index % 8;

    ((FILE << file |
    RANK << 8 * rank) ^
    table) & NOTBORDER
}

