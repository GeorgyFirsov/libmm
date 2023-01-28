use std::fs;
use std::path::Path;

use serde_json as sj;

use crate::error::{ Error, Result };
use super::{ 
    MM_CONFIG_GIT_KEY, 
    MM_CONFIG_GIT_USE_DEFAULT_KEY,
};


/// Creates a default configuration and writes it into a file.
/// 
/// * `config_file` - path to a configuration file to write
///                   default config to
pub(crate) fn create_default(config_file: &Path) -> Result<()> {
    //
    // Let's compose a default configuration
    //

    let default_config = sj::json!({
        MM_CONFIG_GIT_KEY: {
            MM_CONFIG_GIT_USE_DEFAULT_KEY: true,
        }
    });

    //
    // And save it into a file
    //

    let json = sj::to_string_pretty(&default_config)?;
    fs::write(config_file, json)?;

    Ok(())
}
