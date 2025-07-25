pub fn str_len(s: &str ) -> usize {
    let re = s.chars();
    let mut count = 0;
for i in re {
    count+=1;
}
count
}