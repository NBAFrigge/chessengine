#[derive(Clone, Copy)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

pub static ROOK_MAGICS: [Magic; 64] = [
    Magic {
        mask: 0x000101010101017E,
        magic: 0x0480006440018091,
        shift: 52,
        offset: 0,
    },
    Magic {
        mask: 0x000202020202027C,
        magic: 0x2C40100020084000,
        shift: 53,
        offset: 4096,
    },
    Magic {
        mask: 0x000404040404047A,
        magic: 0x0880200008819001,
        shift: 53,
        offset: 6144,
    },
    Magic {
        mask: 0x0008080808080876,
        magic: 0x0480080050008481,
        shift: 53,
        offset: 8192,
    },
    Magic {
        mask: 0x001010101010106E,
        magic: 0x4500080002050090,
        shift: 53,
        offset: 10240,
    },
    Magic {
        mask: 0x002020202020205E,
        magic: 0x0100010004000802,
        shift: 53,
        offset: 12288,
    },
    Magic {
        mask: 0x004040404040403E,
        magic: 0x0080008001000200,
        shift: 53,
        offset: 14336,
    },
    Magic {
        mask: 0x008080808080807E,
        magic: 0x0200002200940041,
        shift: 52,
        offset: 16384,
    },
    Magic {
        mask: 0x0001010101017E00,
        magic: 0x0002800240028020,
        shift: 53,
        offset: 20480,
    },
    Magic {
        mask: 0x0002020202027C00,
        magic: 0x8611400240201008,
        shift: 54,
        offset: 22528,
    },
    Magic {
        mask: 0x0004040404047A00,
        magic: 0x0401002001004010,
        shift: 54,
        offset: 23552,
    },
    Magic {
        mask: 0x0008080808087600,
        magic: 0x0010801000800800,
        shift: 54,
        offset: 24576,
    },
    Magic {
        mask: 0x0010101010106E00,
        magic: 0x8003001100080204,
        shift: 54,
        offset: 25600,
    },
    Magic {
        mask: 0x0020202020205E00,
        magic: 0x2008808002000400,
        shift: 54,
        offset: 26624,
    },
    Magic {
        mask: 0x0040404040403E00,
        magic: 0x221A000892000411,
        shift: 54,
        offset: 27648,
    },
    Magic {
        mask: 0x0080808080807E00,
        magic: 0x10C1000880650002,
        shift: 53,
        offset: 28672,
    },
    Magic {
        mask: 0x00010101017E0100,
        magic: 0x01008480004002A0,
        shift: 53,
        offset: 30720,
    },
    Magic {
        mask: 0x00020202027C0200,
        magic: 0x9280404010002000,
        shift: 54,
        offset: 32768,
    },
    Magic {
        mask: 0x00040404047A0400,
        magic: 0x9060028020821000,
        shift: 54,
        offset: 33792,
    },
    Magic {
        mask: 0x0008080808760800,
        magic: 0x000242000A002210,
        shift: 54,
        offset: 34816,
    },
    Magic {
        mask: 0x00101010106E1000,
        magic: 0x0008010005090010,
        shift: 54,
        offset: 35840,
    },
    Magic {
        mask: 0x00202020205E2000,
        magic: 0x0C13010004000208,
        shift: 54,
        offset: 36864,
    },
    Magic {
        mask: 0x00404040403E4000,
        magic: 0x0400010100020004,
        shift: 54,
        offset: 37888,
    },
    Magic {
        mask: 0x00808080807E8000,
        magic: 0x0204020000508104,
        shift: 53,
        offset: 38912,
    },
    Magic {
        mask: 0x000101017E010100,
        magic: 0x0040400080002080,
        shift: 53,
        offset: 40960,
    },
    Magic {
        mask: 0x000202027C020200,
        magic: 0x0080400140201002,
        shift: 54,
        offset: 43008,
    },
    Magic {
        mask: 0x000404047A040400,
        magic: 0x0200200080100080,
        shift: 54,
        offset: 44032,
    },
    Magic {
        mask: 0x0008080876080800,
        magic: 0x0908204200081200,
        shift: 54,
        offset: 45056,
    },
    Magic {
        mask: 0x001010106E101000,
        magic: 0x0000040080080080,
        shift: 54,
        offset: 46080,
    },
    Magic {
        mask: 0x002020205E202000,
        magic: 0x0001004900022400,
        shift: 54,
        offset: 47104,
    },
    Magic {
        mask: 0x004040403E404000,
        magic: 0x0000818400080210,
        shift: 54,
        offset: 48128,
    },
    Magic {
        mask: 0x008080807E808000,
        magic: 0x900080038000C900,
        shift: 53,
        offset: 49152,
    },
    Magic {
        mask: 0x0001017E01010100,
        magic: 0xA002804001800020,
        shift: 53,
        offset: 51200,
    },
    Magic {
        mask: 0x0002027C02020200,
        magic: 0x0050400080802000,
        shift: 54,
        offset: 53248,
    },
    Magic {
        mask: 0x0004047A04040400,
        magic: 0x4801002005004210,
        shift: 54,
        offset: 54272,
    },
    Magic {
        mask: 0x0008087608080800,
        magic: 0x4010008008080100,
        shift: 54,
        offset: 55296,
    },
    Magic {
        mask: 0x0010106E10101000,
        magic: 0x0201110135001800,
        shift: 54,
        offset: 56320,
    },
    Magic {
        mask: 0x0020205E20202000,
        magic: 0x0106000802001004,
        shift: 54,
        offset: 57344,
    },
    Magic {
        mask: 0x0040403E40404000,
        magic: 0x080090280400460B,
        shift: 54,
        offset: 58368,
    },
    Magic {
        mask: 0x0080807E80808000,
        magic: 0x8000040042000081,
        shift: 53,
        offset: 59392,
    },
    Magic {
        mask: 0x00017E0101010100,
        magic: 0x808002402001C000,
        shift: 53,
        offset: 61440,
    },
    Magic {
        mask: 0x00027C0202020200,
        magic: 0x0010500020004000,
        shift: 54,
        offset: 63488,
    },
    Magic {
        mask: 0x00047A0404040400,
        magic: 0x004A001040820024,
        shift: 54,
        offset: 64512,
    },
    Magic {
        mask: 0x0008760808080800,
        magic: 0x0008000810008080,
        shift: 54,
        offset: 65536,
    },
    Magic {
        mask: 0x00106E1010101000,
        magic: 0x4020040008008080,
        shift: 54,
        offset: 66560,
    },
    Magic {
        mask: 0x00205E2020202000,
        magic: 0x0002000804010100,
        shift: 54,
        offset: 67584,
    },
    Magic {
        mask: 0x00403E4040404000,
        magic: 0xA200040200010100,
        shift: 54,
        offset: 68608,
    },
    Magic {
        mask: 0x00807E8080808000,
        magic: 0x400100A844020001,
        shift: 53,
        offset: 69632,
    },
    Magic {
        mask: 0x007E010101010100,
        magic: 0x010A801040022080,
        shift: 53,
        offset: 71680,
    },
    Magic {
        mask: 0x007C020202020200,
        magic: 0x0800811040002100,
        shift: 54,
        offset: 73728,
    },
    Magic {
        mask: 0x007A040404040400,
        magic: 0x0100100080200880,
        shift: 54,
        offset: 74752,
    },
    Magic {
        mask: 0x0076080808080800,
        magic: 0x4010100080080180,
        shift: 54,
        offset: 75776,
    },
    Magic {
        mask: 0x006E101010101000,
        magic: 0x4209008490080100,
        shift: 54,
        offset: 76800,
    },
    Magic {
        mask: 0x005E202020202000,
        magic: 0x0C1A000409900200,
        shift: 54,
        offset: 77824,
    },
    Magic {
        mask: 0x003E404040404000,
        magic: 0x0000800200010080,
        shift: 54,
        offset: 78848,
    },
    Magic {
        mask: 0x007E808080808000,
        magic: 0x5011112400884200,
        shift: 53,
        offset: 79872,
    },
    Magic {
        mask: 0x7E01010101010100,
        magic: 0x0005009040218005,
        shift: 52,
        offset: 81920,
    },
    Magic {
        mask: 0x7C02020202020200,
        magic: 0x2401022080C00055,
        shift: 53,
        offset: 86016,
    },
    Magic {
        mask: 0x7A04040404040400,
        magic: 0x0090200100400811,
        shift: 53,
        offset: 88064,
    },
    Magic {
        mask: 0x7608080808080800,
        magic: 0x0000050010002009,
        shift: 53,
        offset: 90112,
    },
    Magic {
        mask: 0x6E10101010101000,
        magic: 0x8422002110048802,
        shift: 53,
        offset: 92160,
    },
    Magic {
        mask: 0x5E20202020202000,
        magic: 0x230A00010C100836,
        shift: 53,
        offset: 94208,
    },
    Magic {
        mask: 0x3E40404040404000,
        magic: 0xC000020108009004,
        shift: 53,
        offset: 96256,
    },
    Magic {
        mask: 0x7E80808080808000,
        magic: 0x8920004114208402,
        shift: 52,
        offset: 98304,
    },
];

