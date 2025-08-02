pub fn num_to_ordinal(x: u32) -> String {
    let mut st = x.to_string();

    if st.ends_with("1") {
        st.push_str("st");
    }else if st.ends_with("2") {
        st.push_str("nd");
    }else if st.ends_with("3") {
        st.push_str("rd");
    } else {
        st.push_str("th");
    }
    st
}
