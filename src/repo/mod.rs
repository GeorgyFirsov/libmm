mod helpers;
mod repository;


pub use self::repository::{Repository};


/// Path to repositories relative to mm's data folder.
const MM_REPOS_SUBFOLDER: &str = "repos/";

/// Name of main repository.
const MM_MAIN_REPO_NAME: &str = "mm_main_local";

/// Name of repository's configuration folder
const MM_CONFIG_FOLDER: &str = ".mm";

/// Name of configuration file, that is located in each repository
const MM_CONFIG_FILE: &str = "mm_config.json";

/// Reference being updated in git repositories
const MM_GIT_HEAD_REF: &str = "HEAD";

/// Message for initial commit
const MM_INITIAL_COMMIT_MESSAGE: &str = "chore: initial commit";

/// Default commit message (if no one is specified)
const MM_DEFAULT_COMMIT_MESSAGE: &str = "chore: several files updated";