pub static BISHOP_MAGICS: [Magic; 64] = [
    Magic {
        mask: 0x0040201008040200,
        magic: 0x2004082841002200,
        shift: 58,
        offset: 0,
    },
    Magic {
        mask: 0x0000402010080400,
        magic: 0x0008180800404A00,
        shift: 59,
        offset: 64,
    },
    Magic {
        mask: 0x0000004020100A00,
        magic: 0x241000C200408040,
        shift: 59,
        offset: 96,
    },
    Magic {
        mask: 0x0000000040221400,
        magic: 0x58110400802E0204,
        shift: 59,
        offset: 128,
    },
    Magic {
        mask: 0x0000000002442800,
        magic: 0x000110400A264101,
        shift: 59,
        offset: 160,
    },
    Magic {
        mask: 0x0000000204085000,
        magic: 0xE0410402C0140208,
        shift: 59,
        offset: 192,
    },
    Magic {
        mask: 0x0000020408102000,
        magic: 0x0622021004850412,
        shift: 59,
        offset: 224,
    },
    Magic {
        mask: 0x0002040810204000,
        magic: 0x10A1004200A00808,
        shift: 58,
        offset: 256,
    },
    Magic {
        mask: 0x0020100804020000,
        magic: 0x2B42040408084100,
        shift: 59,
        offset: 320,
    },
    Magic {
        mask: 0x0040201008040000,
        magic: 0x0218604881085080,
        shift: 59,
        offset: 352,
    },
    Magic {
        mask: 0x00004020100A0000,
        magic: 0x4800101400803402,
        shift: 59,
        offset: 384,
    },
    Magic {
        mask: 0x0000004022140000,
        magic: 0x988C080681000021,
        shift: 59,
        offset: 416,
    },
    Magic {
        mask: 0x0000000244280000,
        magic: 0x0201C11040081A01,
        shift: 59,
        offset: 448,
    },
    Magic {
        mask: 0x0000020408500000,
        magic: 0x0202084402202000,
        shift: 59,
        offset: 480,
    },
    Magic {
        mask: 0x0002040810200000,
        magic: 0x00C0041424020808,
        shift: 59,
        offset: 512,
    },
    Magic {
        mask: 0x0004081020400000,
        magic: 0x0000184114100221,
        shift: 59,
        offset: 544,
    },
    Magic {
        mask: 0x0010080402000200,
        magic: 0x0441004408080140,
        shift: 59,
        offset: 576,
    },
    Magic {
        mask: 0x0020100804000400,
        magic: 0x0020C20808212040,
        shift: 59,
        offset: 608,
    },
    Magic {
        mask: 0x004020100A000A00,
        magic: 0x6201021004002042,
        shift: 57,
        offset: 640,
    },
    Magic {
        mask: 0x0000402214001400,
        magic: 0x1002042420220110,
        shift: 57,
        offset: 768,
    },
    Magic {
        mask: 0x0000024428002800,
        magic: 0x2604020822081000,
        shift: 57,
        offset: 896,
    },
    Magic {
        mask: 0x0002040850005000,
        magic: 0x0081020210008400,
        shift: 57,
        offset: 1024,
    },
    Magic {
        mask: 0x0004081020002000,
        magic: 0x0801008208020208,
        shift: 59,
        offset: 1152,
    },
    Magic {
        mask: 0x0008102040004000,
        magic: 0x900420010C024200,
        shift: 59,
        offset: 1184,
    },
    Magic {
        mask: 0x0008040200020400,
        magic: 0x00844020301001D0,
        shift: 59,
        offset: 1216,
    },
    Magic {
        mask: 0x0010080400040800,
        magic: 0x2010082010410100,
        shift: 59,
        offset: 1248,
    },
    Magic {
        mask: 0x0020100A000A1000,
        magic: 0x20080102080A0020,
        shift: 57,
        offset: 1280,
    },
    Magic {
        mask: 0x0040221400142200,
        magic: 0x8000404004010200,
        shift: 55,
        offset: 1408,
    },
    Magic {
        mask: 0x0002442800284400,
        magic: 0x108100100100400C,
        shift: 55,
        offset: 1920,
    },
    Magic {
        mask: 0x0004085000500800,
        magic: 0x1024450132030300,
        shift: 57,
        offset: 2432,
    },
    Magic {
        mask: 0x0008102000201000,
        magic: 0x8000910822080240,
        shift: 59,
        offset: 2560,
    },
    Magic {
        mask: 0x0010204000402000,
        magic: 0x2002002060808801,
        shift: 59,
        offset: 2592,
    },
    Magic {
        mask: 0x0004020002040800,
        magic: 0x2050425060081090,
        shift: 59,
        offset: 2624,
    },
    Magic {
        mask: 0x0008040004081000,
        magic: 0x420414023020C200,
        shift: 59,
        offset: 2656,
    },
    Magic {
        mask: 0x00100A000A102000,
        magic: 0x1011044040080080,
        shift: 57,
        offset: 2688,
    },
    Magic {
        mask: 0x0022140014224000,
        magic: 0x0302020082080080,
        shift: 55,
        offset: 2816,
    },
    Magic {
        mask: 0x0044280028440200,
        magic: 0x0020040900202002,
        shift: 55,
        offset: 3328,
    },
    Magic {
        mask: 0x0008500050080400,
        magic: 0x2000880600104120,
        shift: 57,
        offset: 3840,
    },
    Magic {
        mask: 0x0010200020100800,
        magic: 0x0001291400210424,
        shift: 59,
        offset: 3968,
    },
    Magic {
        mask: 0x0020400040201000,
        magic: 0x01088210890A0084,
        shift: 59,
        offset: 4000,
    },
    Magic {
        mask: 0x0002000204081000,
        magic: 0x0141010840022000,
        shift: 59,
        offset: 4032,
    },
    Magic {
        mask: 0x0004000408102000,
        magic: 0x0041082210020212,
        shift: 59,
        offset: 4064,
    },
    Magic {
        mask: 0x000A000A10204000,
        magic: 0x20224C0048000C00,
        shift: 57,
        offset: 4096,
    },
    Magic {
        mask: 0x0014001422400000,
        magic: 0x0840202018000902,
        shift: 57,
        offset: 4224,
    },
    Magic {
        mask: 0x0028002844020000,
        magic: 0x0000042892000400,
        shift: 57,
        offset: 4352,
    },
    Magic {
        mask: 0x0050005008040200,
        magic: 0x0410011000240700,
        shift: 57,
        offset: 4480,
    },
    Magic {
        mask: 0x0020002010080400,
        magic: 0x8010100485140080,
        shift: 59,
        offset: 4608,
    },
    Magic {
        mask: 0x0040004020100800,
        magic: 0x0250010921080020,
        shift: 59,
        offset: 4640,
    },
    Magic {
        mask: 0x0000020408102000,
        magic: 0xA002009005100018,
        shift: 59,
        offset: 4672,
    },
    Magic {
        mask: 0x0000040810204000,
        magic: 0x50220088C4100008,
        shift: 59,
        offset: 4704,
    },
    Magic {
        mask: 0x00000A1020400000,
        magic: 0xC040002412082800,
        shift: 59,
        offset: 4736,
    },
    Magic {
        mask: 0x0000142240000000,
        magic: 0x2500124084040001,
        shift: 59,
        offset: 4768,
    },
    Magic {
        mask: 0x0000284402000000,
        magic: 0x0000244043820100,
        shift: 59,
        offset: 4800,
    },
    Magic {
        mask: 0x0000500804020000,
        magic: 0x0000212802482200,
        shift: 59,
        offset: 4832,
    },
    Magic {
        mask: 0x0000201008040200,
        magic: 0x9C40100C810D5020,
        shift: 59,
        offset: 4864,
    },
    Magic {
        mask: 0x0000402010080400,
        magic: 0x4902820401020400,
        shift: 59,
        offset: 4896,
    },
    Magic {
        mask: 0x0002040810204000,
        magic: 0x0002004208040200,
        shift: 58,
        offset: 4928,
    },
    Magic {
        mask: 0x0004081020400000,
        magic: 0x18040100A8842030,
        shift: 59,
        offset: 4992,
    },
    Magic {
        mask: 0x000A102040000000,
        magic: 0x0400280020841002,
        shift: 59,
        offset: 5024,
    },
    Magic {
        mask: 0x0014224000000000,
        magic: 0x00A2060900208801,
        shift: 59,
        offset: 5056,
    },
    Magic {
        mask: 0x0028440200000000,
        magic: 0x1001200040050100,
        shift: 59,
        offset: 5088,
    },
    Magic {
        mask: 0x0050080402000000,
        magic: 0x8020000688900500,
        shift: 59,
        offset: 5120,
    },
    Magic {
        mask: 0x0020100804020000,
        magic: 0x0006A00410021048,
        shift: 59,
        offset: 5152,
    },
    Magic {
        mask: 0x0040201008040200,
        magic: 0x0240080208802300,
        shift: 58,
        offset: 5184,
    },
];

