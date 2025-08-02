use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    let mut count = 0;
    let mut map = HashMap::new();
    for i in s.chars() {
        if i.is_ascii_alphabetic(){

            map.insert(i,0);
        }
    }
    if map.len() == 26 {
        return true;
    }
    false
}
