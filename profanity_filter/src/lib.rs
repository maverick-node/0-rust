pub fn check_ms(message: &str) -> Result<&str, &str> {
    let mut ve: Vec<String> = Vec::new();

    for i in message.split_whitespace() {
        ve.push(i.to_string());
    }
    if ve.contains(&"stupid".to_string()) {
        return Err("ERROR: illegal");
    }

    if ve.is_empty() {
        return Err("ERROR: illegal");
    }

    Ok(message)
}

pub struct Message {
    msg: String,
}

impl Message {
    pub fn new(st: String, st2: String) -> String {
        if st == "" || st2 == "" {
            return "".to_string();
        }
        st + " " + &st2
    }
}
