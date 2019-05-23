fn get_factors(n: u32) -> Vec<u32> {
    let mut factors = vec![];
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
}

fn get_sounds(f: &[u32]) -> (u8, u8, u8) {
    let mut tuple: (u8, u8, u8) = (0, 0, 0);
    for x in f {
        match x {
            3 => tuple.0 = 1,
            5 => tuple.1 = 1,
            7 => tuple.2 = 1,
            _ => {}
        }
    }
    tuple
}

fn build_string(s: (u8, u8, u8), n: u32) -> String {
    if s == (0, 0, 0) {
        n.to_string()
    } else {
        let mut sound = String::new();
        if s.0 == 1 {
            sound += "Pling";
        }
        if s.1 == 1 {
            sound += "Plang";
        }
        if s.2 == 1 {
            sound += "Plong";
        }
        sound
    }
}

pub fn raindrops(n: u32) -> String {
    let factors = get_factors(n);
    let sounds = get_sounds(&factors);
    build_string(sounds, n)
}
