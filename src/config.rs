pub mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;

    #[derive (Serialize, Deserialize)]
    pub struct Somestruct{
        pub api_addr: String,
        pub redis_addr: String,
    }

    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}
