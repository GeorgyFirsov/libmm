mod cfg;


pub(crate) use self::cfg::{ Config };


/// A key in config, that is responsible for all git configuration items
const MM_CONFIG_GIT_KEY: &str = "git";

/// A boolean configuration property, that designates a system-default 
/// git config usage
const MM_CONFIG_GIT_USE_DEFAULT_KEY: &str = "default";
