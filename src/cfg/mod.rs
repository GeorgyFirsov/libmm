mod cfg;


pub(crate) use self::cfg::{Config};


/// A key in config, that is responsible for all git configuration items
const MM_GIT_KEY: &str = "git";

/// A boolean configuration property, that designates a system-default 
/// git config usage
const MM_GIT_USE_DEFAULT_KEY: &str = "use.default";

/// A string configuration property, that contains a git user email
const MM_GIT_EMAIL_KEY: &str = "user.email";

/// A string configuration property, that contains a git user name
const MM_GIT_NAME_KEY: &str = "user.name";
