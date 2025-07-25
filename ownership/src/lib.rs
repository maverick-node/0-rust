pub fn first_subword(mut s: String) -> String {
    let mut res = String::new();
    for (i,c) in s.chars().enumerate(){
          if (c.is_uppercase()|| c == '_') && i >0 {
            break;
        }
        res.push(c);
      
    }
    res
}
