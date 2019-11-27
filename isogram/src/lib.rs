pub fn check(candidate: &str) -> bool {
    let vec = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();
    let mut dedup_vec: Vec<char> = vec.clone();
    dedup_vec.sort();
    dedup_vec.dedup();
    if dedup_vec.len() == vec.len() {
        true
    } else {
        false
    }
}
