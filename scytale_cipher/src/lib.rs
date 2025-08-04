pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.len() == 0 || i == 0{
        return "".to_string()
    }
    let rows = (message.len() + i - 1) / i;

    let mut vac = vec![vec![' ';i]; rows];

    for (index, ch) in message.chars().enumerate() {
        let row = index / i;
        let col = index % i;
        vac[row][col] = ch;
    }

    let mut res = String::new();
    for col in 0..i {
        for row in 0..rows {
            res.push(vac[row][col]);
        }
    }

    res.trim_end().to_string()
}
