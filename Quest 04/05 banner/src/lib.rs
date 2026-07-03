use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug)]
pub struct Flag {
   short_hand: String,
    long_hand: String,
    desc: String
}


impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let f = name.chars().next().unwrap();
        Self {
            short_hand : format!("-{}",f),
            long_hand : format!("--{}", name),
            desc : d.to_string()
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand,func);
        self.flags.insert(flag.long_hand,func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if argv.len() < 2 {
            return Err("The callback should be executed with two arguments".to_string());
        }
        match self.flags.get(input) {
            Some(fun) => fun(argv[0],argv[1]).map_err(|e| e.to_string()),
            None => Err("invalid float literal".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let res = a.parse::<f64>()? / b.parse::<f64>()?;
    Ok(res.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let res = a.parse::<f64>()? % b.parse::<f64>()?;
    Ok(res.to_string())
}
