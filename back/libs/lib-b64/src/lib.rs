use base64::engine::{general_purpose, Engine};

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>, B64Error> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(|_| B64Error::FailToB64uDecode)
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String, B64Error> {
    b64u_decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(B64Error::FailToB64uDecode)
}

#[derive(Debug)]
pub enum B64Error {
    FailToB64uDecode,
}