pub static mut ROOK_ATTACKS: [u64; 102400] = [0; 102400];
pub static mut BISHOP_ATTACKS: [u64; 5248] = [0; 5248];

fn index_to_occupancy(index: u32, mut mask: u64) -> u64 {
    let mut result = 0u64;
    let mut bit_count = 0;
    while mask != 0 {
        let square = mask.trailing_zeros();
        mask &= mask - 1;
        if (index & (1 << bit_count)) != 0 {
            result |= 1u64 << square;
        }
        bit_count += 1;
    }
    result
}

fn calculate_rook_attacks(square: usize, occupancy: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square / 8;
    let file = square % 8;
    for r in (rank + 1)..8 {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
    }
    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
    }
    for f in (file + 1)..8 {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
    }
    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
    }
    attacks
}

fn calculate_bishop_attacks(square: usize, occupancy: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square / 8;
    let file = square % 8;
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
        r += 1;
        f += 1;
    }
    r = rank.wrapping_sub(1);
    f = file.wrapping_sub(1);
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
        r = r.wrapping_sub(1);
        f = f.wrapping_sub(1);
    }
    r = rank + 1;
    f = file.wrapping_sub(1);
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
        r += 1;
        f = f.wrapping_sub(1);
    }
    r = rank.wrapping_sub(1);
    f = file + 1;
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 {
            break;
        }
        r = r.wrapping_sub(1);
        f += 1;
    }
    attacks
}

