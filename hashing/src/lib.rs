use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let sum:i32 = list.iter().sum();
    let lenw = list.len() as f64;
    let avgw: f64 = sum as f64/lenw;
    avgw 

}

pub fn median(list: &[i32]) -> i32 {
        let mut sort = list.to_vec();
   sort.sort();
   let midlle = sort.len()/2;
   if sort.len()%2==0{
     (sort[midlle - 1] + sort[midlle] )/ 2
   }else {
    sort[midlle]
   }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut has = HashMap::new();
    let mut re = list.to_vec();
    let mut count = 0;
    for i in &re {
     for j in &re{
        if i == j {
            count+= 1;
        }
     }
     has.insert(i, count);
     count= 0;
    }
    let mut mode = re[0];
    let mut max_count = 0;
    for (key, val) in has {
        if val > max_count {
            max_count = val;
            mode = *key;
        }
    }

    mode

   
}