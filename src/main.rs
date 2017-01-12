use std::env;

extern crate getopts;
use getopts::Options;

extern crate git2;
use git2::{Repository, RepositoryState};
use git2::{FetchOptions, FetchPrune, AutotagOption};

#[macro_use]
mod utils;
mod status;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // program options
    let mut opts = Options::new();
    opts.optflag("", "dry-run", "dry run");
    opts.optflag("h", "help", "print this help menu");
    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            fail!("program option parse error: {}", e);
        }
    };

    if opt_matches.opt_present("h") {
        println!("{}", opts.usage("Usage: git-rup [options]"));
        return;
    }

    let dry_run = opt_matches.opt_present("dry-run");

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
        if dry_run {
            info!("skipped (dry-run)");
        } else {
            match remote.fetch(&[], Some(&mut fetch_options), None) {
                Ok(_) => {
                    info!("success");
                }
                Err(e) => {
                    fail!("fetch failed: {}", e);
                }
            };
        }
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
    info!("repository status:");
    let is_clean = match repo.statuses(None) {
        Ok(statuses) => {
            statuses.iter().map(|st| status::pprint(&st)).collect::<Vec<_>>();
            status::is_clean(&statuses)
        }
        Err(e) => {
            fail!("failed to get repository status: {}", e);
        }
    };
    info!();

    // stash
    if !is_clean {
        match repo.stash_save(&signature, "automatically stashed by git-rup", None) {
            Ok(_) => {
                info!("stashed");
            }
            Err(e) => {
                fail!("failed to create stash: {}", e);
            }
        };

        match repo.stash_pop(0, None) {
            Ok(_) => {
                info!("stash popped");
            }
            Err(e) => {
                fail!("failed to pop stash: {}", e);
            }
        };
    }
}
