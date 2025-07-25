pub fn to_url(s: &str) -> String {
    let res = String::from(s);
    let mut re = String::new();
    for i in res.chars(){
        if i == ' '{
            re.push_str("%20");
        }else{
         re.push(i);

        }
    }
re
}