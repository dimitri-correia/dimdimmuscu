pub enum LoginError {
    LoginFailUsernameNotFound,
    LoginFailPwdNotMatching { user_id: i32 },
}
