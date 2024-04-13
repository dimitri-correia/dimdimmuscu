pub mod methods;
pub mod structs;

const USERS_TABLE: &str = "users";
const USERS_TABLE_COL: [&str; 4] = ["id", "name", "birthdate", "account_creation"];
const USERS_AUTH_TABLE: &str = "users_auth";
const USERS_AUTH_TABLE_COL: [&str; 2] = ["profile_id", "pwd"];
const SESSION_TABLE: &str = "session";
const SESSION_TABLE_COL: [&str; 3] = ["token", "profile_id", "until"];
