use uuid::Uuid;

use lib_auth::token::{error::AuthError, generate_web_token};
use tower_cookies::{Cookie, Cookies};

pub const AUTH_TOKEN: &str = "auth-token";

pub fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<(), AuthError> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

pub fn remove_token_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
}
