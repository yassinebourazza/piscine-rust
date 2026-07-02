#[derive(Debug)]
pub enum Security {
    Warning,
    NotFound,
    Unknown,
    Message,
    UnexpectedUrl,
}

pub fn fetch_data(data: Result<&str, &str>, security: Security) -> String {
     match security {
        Security::Unknown => data.unwrap().to_string(),

        Security::Message => data.expect("ERROR: program stops").to_string(),

        Security::Warning => data.unwrap_or("WARNING: check the server").to_string(),

        Security::NotFound => match data {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },

        Security::UnexpectedUrl => data.unwrap_err().to_string(),
    }
}