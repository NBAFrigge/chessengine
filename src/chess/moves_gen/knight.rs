use std::io::Empty;

const NOT_AFILE: u64 = 0xfefefefefefefefe;
const NOT_HFILE: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_ABFILE: u64 = 0xfcfcfcfcfcfcfcfc;
const NOT_GHFILE: u64 = 0x3f3f3f3f3f3f3f3f;

pub fn moves(b : u64,) -> u64 {
    north_north_west(b) |
    north_north_east(b) |
    north_west_west(b) |
    north_east_east(b) |
    south_west_west(b) |
    south_east_east(b) |
    south_south_west(b) |
    south_south_east(b)
}

fn north_north_west(b :u64) -> u64 {(b << 15) & NOT_HFILE}
fn north_north_east(b :u64) -> u64 {(b << 17) & NOT_AFILE}
fn north_west_west(b :u64) -> u64 {(b << 6) & NOT_GHFILE}
fn north_east_east(b :u64) -> u64 {(b << 10) & NOT_ABFILE}
fn south_west_west(b : u64) -> u64 {(b >> 10) & NOT_GHFILE}
fn south_east_east(b: u64) -> u64 {(b >> 6) & NOT_ABFILE}
fn south_south_west(b: u64) -> u64 {(b >> 17) & NOT_HFILE}
fn south_south_east(b :u64) -> u64 {(b >> 15) & NOT_AFILE}