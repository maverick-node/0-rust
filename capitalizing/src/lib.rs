pub fn capitalize_first(input: &str) -> String {
    let mut res = String::from(input);
    let mut res1 = String::new();
    res = res.to_lowercase();
    for (i, c) in res.chars().enumerate() {
        if i == 0 {
            let upper = c.to_ascii_uppercase();

            res1.push(upper);
            continue;
        }
        res1.push(c);
    }
    res1
}

pub fn title_case(input: &str) -> String {
    let mut res = String::from(input);
    let mut res1 = String::new();
    res = res.to_lowercase();
    let mut count = 0;
    for (i, c) in res.chars().enumerate() {
        if count == 0 {
            let upper = c.to_ascii_uppercase();

            res1.push(upper);
            count += 1;
            continue;
        }
        if c == ' ' {
            res1.push(' ');
            count = 0;
            continue;
        }
        res1.push(c);
        count += 1;
    }
    res1
}

pub fn change_case(input: &str) -> String {
    let mut res = String::from(input);
    let mut res1 = String::new();
    for (i, c) in res.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let lower = c.to_ascii_lowercase();
            res1.push(lower);
            continue;
        } else {
            let upper = c.to_ascii_uppercase();

            res1.push(upper);
        }
    }
    res1
}
