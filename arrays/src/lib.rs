pub fn sum(a: &[i32]) -> i32 {
    let mut r = 0;
    for v in a {
        r += v;
    }
    r
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10;32]
}