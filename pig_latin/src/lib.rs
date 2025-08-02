pub fn pig_latin(text: &str) -> String {
    let vowels = vec!['A', 'E', 'I', 'O', 'U'];
    let first = text.chars().nth(0).unwrap() as char;
    let second = text.chars().nth(1).unwrap() as char;

    let third = text.chars().nth(2).unwrap() as char;

    if vowels.contains(&first.to_ascii_uppercase()) {
        return text.to_owned() + &"ay".to_string();
    }
    let mut res = String::new();

    let mut n = 0;
    for i in text.chars() {
        if n == 0 && first == 'q' && second == 'u' {
            res.push_str("q");
            n += 1;
            break;
        }
        if !vowels.contains(&i.to_ascii_uppercase()) && second == 'q' && third == 'u' {
            res.push(i);
            res.push_str("qu");
            n += 3;
            break;
        }
        if vowels.contains(&i.to_ascii_uppercase()) {
            break;
        }

        if !vowels.contains(&i.to_ascii_uppercase()) {
            res.push(i);
        }

        n += 1;
    }

    let r = &text[n..];
    r.to_owned() + &res + &"ay".to_string()
}
