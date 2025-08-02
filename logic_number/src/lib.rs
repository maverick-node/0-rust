pub fn number_logic(num: u32) -> bool {
    let st = num.to_string();
    let len = st.len();
    let mut res = 0;
    for i in st.chars() {
        let calcul = i.to_digit(10);
        let val = calcul.expect("REASON").pow(len.try_into().unwrap());
        res += val;
    }
    if res == num {
        return true
    }
    false
}
