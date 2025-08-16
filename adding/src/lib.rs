pub fn add_curry(s: i64)-> impl Fn(i64) -> i64{
let res = move |a| a+s;
res
}