pub fn fibonacci(n: u32) -> u32 {
      if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut res=0;
    let mut a = 0;
    let mut b = 1;

    for i in 2..=n {
      let tt = a+b;
        a = b;
        b = tt;
    }
    return b
}