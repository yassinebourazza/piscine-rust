#[derive(Debug)]
pub enum Security {
    Warning,
    NotFound,
    //Unknown,
    Message,
    //UnexpectedUrl,
}

pub fn fetch_data(data: Result<&str, &str>, security: Security) -> String {
    match security {

        Security::Warning => {
            data.unwrap().to_string()
        }

        Security::Message => {
            data.expect("ERROR: program stops").to_string()
        }

         Security::NotFound => {
            let url = data.unwrap();

            if url == "malicious_server.com" {
                panic!("{}", url);
            }

            url.to_string()
        }

    }
}