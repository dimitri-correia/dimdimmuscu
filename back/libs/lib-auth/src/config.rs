use std::sync::OnceLock;

use lib_envs::{get_env_b64u_as_u8s, get_env_parse, EnvError};

pub fn auth_config() -> &'static AuthConfig {
    static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AuthConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct AuthConfig {
    pub PWD_KEY: Vec<u8>,

    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_MS: i64,
}

impl AuthConfig {
    fn load_from_env() -> Result<AuthConfig, EnvError> {
        Ok(AuthConfig {
            PWD_KEY: get_env_b64u_as_u8s("DIMDIMMUSCU_SERVICE_PWD_KEY")?,
            TOKEN_KEY: get_env_b64u_as_u8s("DIMDIMMUSCU_SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_MS: get_env_parse("DIMDIMMUSCU_SERVICE_TOKEN_DURATION_MS")?,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use lib_envs::get_env;

    #[test]
    fn setup_env() {
        assert_eq!(
            get_env("DIMDIMMUSCU_SERVICE_PWD_KEY").unwrap(),
            "CKUGFOD9_2Qf6Pn3ZFRYgPYb8ht4vKqEG9PGMXTB7497bT0367DjoaD6ydFnEVaIRda0kKeBZVCT5Hb62m2sCA".to_string());
        assert_eq!(
            get_env("DIMDIMMUSCU_SERVICE_TOKEN_KEY").unwrap(),
            "9FoHBmkyxbgu_xFoQK7e0jz3RMNVJWgfvbVn712FBNH9LLaAWS3CS6Zpcg6RveiObvCUb6a2z-uAiLjhLh2igw");
        assert_eq!(
            get_env("DIMDIMMUSCU_SERVICE_TOKEN_DURATION_MS").unwrap(),
            "1800000"
        );
    }
}
