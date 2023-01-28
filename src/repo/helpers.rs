use std::path::{Path, PathBuf};

use super::{MM_REPOS_SUBFOLDER, MM_MAIN_REPO_NAME, MM_CONFIG_FILE, MM_CONFIG_FOLDER};
use crate::{data, misc, cfg};
use crate::error::{Result, Error, ErrorCategory};


/// Get full repositories folder path.
pub(super) fn get_repos_folder() -> Option<PathBuf> {
    data::get_mm_folder()
        .map(|path| path.join(MM_REPOS_SUBFOLDER))
}


/// Check if repositories folder exists.
pub(super) fn is_repos_folder_present() -> bool {
    //
    // Well, let's assume, that inaccessible path is inexistent
    //

    get_repos_folder()
        .map_or(false, |path| path.exists())
}


/// Compose full repository path by its name.
/// 
/// * `repo_name` - a name of repository (or `None` for a main repository)
pub(super) fn get_repo_path(repo_name: &Option<&str>) -> Option<PathBuf> {
    get_repos_folder()
        .map(|path| path.join(repo_name.unwrap_or(MM_MAIN_REPO_NAME)))
}


/// Returns path to a repository's configuration folder 
/// by it's working directory.
/// 
/// * `repo_workdir` - repository's working directory
pub(super) fn get_config_girectory(repo_workdir: &Path) -> PathBuf {
    repo_workdir
        .join(MM_CONFIG_FOLDER)
}


/// Returns path to a repository's configuation file
/// by it's working directory.
/// 
/// * `repo_workdir` - repository's working directory
pub(super) fn get_config_file(repo_workdir: &Path) -> PathBuf {
    get_config_girectory(repo_workdir)
        .join(MM_CONFIG_FILE)
}


/// Open or create a git repository by its path.
/// 
/// * `path` - path to the repository's directory
pub(super) fn open_or_create_repository(path: PathBuf) -> Result<git2::Repository> {
    git2::Repository::open(path.to_owned())
        .or_else(|_error| create_repository(&path))
        .map_err(Error::from)
}


/// Creates a git repository with a configuration file
/// 
/// * `path` - path to the repository's directory
fn create_repository(path: &Path) -> Result<git2::Repository> {
    //
    // Fistly create a repository
    //

    let repo = git2::Repository::init(path)?;
    let workdir = repo.workdir()
        .ok_or(Error::from_string("cannot get working directory", ErrorCategory::Git))?;

    //
    // Now we need to create a configuration file inside of a special folder
    //

    misc::create_folder(get_config_girectory(workdir))?;

    let config_file = get_config_file(workdir); 
    misc::touch_new_file(&config_file)?;
    cfg::Config::new()
        .save(&config_file)?;

    // TODO: stage and commit file

    Ok(repo)
}
