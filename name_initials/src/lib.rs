pub fn initials(names: Vec<&str>) -> Vec<String> {
    // let mut s: Vec<String> = names
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect();
    // let mut res: Vec<String> = vec![];
    // for i in s.iter() {
    //     for (j, c) in i.chars().enumerate() {
    //         if let Some(c) = i.chars().next() {
    //             res.push(c.to_string());
    //             continue
    //         }
    //         if c == ' '{
    //              res.push(c.to_string());
    //         }
    //     }
     
    // }
    // println!("{:?}", res);
    // s
    let mut res: Vec<String> = vec![];
    
    for name in &names {
        let mut index = 0;
        let mut start = true;
        let mut n = String::new();
        for chars in name.trim().chars() {
            if chars == ' ' {
                index = 0;
                continue
            }
            if index == 0 {
                n.push(chars);
                n.push('.');
                if start {
                    n.push(' ');
                    start = false;
                }
                index += 1;
            }
        }
        res.push(n);
    }
    res
}
