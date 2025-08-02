pub fn search(array: &[i32], key: i32) -> Option<usize> {
        
 if array.len() == 0 {
        return None;
    }
    let mut res = array.len() - 1;
   

    let rr = array.contains(&key);
    if !rr {
        return None;
    }

    loop {
        for i in array.iter().rev() {
            if res == 0 {
                return Some(0);
            }
            if key == *i {
                return Some(res);
            }
            res -= 1;
        }
        return None;
    }
}
