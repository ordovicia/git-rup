use std::env;
use std::io::{self, Write};

extern crate getopts;
use getopts::Options;

extern crate git2;

#[macro_use]
mod utils;
#[macro_use]
mod result;
mod operations;

macro_rules! print_flush {
    ($($args: expr),*) => {{
        println!($($args),*);
          io::stdout().flush().unwrap();
    }};
}

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
                print_flush!("  fetching ... ");
                if dry_run {
                    println!("skipped (dry-run)");
                } else {
                    let _ = try_unwrap!(operations::fetch(&mut remote));
                    println!("done");
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    if !operations::is_head_on_branch(&repo) {
        println!("HEAD is not on any branch!");
        return;
    }
    // let current_branch = operations::current_branch(&repo);
    let is_dirty = operations::is_dirty(&repo);
    if is_dirty {
        println!("repository is dirty");
    }

    // signature
    let signature = try_unwrap!(repo.signature());
    println!("using signature:\n  {}", signature);
    if is_dirty {
        print_flush!("stashing ... ");
        let _ = try_unwrap!(operations::stash_save(&mut repo, &signature));
        println!("done");
    }

    // merge
    {
        let branch_num = try_unwrap!(repo.branches(Some(git2::BranchType::Local))).count();
        println!("found {} local branch{}:",
                 branch_num,
                 if branch_num == 1 { "" } else { "es" });

        let local_branches = try_unwrap!(repo.branches(Some(git2::BranchType::Local)));
        for branch in local_branches {
            let (branch, _) = try_unwrap!(branch);
            let upstream = branch.upstream();
            match upstream {
                Ok(up) => {
                    let loc_name = try_unwrap!(branch.name());
                    let up_name = try_unwrap!(up.name());
                    match (loc_name, up_name) {
                        (Some(loc_name), Some(up_name)) => {
                            println!("- {} -> {}", loc_name, up_name);
                            print_flush!("  merging ... ");
                            if dry_run {
                                println!("skipped (dry-run)");
                                continue;
                            }
                            println!("done");

                            let loc_commit = operations::branch_commit(&branch);
                            let rem_commit = operations::branch_commit(&up);
                            let _ = repo.merge_commits(&loc_commit, &rem_commit, None);
                        }
                        _ => println!("x non UTF-8 branch name"),
                    }
                }
                _ => {
                    match try_unwrap!(branch.name()) {
                        Some(loc_name) => println!("- {}", loc_name),
                        _ => println!("x non UTF-8 branch name"),
                    }
                }
            }
        }
    }

    {
        let submodules = try_unwrap!(repo.submodules());
        if submodules.len() > 0 {
            println!("found {} submodule{}",
                     submodules.len(),
                     if submodules.len() > 1 { "s" } else { "" });
            for mut sb in submodules {
                {
                    if let Some(name) = sb.name() {
                        println!("- {}", name);
                    } else {
                        println!("x non UTF-8 submodule name");
                        continue;
                    }
                }

                let _ = try_unwrap!(sb.sync());
                let _ = try_unwrap!(sb.reload(false));
            }
        }
    }

    if is_dirty {
        print_flush!("popping stash ... ");
        let _ = try_unwrap!(operations::stash_pop(&mut repo));
        println!("done");
    }
}
