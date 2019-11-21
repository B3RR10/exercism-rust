use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3)
        .into_par_iter()
        .filter_map(|side_a: u32| {
            let side_b: u32 = sum / 2 * (sum - 2 * side_a) / (sum - side_a);
            let side_c: u32 = sum - side_a - side_b;
            if side_a < side_b && side_c.pow(2) == side_a.pow(2) + side_b.pow(2) {
                Some([side_a, side_b, side_c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
