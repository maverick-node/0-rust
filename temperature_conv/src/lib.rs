pub fn fahrenheit_to_celsius(f: f64) -> f64 {

    let mut res;
    res = (f-32.0) * 5.0/9.0;
    res

}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
     let mut res;
    res = (c*9.0/5.0) +32.0;
    res
}