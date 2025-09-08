const NOT_AFILE: u64 = 0xfefefefefefefefe;
const NOT_HFILE: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_ABFILE: u64 = 0xfcfcfcfcfcfcfcfc;
const NOT_GHFILE: u64 = 0x3f3f3f3f3f3f3f3f;

pub fn north_northWest(b :u64) -> u64 {(b << 15) & NOT_HFILE}
pub fn north_northEast(b :u64) -> u64 {(b << 17) & NOT_AFILE}
pub fn north_westWest(b :u64) -> u64 {(b << 6) & NOT_GHFILE}
pub fn north_eastEast(b :u64) -> u64 {(b << 10) & NOT_ABFILE}
pub fn south_westWest(b : u64) -> u64 {(b >> 10) & NOT_GHFILE}
pub fn south_eastEast(b: u64) -> u64 {(b >> 6) & NOT_ABFILE}
pub fn south_southWest(b: u64) -> u64 {(b >> 17) & NOT_HFILE}
pub fn south_southEast(b :u64) -> u64 {(b >> 15) & NOT_AFILE}