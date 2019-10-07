pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut rest = n;
    let mut iter = 2;

    while factors.iter().product::<u64>() != n {
        if rest % iter == 0 {
            factors.push(iter);
            rest /= iter;
        } else {
            iter += 1;
        }
    }

    factors
}
