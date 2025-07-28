use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
   let mut res: HashMap<&'a str, usize> = HashMap::new();
let mut ss = words.to_vec();
let mut count=0;
for i in &ss {
for j in &ss{
    if i.to_lowercase() == j.to_lowercase(){
        count+=1;
    }
}
res.insert(i, count);
count=0
}


res


}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count=0;
for i in frequency_count.keys(){
    count+=1

}
count
}