use std;

extern crate git2;
use git2::{Repository, Remote};
use git2::{FetchOptions, FetchPrune, AutotagOption};
use git2::string_array::StringArray;
use git2::Signature;

use result::{self, Result, Error};

pub fn get_repository() -> Result<Repository> {
    result::with_msg(Repository::discover("."), "failed to open repository")
        .and_then(|r| match r.state() {
            git2::RepositoryState::Clean => Ok(r),
            _ => Err(Error::new("repository is under another operation")),
        })
}

pub fn get_remotes(repo: &Repository) -> Result<StringArray> {
    match repo.remotes() {
        Ok(ref remotes) if remotes.len() == 0 => Err(Error::new("does not have remote repository")),
        Ok(remotes) => Ok(remotes),
        Err(e) => Err(Error::with_err("failed to get remote info", e)),
    }
}

pub fn get_remote_validation<'repo>(repo: &'repo Repository,
                                    name: Option<&'repo str>)
                                    -> std::result::Result<Remote<'repo>, String> {
    if let Some(name) = name {
        match repo.find_remote(name) {
            Ok(remote) => {
                if let Some(_) = remote.url() {
                    Ok(repo.find_remote(name).unwrap())
                } else {
                    Err(format!("x {} non UTF-8 remote URL", name))
                }
            }
            Err(e) => Err(format!("x {} couldn't find: {}", name, e)),
        }
    } else {
        Err("x non UTF-8 remote name or URL".to_owned())
    }
}

pub fn fetch(remote: &mut Remote) -> Result<()> {
    let mut fetch_options = FetchOptions::new();
    fetch_options.prune(FetchPrune::On).download_tags(AutotagOption::All);

    result::with_msg(remote.fetch(&[], Some(&mut fetch_options), None),
                     "fetch failed")
}

pub fn get_signature(repo: &Repository) -> Result<Signature<'static>> {
    result::with_msg(repo.signature(), "failed to create signature")
}

pub fn get_submodules(repo: &Repository) -> Result<Vec<git2::Submodule>> {
    match repo.submodules() {
        Ok(sb) => Ok(sb),
        Err(e) => Err(Error::with_err("failed to get submodule info", e)),
    }
}

pub fn stash_save(repo: &mut Repository, signature: &Signature) -> Result<()> {
    match repo.stash_save(&signature, "automatically stashed by git-rup", None) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::with_err("failed to save stash", e)),
    }
}

pub fn stash_pop(repo: &mut Repository) -> Result<()> {
    match repo.stash_pop(0, None) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::with_err("failed to pop stash", e)),
    }
}
