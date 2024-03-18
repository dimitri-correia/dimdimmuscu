use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum SchemeError {
    Key,
    Salt,
    Hash,
    PwdValidate,
    SchemeNotFound(String),
}
