pub fn moves(knights: u64) -> u64 {
    const NOT_A: u64 = 0xfefefefefefefefe;
    const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;
    const NOT_AB: u64 = 0xfcfcfcfcfcfcfcfc;
    const NOT_GH: u64 = 0x3f3f3f3f3f3f3f3f;

    ((knights << 15) & NOT_H)
        | ((knights << 17) & NOT_A)
        | ((knights << 6) & NOT_GH)
        | ((knights << 10) & NOT_AB)
        | ((knights >> 10) & NOT_GH)
        | ((knights >> 6) & NOT_AB)
        | ((knights >> 17) & NOT_H)
        | ((knights >> 15) & NOT_A)
}
