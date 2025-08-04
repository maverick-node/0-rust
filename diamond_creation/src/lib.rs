pub fn get_diamond(c: char)-> Vec<String>{

    let pos = (c.to_ascii_lowercase() as u8) - b'a';
    let mut count = 0;
    let mut tania = pos;
    let mut res = Vec::new();
    
    for i in 0..=pos {
        let spaces = " ".repeat((pos - i) as usize);
        let letter = (b'A' + i) as char;
        let mut other_space;
        other_space = " ".repeat(count);
         if count ==0 {
             count+=1;
         }else{
            count+=2;
         }
     if letter == 'A'{
        res.push(format!("{}{}{}", spaces, letter,spaces));
     }else{
         res.push(format!("{}{}{}{}{}", spaces, letter, other_space,letter,spaces));
     }
    }
    
let mut ress = res.clone();
ress.pop(); 
res.extend(ress.iter().rev().cloned());
res
    
}










