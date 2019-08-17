pub fn reply(message: &str) -> &str {
    if message.trim() == "" {
        return "Fine. Be that way!";
    }

    let message_vec: Vec<char> = message.trim().chars().collect();
    let last = message_vec.last();
    if last == Some(&'?') {
        if is_shouting(&message_vec) {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else if is_shouting(&message_vec) {
        return "Whoa, chill out!";
    }
    return "Whatever.";
}

fn is_shouting(msg: &Vec<char>) -> bool {
    let letters = msg
        .clone()
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    !letters.is_empty()
        && &letters
            .into_iter()
            .filter(|l| l.is_uppercase())
            .collect::<Vec<char>>()
            .len()
            == &letters.len()
}
