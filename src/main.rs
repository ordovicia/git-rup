extern crate git2;
use git2::{Repository, FetchOptions, FetchPrune, AutotagOption};

const PROGNAME: &'static str = "git-rup";

fn main() {
    macro_rules! info {
        ($($args: expr),*) => {{
            print!("[{}] ", PROGNAME);
            println!($($args),*);
        }};
    }

    macro_rules! warn {
        ($($args: expr),*) => { info!($($args),*); };
    }

    macro_rules! fail {
        ($($args: expr),*) => {{
            print!("[{}] ", PROGNAME);
            println!($($args),*);
            std::process::exit(1);
        }};
    }

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => fail!("{}", e),
    };

    let remotes = match repo.remotes() {
        Ok(remotes) => remotes,
        Err(e) => fail!("{}", e),
    };

    info!("found {} remotes:", remotes.len());
    for r in remotes.iter() {
        match r {
            Some(r) => {
                info!("    {}", r);
            }
            None => {
                info!("    none UTF-8 remote name");
            }
        }
    }
    info!();

    for r in remotes.iter() {
        if r.is_none() {
            continue;
        }
        let r = r.unwrap();

        info!("fetching from {}...", r);

        let mut fetch_options = FetchOptions::new();
        fetch_options.prune(FetchPrune::On).download_tags(AutotagOption::All);

        match repo.find_remote(r)
            .and_then(|ref mut remote| remote.fetch(&[], Some(&mut fetch_options), None)) {
            Ok(_) => {
                info!("fetched successfully");
            }
            Err(e) => {
                warn!("{}", e);
            }
        };
    }
}
