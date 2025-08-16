pub fn first_fifty_even_square() -> Vec<i32> {
    
let mut vc:Vec<i32>= Vec::new();
for i in 1..=50{
    let cal = i * 2;
    let r = cal *cal;
    vc.push(r);
}
vc
}