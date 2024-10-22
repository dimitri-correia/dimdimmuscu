use axum::http::{HeaderName, HeaderValue};
use chrono::NaiveDate;
use dimdimmuscu::models::users::{self, RegisterParams};
use loco_rs::{app::AppContext, TestServer};

const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "1234";

pub struct LoggedInUser {
    pub user: users::Model,
    pub token: String,
}

pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let user_input: RegisterParams = RegisterParams {
        name: "dim".to_string(),
        email: USER_EMAIL.to_string(),
        password: USER_PASSWORD.to_string(),
        birthdate: NaiveDate::from_ymd_opt(1999, 5, 5).unwrap(),
        height_in_cm: 182,
    };

    let register_payload = serde_json::json!(user_input);

    //Creating a new user
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;

    //Check data in db
    let user_db = users::Model::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();
    assert_eq!(user_db, user_input);

    //Logging in the user
    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}
