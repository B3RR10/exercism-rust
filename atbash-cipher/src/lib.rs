use std::char;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    reverse_ascii_string(plain)
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
    reverse_ascii_string(cipher).collect::<String>()
}

fn reverse_ascii_string<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
    input.chars().filter(char::is_ascii_alphanumeric).map(|c| {
        if c.is_ascii_alphabetic() {
            char::from_u32('z' as u32 - c.to_ascii_lowercase() as u32 + 'a' as u32)
                .expect("Can't convert to char")
        } else {
            c
        }
    })
}
