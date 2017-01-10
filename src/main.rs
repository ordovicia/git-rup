extern crate git2;

const PROGNAME: &'static str = "git-rup";

fn main() {
    macro_rules! info {
        ($($args: expr),*) => {{
            print!("[{}] ", PROGNAME);
            println!($($args),*);
        }};
    }

    macro_rules! fail {
        ($($args: expr),*) => {{
            print!("[{}] ", PROGNAME);
            println!($($args),*);
            std::process::exit(1);
        }};
    }

    let repo = match git2::Repository::open(".") {
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
}
