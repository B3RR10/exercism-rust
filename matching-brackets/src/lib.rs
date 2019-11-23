pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if let Some(open) = stack.last() {
                    if dbg!(is_complement(open, &c)) {
                        stack.pop();
                        continue;
                    }
                }
                return false;
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn is_complement(open: &char, close: &char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}
