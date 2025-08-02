pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut res = array.len() - 1;
    loop {
        for i in array.iter().rev() {
            if res <= 0 {
                return None
            }
            if key == *i {
                return Some(res);
            }
            res -= 1;
        }

        
    }

   
}
