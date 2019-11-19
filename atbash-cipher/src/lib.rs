static ALPHABET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| reverse(c))
        .enumerate()
        .fold(String::new(), |acc, (i, c)| {
            if i != 0 && i % 5 == 0 {
                format!("{} {}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| reverse(c))
        .collect::<String>()
}

fn reverse(c: char) -> char {
    ALPHABET
        .iter()
        .zip(ALPHABET.iter().rev())
        .find_map(|(a, b)| if a == &c { Some(b.to_owned()) } else { None })
        .unwrap_or(c)
}
