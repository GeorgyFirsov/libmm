use std::path::Path;

use git2;

use super::{MM_MAIN_REPO_NAME};
use super::helpers;
use crate::{misc, cfg};
use crate::error::{Error, Result, ErrorCategory};


/// A structure, that describes a repository for notes.
pub struct Repository {
    /// Internal git repository, that manages version control
    internal_repo: git2::Repository,

    /// Name of the repository
    name: String,

    /// Optional list of remotes. `None` if repository has no remotes
    remotes: Option<git2::string_array::StringArray>,

    /// Repository's configuration
    config: cfg::Config
}


impl Repository {
    /// Returns a repository ready to use.
    /// 
    /// Supports opening a repository by its name or a main repo if no name given.
    /// 
    /// * `repo_name` - a name of repository to open (pass `None` to open a main repository)
    pub fn open_or_create(repo_name: Option<&str>) -> Result<Self> {
        //
        // Firstly we need to ensure, that we have repositories folder.
        // App may be run for the first time or data may be erased, so
        // we need to create the folder if necessary
        //

        if !helpers::is_repos_folder_present() {
            helpers::get_repos_folder()
                .ok_or(Error::from_string("cannot get repositories folder", ErrorCategory::Os))
                .and_then(misc::create_folder_recursive)?;
        }

        //
        // Now let's try to open an internal git repository.
        // If it doesn't exists, it is neessary to create it.
        //

        let internal_repo = helpers::get_repo_path(&repo_name)
            .ok_or(Error::from_string("cannot get repository path", ErrorCategory::Os))
            .and_then(helpers::open_or_create_repository)?;

        Repository::from_git_repository(internal_repo, repo_name)
    }


    /// Obtains a working directory for current repository.
    pub fn get_workdir(&self) -> Result<&Path> {
        helpers::get_workdir(&self.internal_repo)
    }
    

    /// Adds a note to repository.
    /// 
    /// Note file MUST exist.
    /// 
    /// * `note_path` - absolute path to a note to add
    pub fn add_note(&self, note_path: &Path) -> Result<()> {
        //
        // First check existence of a note and that path 
        // is actually absolute path to a file
        //

        if !note_path.exists() || !note_path.is_file() || !note_path.is_absolute() {
            return Err(Error::from_string("invalid absolute note path", ErrorCategory::Os));
        }

        //
        // And now let's add a note if it is actually located
        // in current repository
        //

        let workdir = self.get_workdir()?;
        note_path
            .strip_prefix(workdir)
            .map_err(Error::from)
            .and_then(|relative_path| self.add_note_internal(relative_path))
    }


    /// Adds a folder to repository. 
    /// 
    /// Folder MUST exist.
    /// 
    /// * `folder_path` - absolute path to a folder to add
    pub fn add_folder(&self, folder_path: &Path) -> Result<()> {
        //
        // Firstly we need to ensure, that we create a valid folder
        //

        if !folder_path.exists() || !folder_path.is_dir() || !folder_path.is_absolute() {
            return Err(Error::from_string("invalid absolute folder path", ErrorCategory::Os));
        }

        //
        // Just create a directory. Nothing else is required.
        //

        let workdir = self.get_workdir()?;
        folder_path
            .strip_prefix(workdir)
            .map_err(Error::from)
            .and_then(|relative_path| self.add_folder_internal(relative_path))
    }


    /// Internal constructor, that constructs a repository instance from 
    /// internal [`git2::Repository`] instance.
    /// 
    /// * `repo` - git repository instance to wrap
    /// * `repo_name` - a name of repository to open (pass `None` to open a main repository)
    fn from_git_repository(repo: git2::Repository, repo_name: Option<&str>) -> Result<Repository> {
        let remotes = repo
            .remotes()
            .ok();

        let config_file = helpers::get_config_file(&repo)?;

        Ok(Repository { 
            internal_repo: repo, 

            name: repo_name
                .unwrap_or(MM_MAIN_REPO_NAME)
                .to_owned(), 

            remotes: remotes,

            config: cfg::Config::load(&config_file)?
        })
    }


    /// Adds a note to repository (internal implementation).
    /// 
    /// Calls `git2::Index::add_all` in order to take `.gitignore` into 
    /// account, because `git2::Index::add_path` forces files to be added.
    /// 
    /// * `relative_path` - relative to working directory path to the note
    fn add_note_internal(&self, relative_path: &Path) -> Result<()> {
        //
        // Let's add folder first (skip step for notes in repository's root)
        //

        if let Some(parent) = relative_path.parent() {
            self.add_folder_internal(parent)?;
        }

        //
        // And now add note itself
        //

        self.internal_repo
            .index()
            .and_then(|mut index| index.add_all([relative_path].iter(), git2::IndexAddOption::DEFAULT, None))
            .map_err(Error::from)
    }


    /// Adds a folder to repository (internal implementation). 
    /// 
    /// Used for optimization: sometimes workdir is already known, so we can 
    /// skip its acquisition.
    /// 
    /// * `relative_path` - relative to working directory path to the folder
    fn add_folder_internal(&self, relative_path: &Path) -> Result<()> {
        if relative_path.as_os_str().is_empty() {
            //
            // Adding repository's root is a normal case but we don't 
            // need to do anything
            //

            return Ok(());
        }

        Ok(())
    }
}
