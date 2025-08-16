pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vc: Vec<usize> = Vec::new();
    let len = arr.len();
    let mut count = 0;
    loop {
        let mut res = 1;
        for (t, i) in arr.iter().enumerate() {
       
            if t != count {
                res *= i;
            }
           
        }
        if count >= len {
            break;
        }
        count += 1;
        vc.push(res);
    }
    vc
}
