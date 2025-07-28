use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = Vec::new();
    for i in h.values(){
        res.push(i);
    }
    **res.iter().max().unwrap()
}