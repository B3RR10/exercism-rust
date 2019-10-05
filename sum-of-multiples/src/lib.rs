use rayon::prelude::*;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .into_par_iter()
        .filter(|x| factors.iter().any(|y| y != &0 && x % y == 0))
        .sum()
}
