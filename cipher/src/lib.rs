#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected : String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let err = CipherError { expected : ciphered.to_string()};
    let mut res = Vec::new();
    for r in original.chars() {
        let mut min = 0;
        let mut max = 0;
        if r as u8 >= 65 && r as u8 <= 90 {
            min = 65;
            max = 90;
            res.push(((max-((r as u8)-min)) as char).to_string())
        } else if r as u8 >= 97 && r as u8 <= 122 {
            min = 97;
            max = 122;
            res.push(((max-((r as u8)-min)) as char).to_string())
        } else {
            res.push(r.to_string())
        }
    }
    if res.join("") != ciphered {
        return Err(err)
    }
    Ok(())
}
