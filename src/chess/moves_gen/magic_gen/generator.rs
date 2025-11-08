use rand::Rng;

fn generate_rook_mask(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut mask = 0u64;
    for f in 1..7 {
        if f != file { mask |= 1u64 << (rank * 8 + f); }
    }
    for r in 1..7 {
        if r != rank { mask |= 1u64 << (r * 8 + file); }
    }
    mask
}

fn generate_bishop_mask(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut mask = 0u64;
    let mut r = rank + 1;
    let mut f = file + 1;
    while r <= 6 && f <= 6 {
        mask |= 1u64 << (r * 8 + f);
        r += 1; f += 1;
    }
    r = rank.wrapping_sub(1);
    f = file.wrapping_sub(1);
    while r >= 1 && r < 8 && f >= 1 && f < 8 {
        mask |= 1u64 << (r * 8 + f);
        r = r.wrapping_sub(1); f = f.wrapping_sub(1);
    }
    r = rank + 1;
    f = file.wrapping_sub(1);
    while r <= 6 && f >= 1 && f < 8 {
        mask |= 1u64 << (r * 8 + f);
        r += 1; f = f.wrapping_sub(1);
    }
    r = rank.wrapping_sub(1);
    f = file + 1;
    while r >= 1 && r < 8 && f <= 6 {
        mask |= 1u64 << (r * 8 + f);
        r = r.wrapping_sub(1); f += 1;
    }
    mask
}

fn index_to_occupancy(index: u32, mut mask: u64) -> u64 {
    let mut result = 0u64;
    let mut bit_count = 0;
    while mask != 0 {
        let square = mask.trailing_zeros();
        mask &= mask - 1;
        if (index & (1 << bit_count)) != 0 { result |= 1u64 << square; }
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
        if (occupancy & (1u64 << sq)) != 0 { break; }
    }
    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
    }
    for f in (file + 1)..8 {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
    }
    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
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
        if (occupancy & (1u64 << sq)) != 0 { break; }
        r += 1; f += 1;
    }
    r = rank.wrapping_sub(1);
    f = file.wrapping_sub(1);
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
        r = r.wrapping_sub(1); f = f.wrapping_sub(1);
    }
    r = rank + 1;
    f = file.wrapping_sub(1);
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
        r += 1; f = f.wrapping_sub(1);
    }
    r = rank.wrapping_sub(1);
    f = file + 1;
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (occupancy & (1u64 << sq)) != 0 { break; }
        r = r.wrapping_sub(1); f += 1;
    }
    attacks
}

fn random_u64_fewbits() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>()
}

fn test_magic(square: usize, magic: u64, mask: u64, is_rook: bool) -> bool {
    let n_bits = mask.count_ones();
    let shift = 64 - n_bits;
    let size = 1usize << n_bits;
    let mut table = vec![0u64; size];
    let mut used = vec![false; size];
    
    for i in 0..size {
        let occ = index_to_occupancy(i as u32, mask);
        let att = if is_rook { calculate_rook_attacks(square, occ) } else { calculate_bishop_attacks(square, occ) };
        let idx = ((occ.wrapping_mul(magic)) >> shift) as usize;
        
        if used[idx] {
            if table[idx] != att { return false; }
        } else {
            used[idx] = true;
            table[idx] = att;
        }
    }
    true
}

fn find_magic(square: usize, is_rook: bool) -> u64 {
    let mask = if is_rook { generate_rook_mask(square) } else { generate_bishop_mask(square) };
    let p = if is_rook { "R" } else { "B" };
    let r = square / 8;
    let f = (b'a' + (square % 8) as u8) as char;
    print!("{}{}{} ", p, f, r + 1);
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    
    for _ in 0..100_000_000 {
        let m = random_u64_fewbits();
        if ((mask.wrapping_mul(m)) & 0xFF00000000000000).count_ones() < 6 { continue; }
        if test_magic(square, m, mask, is_rook) { return m; }
    }
    panic!("No magic found for square {}", square);
}

#[derive(Clone, Copy)]
struct Magic { mask: u64, magic: u64, shift: u8, offset: u32 }

fn main() {
    println!("Generating...");
    let t = std::time::Instant::now();
    
    let mut rm = Vec::new();
    let mut off = 0;
    for sq in 0..64 {
        let mask = generate_rook_mask(sq);
        let nb = mask.count_ones();
        rm.push(Magic { mask, magic: find_magic(sq, true), shift: (64 - nb) as u8, offset: off });
        off += 1 << nb;
    }
    println!("\nRook: {} entries", off);
    
    let mut bm = Vec::new();
    off = 0;
    for sq in 0..64 {
        let mask = generate_bishop_mask(sq);
        let nb = mask.count_ones();
        bm.push(Magic { mask, magic: find_magic(sq, false), shift: (64 - nb) as u8, offset: off });
        off += 1 << nb;
    }
    println!("\nBishop: {} entries", off);
    println!("Done in {:.1}m\n", t.elapsed().as_secs_f64() / 60.0);
    
    println!("pub static ROOK_MAGICS: [Magic; 64] = [");
    for m in &rm {
        println!("    Magic {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {:2}, offset: {:6} }},", m.mask, m.magic, m.shift, m.offset);
    }
    println!("];\n");
    
    println!("pub static BISHOP_MAGICS: [Magic; 64] = [");
    for m in &bm {
        println!("    Magic {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {:2}, offset: {:6} }},", m.mask, m.magic, m.shift, m.offset);
    }
    println!("];");
}
