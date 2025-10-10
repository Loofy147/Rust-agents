use anyhow::Result;
use serde::Deserialize;

/// Represents the application's configuration.
///
/// This struct is loaded from a configuration file and contains settings
/// for the LLM, such as the API key and model name.
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    /// The settings for the Language Model.
    pub llm: Llm,
}

/// Represents the configuration for the Language Model.
#[derive(Debug, Deserialize, Clone)]
pub struct Llm {
    /// The API key for the LLM service.
    pub api_key: String,
    /// The name of the model to use.
    pub model: String,
}

impl Settings {
    /// Loads the application settings from a configuration file.
    ///
    /// The configuration is loaded from a file named `config.toml` in the
    /// current directory.
    pub fn new() -> Result<Self> {
        let s = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}