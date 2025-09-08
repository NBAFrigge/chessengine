const NOT_AFILE: u64 = 0xfefefefefefefefe;
const NOT_HFILE: u64 = 0x7f7f7f7f7f7f7f7f;
pub fn north_one(b: u64) -> u64 {b << 8}

pub fn south_one(b: u64) -> u64 {b >> 8}

pub fn east_one(b: u64)  -> u64    {(b << 1) & NOT_AFILE}
pub fn northeast_one(b: u64) -> u64{(b << 9) & NOT_AFILE}
pub fn southeast_one(b: u64) -> u64{(b >> 7) & NOT_AFILE}
pub fn west_one(b: u64)    -> u64  {(b >> 1) & NOT_HFILE}
pub fn southwest_one(b: u64) -> u64{(b >> 9) & NOT_HFILE}
pub fn northwest_one(b: u64) -> u64 {(b << 7) & NOT_HFILE}