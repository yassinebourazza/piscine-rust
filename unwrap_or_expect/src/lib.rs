#[derive(Debug)]
pub enum Security {
    Warning,
    NotFound,
    Unknown,
    Message,
    UnexpectedUrl,
}

pub fn fetch_data(data: Result<&str, &str>, security: Security) -> String {
     match security_level {
        Security::Unknown => server.unwrap().to_string(),

        Security::Message => server.expect("ERROR: program stops").to_string(),

        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),

        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },

        Security::UnexpectedUrl => server.unwrap_err().to_string(),
    }
}