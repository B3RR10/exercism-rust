fn is_prime(x: u32, primes: &[u32]) -> bool {
    let vec: Vec<&u32> = primes.into_iter().filter(|p| x % *p == 0).collect();
    vec.len() == 0
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];

    let mut i = 3;

    while primes.len() <= n as usize {
        if is_prime(i, &primes) {
            primes.push(i.clone())
        }
        i += 1;
    }

    primes[n as usize]
}
