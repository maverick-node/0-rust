pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::NotFound => match server {
            Ok(s) => s.to_string(),
            Err(e) => format!("Not found: {}", e),
        },
        Security::UnexpectedUrl => server.unwrap_err().to_string(),
    }
}
