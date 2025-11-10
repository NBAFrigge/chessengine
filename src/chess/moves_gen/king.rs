const NOT_AFILE: u64 = 0xfefefefefefefefe;
const NOT_HFILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn moves(b: u64) -> u64 {
    north_one(b)
        | east_one(b)
        | west_one(b)
        | south_one(b)
        | northeast_one(b)
        | southeast_one(b)
        | southwest_one(b)
        | northwest_one(b)
}

fn north_one(b: u64) -> u64 {
    b << 8
}
fn south_one(b: u64) -> u64 {
    b >> 8
}
fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_AFILE
}
fn northeast_one(b: u64) -> u64 {
    (b << 9) & NOT_AFILE
}
fn southeast_one(b: u64) -> u64 {
    (b >> 7) & NOT_AFILE
}
fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_HFILE
}
fn southwest_one(b: u64) -> u64 {
    (b >> 9) & NOT_HFILE
}
fn northwest_one(b: u64) -> u64 {
    (b << 7) & NOT_HFILE
}

