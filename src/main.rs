use git2::Repository;
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let repo = match Repository::open(current_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{:?}", repo.state());
}
