use std::env;

extern crate getopts;
use getopts::Options;

extern crate git2;

#[macro_use]
mod utils;
#[macro_use]
mod result;
mod operations;
mod status;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // program options
    let mut opts = Options::new();
    opts.optflag("", "dry-run", "dry run");
    opts.optflag("h", "help", "print this help menu");
    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => fail!("program option parse error: {}", e),
    };

    if opt_matches.opt_present("h") {
        println!("{}", opts.usage("Usage: git-rup [options]"));
        return;
    }

    let dry_run = opt_matches.opt_present("dry-run");

    // repository
    let mut repo = try_unwrap!(operations::get_repository());

    // remote
    let remotes = try_unwrap!(operations::get_remotes(&repo));
    assert!(remotes.len() > 0);
    println!("found {} remote{}:",
             remotes.len(),
             if remotes.len() == 1 { "" } else { "s" });

    // fetch
    for name in remotes.iter() {
        match operations::get_remote_validation(&repo, name) {
            Ok(mut remote) => {
                println!("- {} ({})", name.unwrap(), remote.url().unwrap());
                print!("  fetching ... ");
                if dry_run {
                    println!("skipped (dry-run)");
                } else {
                    let _ = try_unwrap!(operations::fetch(&mut remote));
                    println!("success");
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    println!();

    // signature
    let signature = try_unwrap!(operations::get_signature(&repo));
    println!("using signature: {}\n", signature);

    // status
    println!("repository status:");
    // TODO: prety print
    let is_clean = {
        let statuses = try_unwrap!(repo.statuses(None));
        statuses.iter().map(|st| status::pprint(&st)).collect::<Vec<_>>();
        status::is_clean(&statuses)
    };
    if is_clean {
        println!("    clean");
    }
    println!();

    /*
    // save submodules' branch
    {
        let submodules = try_unwrap!(operations::get_submodules(&repo));
        if submodules.len() > 0 {
            println!("submodules:");
            for sb in submodules.iter() {
                match (sb.name(), sb.branch()) {
                    // TODO: always invalid?
                    (Some(n), Some(b)) => {
                        println!("- {} ({})", n, b);
                    }
                    (Some(n), None) => {
                        println!("x {} (invalid branch)", n);
                    }
                    _ => {
                        println!("x invalid submodule name");
                    }
                }
            }
        }
    }
    println!();
    */

    // save stash
    if !is_clean {
        let _ = try_unwrap!(operations::stash_save(&mut repo, &signature));
        println!("stash saved");
    }

    // TODO: Merge all local branches which tracks remote.
    // TODO: Fetch and merge all submodules
    // TODO: Checkout submodules' saved branch

    // pop stash
    if !is_clean {
        let _ = try_unwrap!(operations::stash_pop(&mut repo));
        println!("stash poped");
    }

}
