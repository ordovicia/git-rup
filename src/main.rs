extern crate git2;

fn main() {
    use git2::Repository;

    let remotes = Repository::open(".").and_then(|repo| repo.remotes());
    if let Result::Err(e) = remotes {
        println!("{}", e);
        return;
    };

    let remotes = remotes.unwrap();
    println!("found {} remotes:", remotes.len());
    for r in remotes.iter() {
        println!("\t{}", r.unwrap());
    }
}
