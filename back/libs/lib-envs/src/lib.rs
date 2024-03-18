use std::env;
use std::str::FromStr;

use lib_b64::b64u_decode;

pub fn get_env(name: &'static str) -> Result<String, EnvError> {
    env::var(name).map_err(|_| EnvError::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T, EnvError> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| EnvError::WrongFormat(name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>, EnvError> {
    b64u_decode(&get_env(name)?).map_err(|_| EnvError::WrongFormat(name))
}

#[derive(Debug)]
pub enum EnvError {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_exists() {
        env::set_var("TEST_ENV", "value");

        assert_eq!(get_env("TEST_ENV").unwrap(), "value".to_string());
    }
}
