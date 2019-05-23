use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // a^2 + b^2 = c^2
    // a + b + c = N
    // -> 0 = N^2 - 2N(a+b) + 2ab, c = N - (a+b)

    let mut result: HashSet<[u32; 3]> = HashSet::new();

    for a in 1..(sum / 3) {
        for b in a..(2 * sum / 3) {
            if sum <= a + b {
                break;
            }
            let c = sum - (a + b);
            if c * c == a * a + b * b && a < b && b < c {
                result.insert([a, b, c]);
            }
        }
    }
    result
}
