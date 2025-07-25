pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut count = 0;
    for i in s.chars() {
        if count > 0 && i != '-' && i != '+' {
            count -= 1;
            continue;
        } else if i == '-' {
            res.pop();
        } else if i == '+' {
            count += 1;
        } else {
            res.push(i);
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut [String]) {
 let mut res = String::new();
     for i in v {
        if i.contains("+") {
            let sp: Vec<&str> = i.split(|x| x == '+').collect();
            let part1 = sp[0].parse::<i32>().unwrap_or(0);
            let part2 = sp[1].parse::<i32>().unwrap_or(0);
            res.push_str(&(part1+part2).to_string())
        }
        if i.contains("-") {
            let sp: Vec<&str> = i.split(|x| x == '-').collect();
            let part1 = sp[0].parse::<i32>().unwrap_or(0);
            let part2 = sp[1].parse::<i32>().unwrap_or(0);
            res.push_str(&(part1-part2).to_string())
        }

        *i = res.clone();
        res.clear();
    }

}
