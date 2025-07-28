use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str)-> bool {
    let mut first: Vec<String> = Vec::new();
   let mut second = Vec::new();
    for i in s1.chars(){
        first.push(i.to_string());
    }
     for i in s2.chars(){
        second.push(i.to_string());
    }
    let mut has = HashMap::new();
     let mut has2 = HashMap::new();
let mut count=0;
    for i in &first{
        for k in &first{
            if i.to_lowercase() == k.to_lowercase(){
                count+=1
            }
        }
        has.insert(i,count);
        count=0
    }
    for i in &second{
        for k in &second{
            if i.to_lowercase() == k.to_lowercase(){
                count+=1
            }
        }
        has2.insert(i,count);
        count=0
    }



for (k,v) in &has{
    for (j,y) in &has2{
        if k == j && v==y {

            return true
        }
    }
}

false

}