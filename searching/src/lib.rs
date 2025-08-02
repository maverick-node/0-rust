pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut res = array.len()-1;
    for i in array.iter().rev(){
        if key == *i{
            return Some(res)
        }
        res-=1;
    }
    Some(res)
}