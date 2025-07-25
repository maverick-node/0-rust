pub fn is_empty(v: &str) -> bool {
    if v.len() == 0 {
        return true;
    }
    false
}

pub fn is_ascii(v: &str) -> bool {
    let mut re = String::from(v);
    let mut count = 0;
    for i in re.chars() {
        count += 1;
    }
    if count == v.len() {
        return true;
    }
    return false;
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {
        return true;
    }
    return false;
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[0..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    let mut count = 0;
    for i in v.chars() {
        if i == pat {
            return count;
        }
        count += 1;
    }
    return 0;
}
