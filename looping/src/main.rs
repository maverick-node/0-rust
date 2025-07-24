use std::io;

fn main(){
    const ANSWER: &str = "the letter e";
            let mut input= Default::default(); 
                    let mut count = 1;
    loop{


println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
io::stdin().read_line(&mut input).unwrap();
let trimmed = input.trim();
let lower = input.to_lowercase();
let re = lower.trim();
if re ==ANSWER{
    println!("Number of trials: {}",count);
    break 
}else {
    input = "".to_string();
    count+=1;
}


    }

}