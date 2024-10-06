use std::env::VarError;
use std::num::ParseIntError;

pub enum EnvVariableError {
    EnvVariableNotSetup(VarError),
    FailingParsing(ParseIntError),
}
