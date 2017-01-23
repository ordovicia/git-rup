extern crate git2;
use git2::Error;
use git2::{Repository, Remote};
use git2::{FetchOptions, FetchPrune, AutotagOption};
use git2::string_array::StringArray;
use git2::Signature;

use result::Result;

pub fn get_repository() -> Result<Repository> {
    let repo = try!(Repository::discover("."));
    match repo.state() {
        git2::RepositoryState::Clean => Ok(repo),
        _ => Err(Error::from_str("repository is under another operation")),
    }
}

pub fn get_remotes(repo: &Repository) -> Result<StringArray> {
    match repo.remotes() {
        Ok(ref remotes) if remotes.len() == 0 => {
            Err(Error::from_str("does not have remote repository"))
        }
        r => r,
    }
}

pub fn get_remote_validation<'repo>(repo: &'repo Repository,
                                    name: Option<&'repo str>)
                                    -> Result<Remote<'repo>> {
    if let Some(name) = name {
        match repo.find_remote(name) {
            Ok(remote) => {
                if let Some(_) = remote.url() {
                    Ok(repo.find_remote(name).unwrap())
                } else {
                    Err(Error::from_str(&format!("x {} non UTF-8 remote URL", name)))
                }
            }
            Err(e) => Err(Error::from_str(&format!("x {} couldn't find: {}", name, e))),
        }
    } else {
        Err(Error::from_str("x non UTF-8 remote name or URL"))
    }
}

pub fn fetch(remote: &mut Remote) -> Result<()> {
    let mut fetch_options = FetchOptions::new();
    fetch_options.prune(FetchPrune::On).download_tags(AutotagOption::All);
    remote.fetch(&[], Some(&mut fetch_options), None)
}

pub fn stash_save(repo: &mut Repository, signature: &Signature) -> Result<git2::Oid> {
    repo.stash_save(&signature, "automatically stashed by git-rup", None)
}

pub fn stash_pop(repo: &mut Repository) -> Result<()> {
    repo.stash_pop(0, None)
}
