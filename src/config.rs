use std::fs;

use serde::Deserialize;

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub(crate) server: ServerConfig,
    pub(crate) auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub(crate) port: u16,
    pub(crate) db_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    pub(crate) private_key: String,
    pub(crate) public_key: String,
}

impl AppConfig {
    pub fn load(path: String) -> Result<Self, AppError> {
        let config_content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::AppConfig;
    #[test]
    fn load_app_config_work() -> Result<()> {
        let config = AppConfig::load("config.toml".to_string())?;
        assert_eq!(config.server.port, 8686 as u16);
        assert_eq!(
            config.server.db_url,
            "postgres://db_manager:super_admin8801@localhost:5432/todolist"
        );

        let test_private_key = r#"-----BEGIN PRIVATE KEY-----
test
-----END PRIVATE KEY-----"#;
        assert_eq!(config.auth.private_key, test_private_key);

        let test_pub_key = r#"-----BEGIN PUBLIC KEY-----
test
-----END PUBLIC KEY-----"#;
        assert_eq!(config.auth.public_key, test_pub_key);

        Ok(())
    }
}
