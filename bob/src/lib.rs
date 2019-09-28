pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let question = msg.chars().last() == Some('?');
    let msg_chars: Vec<char> = msg.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let yell = msg_chars.len() > 0 && msg_chars.iter().all(|c| c.is_ascii_uppercase());

    if question && yell {
        return "Calm down, I know what I'm doing!";
    }
    if question {
        return "Sure.";
    }
    if yell {
        return "Whoa, chill out!";
    }
    "Whatever."
}
