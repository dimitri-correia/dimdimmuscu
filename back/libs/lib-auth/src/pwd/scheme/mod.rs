use enum_dispatch::enum_dispatch;

use crate::pwd::error::PwdError;
use crate::pwd::scheme::error::SchemeError;
use crate::pwd::ContentToHash;

pub mod error;
mod scheme_01;
mod scheme_02;

pub const DEFAULT_SCHEME: &str = "02";

#[derive(Debug)]
pub enum SchemeStatus {
    Ok,
    Outdated,
}

#[enum_dispatch]
pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String, PwdError>;

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<(), PwdError>;
}

#[enum_dispatch(Scheme)]
pub enum SchemeDispatcher {
    Scheme01(scheme_01::Scheme01),
    Scheme02(scheme_02::Scheme02),
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme, PwdError> {
    match scheme_name {
        "01" => Ok(SchemeDispatcher::Scheme01(scheme_01::Scheme01)),
        "02" => Ok(SchemeDispatcher::Scheme02(scheme_02::Scheme02)),
        _ => Err(PwdError::SchemeError(SchemeError::SchemeNotFound(
            scheme_name.to_string(),
        ))),
    }
}
