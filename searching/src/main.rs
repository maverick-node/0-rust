use searching::*;

fn main() {
    let ar = [];
    let f = search(&ar, 8);
    println!(
        "the element 8 is last in the position {:?} in the array {:?}",
        f, ar
    );
}