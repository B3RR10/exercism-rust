use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3)
        .into_par_iter()
        .filter_map(|a| {
            let b = sum / 2 * (sum - 2 * a) / (sum - a);
            let c = sum - a - b;
            if a < b && b < c && c.pow(2) == a.pow(2) + b.pow(2) {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
