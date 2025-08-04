use diamond_creation::*;

fn main() {

    println!("{:?}", get_diamond('D'));
    for line in get_diamond('C') {
        println!("{}", line);
    }
}