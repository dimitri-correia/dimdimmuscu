use serde::Serialize;

use crate::pwd::scheme::error::SchemeError;

#[derive(Debug, Serialize)]
pub enum PwdError {
    PwdWithSchemeFailedParse,
    FailSpawnBlockForValidate,
    FailSpawnBlockForHash,
    SchemeError(SchemeError),
}
