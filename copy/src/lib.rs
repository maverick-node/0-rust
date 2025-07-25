pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let de = c as f64;
    let exp = de.exp();
    let log = de.ln();

    (c, exp, log)
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for i in a.chars() {
        if i == ' ' {
            res.push_str(" ");
            continue;
        }
        let num: f64 = i.to_digit(10).unwrap() as f64;
        let exp = num.exp();
        res.push_str(&exp.to_string());
    }
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res: Vec<f64>= vec![];
    for i in &b {
        let num = *i as f64;
            let log = num.ln();
       res.push(log)
    }
    (b,res)

}
