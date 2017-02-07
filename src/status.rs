extern crate git2;

fn print_head(repo: &git2::Repository) {
    let head = try_unwrap!(repo.head());

    if head.is_branch() {
        let name = try_unwrap_opt!(head.shorthand(), "non UTF-8 branch name");
        let branch = try_unwrap!(repo.find_branch(name, git2::BranchType::Local));
        let upstream = branch.upstream();
        if let Ok(upstream) = upstream {
            let up_name = try_unwrap_opt!(try_unwrap!(upstream.name()),
                                          "non UTF-8 upstream branch name");
            println!("  On branch {} ... {}", name, up_name);
        } else {
            println!("  On branch {}", name);
        }
    } else {
        println!("  not on any branch");
    }
}

pub fn print(repo: &git2::Repository) {
    print_head(repo);

    let statuses = try_unwrap!(repo.statuses(None));

    // Staged
    let mut staged_exist = false;
    for entry in statuses.iter().filter(|e| e.status() != git2::STATUS_CURRENT) {
        let type_str = match entry.status() {
            s if s.contains(git2::STATUS_INDEX_NEW) => "new file:     ",
            s if s.contains(git2::STATUS_INDEX_MODIFIED) => "modified:     ",
            s if s.contains(git2::STATUS_INDEX_DELETED) => "deleted:      ",
            s if s.contains(git2::STATUS_INDEX_RENAMED) => "renamed:      ",
            s if s.contains(git2::STATUS_INDEX_TYPECHANGE) => "type changed: ",
            _ => continue,
        };

        if !staged_exist {
            println!("= staged =");
            staged_exist = true;
        }

        let old_path = entry.head_to_index().unwrap().old_file().path();
        let new_path = entry.head_to_index().unwrap().new_file().path();
        match (old_path, new_path) {
            (Some(ref old), Some(ref new)) if old != new => {
                println!("- {} {} -> {}", type_str, old.display(), new.display());
            }
            (old, new) => {
                println!("- {} {}", type_str, old.or(new).unwrap().display());
            }
        }
    }

    // Working tree
    let mut wt_exist = false;
    for entry in statuses.iter()
        .filter(|e| e.status() != git2::STATUS_CURRENT && !e.index_to_workdir().is_none()) {
        let type_str = match entry.status() {
            s if s.contains(git2::STATUS_WT_MODIFIED) => "modified:     ",
            s if s.contains(git2::STATUS_WT_DELETED) => "deleted:      ",
            s if s.contains(git2::STATUS_WT_RENAMED) => "renamed:      ",
            s if s.contains(git2::STATUS_WT_TYPECHANGE) => "type changed: ",
            _ => continue,
        };

        if !wt_exist {
            println!("= working tree =");
            wt_exist = true;
        }

        let old_path = entry.index_to_workdir().unwrap().old_file().path();
        let new_path = entry.index_to_workdir().unwrap().new_file().path();
        match (old_path, new_path) {
            (Some(ref old), Some(ref new)) if old != new => {
                println!("- {} {} -> {}", type_str, old.display(), new.display());
            }
            (old, new) => {
                println!("- {} {}", type_str, old.or(new).unwrap().display());
            }
        }
    }

    for entry in statuses.iter().filter(|e| e.status() == git2::STATUS_WT_NEW) {
        println!("- untracked:     {}",
                 entry.index_to_workdir().unwrap().old_file().path().unwrap().display());
    }
}
