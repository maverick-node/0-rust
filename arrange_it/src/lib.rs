pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    let mut res = String::new();
    words.sort_by_key(|w| w.chars().find(|c| c.is_ascii_digit()));
    for word in words {
        for c in word.chars() {
            if !c.is_ascii_digit() {
                res.push(c);
            }
        }
        res.push(' ');
    }
   res.trim().to_string()

}
