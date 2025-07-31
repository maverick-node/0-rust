#[derive(Debug, PartialEq)]
pub struct CipherError {
   pub expected: String,
}
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res = String::new();
    for i in original.chars() {
        if i.is_alphabetic() {
            if i.is_uppercase() {
                let a = 'a' as u8;
                let pos = (i.to_ascii_lowercase() as u8) - a + 1;
                let final_pos = 27 - pos;
                let cipher_char = (a + final_pos - 1) as char;
                res.push_str(&cipher_char.to_string().to_uppercase());
            }
            if i.is_lowercase() {
                let a = 'a' as u8;
                let pos = (i.to_ascii_lowercase() as u8) - a + 1;
                let final_pos = 27 - pos;
                let cipher_char = (a + final_pos - 1) as char;
                res.push_str(&cipher_char.to_string().to_lowercase());
            }
        } else {
            res.push_str(&i.to_string());
        }
    }
    if res == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: res.to_string(),
        })
    }
}
