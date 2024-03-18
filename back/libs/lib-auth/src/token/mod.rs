use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Local, TimeDelta};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use uuid::Uuid;

use lib_b64::{b64u_decode_to_string, b64u_encode};

use crate::config::auth_config;
use crate::token::error::AuthError;

mod error;

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    // Identifier (username for example).
    pub ident: String,
    // Expiration date in Rfc3339.
    pub exp: String,
    // Signature, base64url encoded.
    pub sign_b64u: String,
}

impl FromStr for Token {
    type Err = AuthError;

    fn from_str(token_str: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(AuthError::InvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| AuthError::CannotDecodeIdent)?,

            exp: b64u_decode_to_string(exp_b64u).map_err(|_| AuthError::CannotDecodeExp)?,

            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token, AuthError> {
    let config = &auth_config();
    _generate_token(user, config.TOKEN_DURATION_MS, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<(), AuthError> {
    let config = &auth_config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

fn _generate_token(
    ident: &str,
    duration_ms: i64,
    salt: Uuid,
    key: &[u8],
) -> Result<Token, AuthError> {
    // -- Compute the two first components.
    let ident = ident.to_string();
    let exp = now_utc_plus_ms_str(duration_ms);

    // -- Sign the two first components.
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: Uuid,
    key: &[u8],
) -> Result<(), AuthError> {
    // -- Validate signature.
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    if new_sign_b64u != origin_token.sign_b64u {
        return Err(AuthError::SignatureNotMatching);
    }

    // -- Validate expiration.
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| AuthError::ExpNotIso)?;

    if origin_exp < Local::now() {
        return Err(AuthError::Expired);
    }

    Ok(())
}

fn now_utc_plus_ms_str(dur: i64) -> String {
    let new_time = Local::now() + TimeDelta::try_milliseconds(dur).unwrap();

    new_time.to_rfc3339()
}

pub fn parse_utc(datetime_str: &str) -> Result<DateTime<Local>, AuthError> {
    DateTime::<Local>::from_str(datetime_str).map_err(|_| AuthError::FailToDateParse)
}

/// Create token signature from token parts
/// and salt.
fn _token_sign_into_b64u(
    ident: &str,
    exp: &str,
    salt: Uuid,
    key: &[u8],
) -> Result<String, AuthError> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| AuthError::HmacFailNewFromSlice)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_token_display_ok() {
        // -- Fixtures
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        // -- Exec & Check
        assert_eq!(fx_token.to_string(), fx_token_str);
    }

    #[test]
    fn test_token_from_str_ok() {
        // -- Fixtures
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        // -- Exec
        let token: Token = fx_token_str.parse().unwrap();

        // -- Check
        assert_eq!(token, fx_token);
    }

    #[test]
    fn test_validate_web_token_ok() {
        // -- Setup & Fixtures
        let fx_user = "user_one";
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453").unwrap();
        let fx_duration_ms = 20;
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_ms, fx_salt, token_key).unwrap();

        // -- Exec
        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, fx_salt);

        // -- Check
        res.unwrap();
    }

    #[test]
    fn test_validate_web_token_err_expired() {
        // -- Setup & Fixtures
        let fx_user = "user_one";
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453").unwrap();
        let fx_duration_sec = 10;
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_sec, fx_salt, token_key).unwrap();

        // -- Exec
        thread::sleep(Duration::from_millis(20));
        let res = validate_web_token(&fx_token, fx_salt);

        // -- Check
        assert!(
            matches!(res, Err(AuthError::Expired)),
            "Should have matched `Err(Error::Expired)` but was `{res:?}`"
        );
    }
}
