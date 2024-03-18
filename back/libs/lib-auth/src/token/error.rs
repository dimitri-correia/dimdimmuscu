use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AuthError {
    HmacFailNewFromSlice,
    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,
    FailToDateParse,
}
