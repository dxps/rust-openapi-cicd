use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub http: HttpSettings,
}

#[derive(Deserialize)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either local or production"
            )),
        }
    }
}

/// Get the application config.
pub fn get_config() -> Result<AppConfig, config::ConfigError> {
    // In the `./config` directory we have 3 .yaml files: base, local, and production.
    let base_dir = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_dir.join("config");

    let base_src =
        config::File::with_name(config_dir.join("base").to_str().unwrap()).required(true);

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let env_src =
        config::File::with_name(config_dir.join(env.as_str()).to_str().unwrap()).required(true);

    config::Config::builder()
        // Load the config.(yml|yaml|toml|...) file.
        .add_source(base_src)
        .add_source(env_src)
        .build()
        .unwrap()
        .try_deserialize()
}
