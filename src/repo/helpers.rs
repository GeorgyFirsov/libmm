use std::path::{Path, PathBuf};

use crate::{data, misc, cfg};
use crate::error::{Result, Error, ErrorCategory};
use super::{
    MM_REPOS_SUBFOLDER, 
    MM_MAIN_REPO_NAME,
    MM_CONFIG_FILE, 
    MM_CONFIG_FOLDER,
    MM_GIT_HEAD_REF,
    MM_INITIAL_COMMIT_MESSAGE,
    MM_DEFAULT_COMMIT_MESSAGE,
};


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


/// Returns path to a repository's working directory
/// 
/// * `repo` - repository to get working directory of
pub(super) fn get_workdir(repo: &git2::Repository) -> Result<&Path> {
    repo.workdir()
        .ok_or(Error::from_string("cannot get working directory", ErrorCategory::Git))
}


/// Returns path to a repository's configuration folder 
/// 
/// * `repo` - repository to get working directory of
pub(super) fn get_config_girectory(repo: &git2::Repository) -> Result<PathBuf> {
    get_workdir(&repo)
        .map(|workdir| workdir.join(MM_CONFIG_FOLDER))
}


/// Returns path to a repository's configuation file
/// 
/// * `repo` - repository to get working directory of
pub(super) fn get_config_file(repo: &git2::Repository) -> Result<PathBuf> {
    get_config_girectory(&repo)
        .map(|config_folder| config_folder.join(MM_CONFIG_FILE))
}


/// Stages and commits all specified files.
/// 
/// * `repo` - reference to git repository instance
/// * `config` - reference to configuration instance
/// * `pathspecs` - list of files to be committed (paths 
///                 MUST be relative to the repository's 
///                 working directory)
/// * `message` - optional commit message (default one is 
///               [`super::MM_DEFAULT_COMMIT_MESSAGE`])
pub(super) fn commit_files<T, I>(repo: &git2::Repository, config: &cfg::Config, pathspecs: I, message: Option<&str>) -> Result<()>
where
    T: git2::IntoCString,
    I: IntoIterator<Item = T>
{
    //
    // First of all, we need to stage all the changes
    //

    let mut index = repo.index()?;
    
    index.add_all(pathspecs, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_oid = index.write_tree()?;

    //
    // Now let's create a commit
    //

    let tree = repo.find_tree(tree_oid)?;
    let author = git2::Signature::now(config.query_name()?, config.query_email()?)?;
    let message = message
        .unwrap_or(MM_DEFAULT_COMMIT_MESSAGE);

    //
    // Well... Here I need a slice with references for parents container,
    // hence I MUST do it in the following scary way :(
    // Error is ignored intentionally
    //

    let head = repo
        .refname_to_id(MM_GIT_HEAD_REF)
        .and_then(|head_oid| repo.find_commit(head_oid))
        .ok();

    let head_holder;
    let parents = match head.as_ref() {
        Some(head) => {
            head_holder = [head];
            &head_holder[..]
        },
        _ => &[]
    };

    repo.commit(Some(MM_GIT_HEAD_REF), &author, &author, message, &tree, parents)?;

    Ok(())
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

    //
    // Now we need to create a configuration file inside of a special folder
    //

    let config_folder = get_config_girectory(&repo)?;
    misc::create_folder(&config_folder)?;

    let config_file = get_config_file(&repo)?; 
    misc::touch_new_file(&config_file)?;

    let config = cfg::Config::new()?;
    config.save(&config_file)?;

    //
    // To commit config file I need to convert its path to the relative one
    //

    let workdir = get_workdir(&repo)?;
    let relative_path = config_file.strip_prefix(workdir)?;

    commit_files(&repo, &config, [relative_path].iter(), Some(MM_INITIAL_COMMIT_MESSAGE))?;

    //
    // Done for now!
    //

    Ok(repo)
}
