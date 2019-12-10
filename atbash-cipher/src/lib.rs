use std::char;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    reverse_ascii_string(plain)
        .enumerate()
        .flat_map(|(i, c)| format(i, c))
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    reverse_ascii_string(cipher).collect::<String>()
}

fn format(i: usize, c: char) -> impl Iterator<Item = char> {
    if i != 0 && i % 5 == 0 {
        Some(' ').into_iter().chain(Some(c))
    } else {
        Some(c).into_iter().chain(None)
    }
}

fn reverse_ascii_string<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
    input.chars().filter(char::is_ascii_alphanumeric).map(|c| {
        if c.is_ascii_alphabetic() {
            (b'z' - c.to_ascii_lowercase() as u8 + b'a') as char
        } else {
            c
        }
    })
}
