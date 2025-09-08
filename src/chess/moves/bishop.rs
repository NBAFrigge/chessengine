const NOTBORDER: u64 = 0x7e7e7e7e7e7e00;

const DP:u64 = 0x8040201008040201;
const DS :u64 = 0x102040810204080;
pub fn generateMask(index: u64) -> u64 {
    let y = 8 - index / 8;
    let x = 7 - index % 8;

    let mut m :u64 = 0;
    if (x >= y) {
        m |= DP << ((x - y) << 3);
    } else {
        m |= DP >> ((y - x) << 3);
    }

    let z = 7 - x;

    if (z >= y) {
        m |= DS << ((z - y) << 3);
    } else {
        m |= DS >> ((y - z) << 3);
    }

    m
}