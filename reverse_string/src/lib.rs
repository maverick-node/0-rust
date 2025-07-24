pub fn rev_str(input: &str) -> String {
    let mut chars = String::new(); 
    for i in input.chars().rev(){
        chars.push(i)
    }

return chars
}