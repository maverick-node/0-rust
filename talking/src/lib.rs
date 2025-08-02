pub fn talking(text: &str) -> &str {
    let trm = text.trim();
    if trm.len() == 0 {
        return "Just say something!";
    }
    let mut count = 0;
    for i in text.chars() {
        if
            i.is_uppercase() ||
            i == ' ' ||
            i == '!' ||
            i == '\'' ||
            i.is_numeric() ||
            i == ','
        {
            count += 1;
        }
    }
    if count == text.len() {
        return "There is no need to yell, calm down!";
    }

    count = 0;
    for i in text.chars() {
        if i.is_uppercase() || i == ' ' || i == '?' || i == '\'' {
            count += 1;
        }
    }
    if count == text.len() && text.contains("?") {
        return "Quiet, I am thinking!";
    }

    if text.ends_with("?") {
        return "Sure.";
    }

    "Interesting"
}
