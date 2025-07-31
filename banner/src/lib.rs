use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let s = format!("-{}", name.chars().next().unwrap());
        let l = format!("--{}", name);
        let de = d.to_string();
        Flag {
            short_hand: s,
            long_hand: l,
            desc: de,
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if argv.len() < 2 {
            return Err("not enough arguments".to_string());
        }
        if self.flags.contains_key(input) {
            let callback = self.flags.get(input).unwrap();

            match callback(argv[0], argv[1]) {
                Ok(val) => Ok(val),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("flag not found".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f64>()?;
    let b = b.parse::<f64>()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f64>()?;
    let b = b.parse::<f64>()?;
    Ok((a % b).to_string())
}