pub fn init() {
    for square in 0..64 {
        let magic = &ROOK_MAGICS[square];
        let n_bits = magic.mask.count_ones();
        for i in 0..(1 << n_bits) {
            let occupancy = index_to_occupancy(i, magic.mask);
            let attacks = calculate_rook_attacks(square, occupancy);
            let hash = occupancy.wrapping_mul(magic.magic);
            let index = (hash >> magic.shift) as usize;
            unsafe {
                ROOK_ATTACKS[magic.offset as usize + index] = attacks;
            }
        }
    }
    for square in 0..64 {
        let magic = &BISHOP_MAGICS[square];
        let n_bits = magic.mask.count_ones();
        for i in 0..(1 << n_bits) {
            let occupancy = index_to_occupancy(i, magic.mask);
            let attacks = calculate_bishop_attacks(square, occupancy);
            let hash = occupancy.wrapping_mul(magic.magic);
            let index = (hash >> magic.shift) as usize;
            unsafe {
                BISHOP_ATTACKS[magic.offset as usize + index] = attacks;
            }
        }
    }
}

#[inline(always)]
pub fn rook_attacks(square: usize, occupancy: u64) -> u64 {
    unsafe {
        let magic = ROOK_MAGICS.get_unchecked(square);
        let relevant = occupancy & magic.mask;
        let hash = relevant.wrapping_mul(magic.magic);
        let index = (hash >> magic.shift) as usize;

        let ptr = std::ptr::addr_of!(ROOK_ATTACKS);
        (*ptr).get_unchecked(magic.offset as usize + index).clone()
    }
}

#[inline(always)]
pub fn bishop_attacks(square: usize, occupancy: u64) -> u64 {
    unsafe {
        let magic = BISHOP_MAGICS.get_unchecked(square);
        let relevant = occupancy & magic.mask;
        let hash = relevant.wrapping_mul(magic.magic);
        let index = (hash >> magic.shift) as usize;

        let ptr = std::ptr::addr_of!(BISHOP_ATTACKS);
        (*ptr).get_unchecked(magic.offset as usize + index).clone()
    }
}

#[inline(always)]
pub fn queen_attacks(square: usize, occupancy: u64) -> u64 {
    rook_attacks(square, occupancy) | bishop_attacks(square, occupancy)
}

pub fn rook_moves(b: u64, occ: u64) -> u64 {
    rook_attacks(b.trailing_zeros() as usize, occ)
}

pub fn bishop_moves(b: u64, occ: u64) -> u64 {
    bishop_attacks(b.trailing_zeros() as usize, occ)
}

pub fn queen_moves(b: u64, occ: u64) -> u64 {
    queen_attacks(b.trailing_zeros() as usize, occ)
}
