use crate::mode::Mode;
use std::str::FromStr;

pub struct Config<T> {
    pub mode: Mode<T>,
    pub value: Option<String>
}

impl<T> Config<T> {
    pub fn new(mut args: impl Iterator<Item = String> + ExactSizeIterator) -> Result<Config<T>, String> {
        if args.len() < 1 {
            return Err("Insufficent number of arguments".into());
        }

        let env_mode = match args.next() {
            Some(x) => String::from(x),
            None => return Err("Mode must be supplied".into()),
        };

        fn mode_type<T>(mode: &str) -> Result<Mode<T>, String> {         
            //Result<Mode<String>, ParseError>
            match Mode::<T>::from_str(&mode) {
                Ok(x) => Ok(x),
                _ => Err(String::from("")),
            }
        }

        let env_value = args.next();

        Ok(Config {mode: mode_type(&env_mode)?, value: env_value})
    }
}