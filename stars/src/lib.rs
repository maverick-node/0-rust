pub fn stars(n: u32) -> String {
    let pows = (2_i32).pow(n);

    let mut res = String::new();
    for i in 0..pows {
        res.push('x');
    }
    res
}
