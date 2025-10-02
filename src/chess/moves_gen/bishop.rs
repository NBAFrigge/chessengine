use crate::chess::moves_gen::hyperbola_quint::hyp_quint;
use crate::chess::moves_gen::hyperbola_quint::DIAG;
use crate::chess::moves_gen::hyperbola_quint::ANTI_DIAG;

pub fn moves(b :u64, occ: u64) -> u64{
    let index = b.trailing_zeros();
    let tr = index as usize / 8;
    let tf = index as usize % 8;

    let diag_index: usize = 7 + tr - tf;
    let anti_diag_index: usize = tr + tf;

    hyp_quint(index as u64, occ, DIAG[diag_index]) | hyp_quint(index as u64, occ, ANTI_DIAG[anti_diag_index])
}