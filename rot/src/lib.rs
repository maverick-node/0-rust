pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();

    for i in input.chars() {
        if i.is_ascii_uppercase() {
            let ch = i.to_ascii_lowercase();
            let a = b'a' as i8;
            let c = ch as i8;
            let shif = key as i8;
            let cal = (((c - a + 26 + key) % 26) + a) as u8;
            let gg = cal as char;
            res.push_str(&gg.to_string().to_ascii_uppercase());
        }else if i.is_ascii_lowercase() {
            let ch = i.to_ascii_lowercase();
            let a = b'a' as i8;
            let c = ch as i8;
            let shif = key as i8;
            let cal = (((c - a+26 + shif) % 26) + a) as u8;
            let gg = cal as char;
            res.push_str(&gg.to_string());
        }else{
            res.push(i)
        }
    }
    res
}
