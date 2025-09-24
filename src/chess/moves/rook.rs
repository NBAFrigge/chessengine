use crate::chess::moves::hyperbola_quint::hyp_quint;
use crate::chess::moves::hyperbola_quint::FILES;
use crate::chess::moves::hyperbola_quint::RANKS;

pub fn moves(b: u64, occ: u64) -> u64 {
    let index = b.trailing_zeros();
    let tr = index as usize / 8;
    let tf = index as usize % 8;

    hyp_quint(index as u64, occ, RANKS[tr]) | hyp_quint(index as u64, occ, FILES[tf])
}