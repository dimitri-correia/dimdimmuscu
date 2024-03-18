use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use lib_auth::token::{generate_web_token, TokenError};

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<(), TokenError> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<(), TokenError> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
