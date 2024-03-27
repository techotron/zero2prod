#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension that `config`
    // knows how to parse: yaml, json etc.
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();

    // Try to convert the configuration values it read into our settings type
    settings.try_deserialize()
}
