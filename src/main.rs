mod chess;

fn main() {
    let b = chess::table::Board::new();
    // println!("{}", b.to_string());
    // let t:u64 = 1<< 45;
    // println!("{}", format!("{:064b}", t));
    // println!("{}", b.bitboard_to_string(t));
    println!("{}", b.bitboard_to_string(b.white_moves_bitboard_bishop()));
}