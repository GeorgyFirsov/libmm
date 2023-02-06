use std::fs;
use std::path::Path;

use serde_json as sj;
use git2;

use crate::error::{Result, Error, ErrorCategory};
use super::{ 
    MM_GIT_KEY, 
    MM_GIT_USE_DEFAULT_KEY,
    MM_GIT_EMAIL_KEY,
    MM_GIT_NAME_KEY,
};


/// Struct, that provides an interface to configuration of `libmm`.
pub(crate) struct Config {
    /// Internal JSON structure 
    internal: sj::Value,

    /// Default git config
    git_config: git2::Config,
}


impl Config {
    /// Creates a default configuration. Fills all necessary values 
    /// with their defaults. Optional values are omitted.
    pub(crate) fn new() -> Result<Self> {
        //
        // Let's compose a default configuration...
        //
    
        let default_config = sj::json!({
            MM_GIT_KEY: {
                MM_GIT_USE_DEFAULT_KEY: true,
            }
        });

        //
        // ... and wrap it into an instance of `Config`
        //

        Config::from_raw(default_config)
    }


    /// Loads configuration from a file.
    /// 
    /// * `config_file` - path to a file to read configuration from
    pub(crate) fn load(config_file: &Path) -> Result<Self> {
        let file_content = fs::read(config_file)?;

        //
        // Create a JSON instance and config from it
        //

        Config::from_raw(sj::from_slice(file_content.as_slice())?)
    }


    /// Saves a configuration into a file.
    /// 
    /// * `config_file` - path to a file to write config to
    pub(crate) fn save(&self, config_file: &Path) -> Result<()> {
        //
        // Well... Just save it into a file!
        //

        let json = sj::to_string_pretty(&self.internal)?;
        fs::write(config_file, json)?;

        Ok(())
    }


    /// Query git user email
    pub(crate) fn query_email(&self) -> Result<&str> {
        self.query_git_parameter(MM_GIT_EMAIL_KEY)
    }


    /// Query git user name
    pub(crate) fn query_name(&self) -> Result<&str> {
        self.query_git_parameter(MM_GIT_NAME_KEY)
    }


    /// Query git parameter by its name
    fn query_git_parameter(&self, parameter: &str) -> Result<&str> {
        let section = self.query_git_section()?;
        let use_default = section
            .get(MM_GIT_USE_DEFAULT_KEY)
            .unwrap_or(&sj::Value::Bool(false));

        //
        // Here I decide, if I need to query default git configuration
        // or look at application's one
        //

        if let Some(true) = use_default.as_bool() {
            self.git_config
                .get_str(parameter)
                .map_err(Error::from)
        }
        else {
            section.get(parameter)
                .and_then(sj::Value::as_str)
                .ok_or(Error::from_string(format!("missing {} parameter in git section", parameter), ErrorCategory::Config))
        }
    }


    /// Obtains a reference to the git section inside of current config.
    fn query_git_section(&self) -> Result<&sj::Value> {
        self.internal
            .get(MM_GIT_KEY)
            .ok_or(Error::from_string("missing git section in config", ErrorCategory::Config))
    }


    /// Validates a JSON object currently stored in a configuration instance.
    fn validate(_raw_config: &sj::Value) -> Result<()> {
        // TODO: validation against JSON schema

        Ok(())
    }


    /// Creates a config instance from parsed JSON, which is validated 
    /// against proper JSON schema.
    fn from_raw(raw_config: sj::Value) -> Result<Self> {
        Config::validate(&raw_config)?;

        Ok(Config {
            internal: raw_config,

            git_config: git2::Config::open_default()
                .and_then(|mut config| config.snapshot())?
        })
    }
}
