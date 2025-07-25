pub fn initials(names: Vec<&str>) -> Vec<String> {
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
