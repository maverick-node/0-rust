use arrays::*;

fn main() {
    let a = [10;10];
    let b = [50;10];

    println!("The sum of the elements in {:?} is {}", a, sum(a));
    println!("The sum of the elements in {:?} is {}", b, sum(b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}