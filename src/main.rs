extern crate git2;
use git2::{Repository, RepositoryState};
use git2::{FetchOptions, FetchPrune, AutotagOption};

#[macro_use]
mod utils;

fn main() {
    // repository
    let mut repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => fail!("failed to open repository: {}", e),
    };

    match repo.state() {
        RepositoryState::Clean => {}
        st => {
            fail!("repository is not clean: {:#?}", st);
        }
    };

    // remotes
    let remotes = match repo.remotes() {
        Ok(remotes) => remotes,
        Err(e) => fail!("failed to get remotes info: {}", e),
    };

    // validate remotes
    info!("found {} remotes:", remotes.len());
    let mut valid_remotes_idx = vec![];
    for remote_name_or in remotes.iter() {
        if let Some(remote_name) = remote_name_or {
            let is_valid = match repo.find_remote(remote_name) {
                Ok(remote) => {
                    if let Some(url) = remote.url() {
                        info!("- {} ({})", remote_name, url);
                        true
                    } else {
                        warn!("# {} non UTF-8 remote URL", remote_name);
                        false
                    }
                }
                Err(e) => {
                    warn!("# {} couldn't find: {}", remote_name, e);
                    false
                }
            };

            valid_remotes_idx.push(is_valid);
        } else {
            warn!("# non UTF-8 remote name or URL");
        }
    }
    info!();

    // fetch
    let valid_remotes = remotes.iter()
        .zip(valid_remotes_idx)
        .filter_map(|(r, v)| if v { Some(r) } else { None })
        .map(|n| n.unwrap());

    for remote_name in valid_remotes {
        let mut remote = repo.find_remote(remote_name).unwrap();
        info!("fetching from {} ({}) ...",
              remote_name,
              remote.url().unwrap());

        let mut fetch_options = FetchOptions::new();
        fetch_options.prune(FetchPrune::On).download_tags(AutotagOption::All);
        match remote.fetch(&[], Some(&mut fetch_options), None) {
            Ok(_) => {
                info!("success");
            }
            Err(e) => {
                fail!("fetch failed: {}", e);
            }
        };
    }
    info!();

    // signature
    let signature = match repo.signature() {
        Ok(sig) => sig,
        Err(e) => fail!("failed to create signature: {}", e),
    };
    info!("using signature: {}", signature);
    info!();

    // status
    info!("repository statue:");
    let is_clean = match repo.statuses(None) {
        Ok(statuses) => {
            // TODO: WIP
            for st in statuses.iter() {
                info!("{:?}: {}", st.status(), st.path().unwrap());
            }
            info!("{}", statuses.len());
            statuses.len() == 0
        }
        Err(e) => {
            fail!("failed to get repository status: {}", e);
        }
    };

    // stash
    if is_clean {
        if let Err(e) = repo.stash_save(&signature, "automatically stashed by git-rup", None) {
            fail!("failed to create stash: {}", e);
        };

        if let Err(e) = repo.stash_pop(0, None) {
            fail!("failed to pop stash: {}", e);
        };
    }
}
