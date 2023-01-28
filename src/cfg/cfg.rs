use std::fs;
use std::path::Path;

use serde_json as sj;

use crate::error::{ Error, Result };
use super::{ 
    MM_CONFIG_GIT_KEY, 
    MM_CONFIG_GIT_USE_DEFAULT_KEY,
};


/// Struct, that provides an interface to configuration of `libmm`.
pub(crate) struct Config {
    /// Internal JSON structure 
    internal: sj::Value,
}


impl Config {
    /// Creates a default configuration. Fills all necessary values 
    /// with their defaults. Optional values are omitted.
    pub(crate) fn new() -> Self {
        //
        // Let's compose a default configuration...
        //
    
        let default_config = sj::json!({
            MM_CONFIG_GIT_KEY: {
                MM_CONFIG_GIT_USE_DEFAULT_KEY: true,
            }
        });

        //
        // ... and wrap it into an instance of `Config`
        //

        Config { 
            internal: default_config
        }
    }


    /// Loads configuration from a file.
    /// 
    /// * `config_file` - path to a file to read configuration from
    pub(crate) fn load(config_file: &Path) -> Result<Config> {
        let file_content = fs::read(config_file)?;

        //
        // Create a JSON instance and validate it
        //

        let raw_config = sj::from_slice(file_content.as_slice())?;
        Config::validate(&raw_config)?;

        //
        // Here is everything OK
        //

        Ok(Config {
            internal: raw_config
        })
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


    /// Validates a JSON object currently stored in a configuration instance
    fn validate(raw_config: &sj::Value) -> Result<()> {
        // TODO: validation against JSON schema

        Ok(())
    }
}
