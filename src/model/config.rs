use serde::Deserialize;
#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub setting: Option<Setting>,
    pub database: Database,
    pub jwt: Jwt,
}

//--------------------------------------

#[derive(Deserialize, Debug, Default)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i64,
    pub dbname: String,
}

//--------------------------------------

#[derive(Deserialize, Debug, Clone)]
pub struct Setting {
    #[serde(default = "default_setting_host")]
    pub host: String,
    #[serde(default = "default_setting_port")]
    pub port: u16,
}

fn default_setting_host() -> String {
    super::constance::DEFAULT_HOST.to_string()
}

fn default_setting_port() -> u16 {
    super::constance::DEFAULT_PORT
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            host: default_setting_host(),
            port: default_setting_port(),
        }
    }
}

//--------------------------------------

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Jwt {
    pub token_secret: String,
    #[serde(default = "default_jwt_expiration_time")]
    pub expiration_time: i64,
}

fn default_jwt_expiration_time() -> i64 {
    super::constance::DEFAULT_JWT_EXPIRATION_TIME_HOUR
}
