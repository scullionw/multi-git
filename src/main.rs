use git2::{Repository, StatusOptions};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let config = Config::from_args();
    let current_dir = env::current_dir().unwrap();
    let target = config.target_dir.as_ref().unwrap_or(&current_dir).as_path();

    if target.is_dir() {
        match git_status(target) {
            DirectoryStatus::NOTAREPO => {
                for entry in fs::read_dir(target).unwrap() {
                    let entry = entry.unwrap().path();
                    check(&entry);
                }
            }
            _ => check(target),
        }
    } else {
        panic!("{} is not a directory!", target.display());
    }
}

enum DirectoryStatus {
    OK,
    NOTOK,
    NOTAREPO,
}

fn check(path: impl AsRef<Path>) {
    let name = path.as_ref().iter().last().unwrap().to_str().unwrap();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match git_status(&path) {
        DirectoryStatus::OK => {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            write!(&mut stdout, "OK: ").unwrap();
        }
        DirectoryStatus::NOTOK => {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            write!(&mut stdout, "NEEDS CARE: ").unwrap();
        }
        DirectoryStatus::NOTAREPO => (),
    }
    stdout.reset().unwrap();
    println!("{}", name)
}

fn git_status(path: impl AsRef<Path>) -> DirectoryStatus {
    match Repository::open(path) {
        Ok(repo) => {
            let mut opts = StatusOptions::new();
            opts.include_ignored(false);
            opts.include_untracked(true).recurse_untracked_dirs(true);
            let statuses = repo.statuses(Some(&mut opts)).unwrap();
            if statuses.iter().count() == 0 {
                DirectoryStatus::OK
            } else {
                DirectoryStatus::NOTOK
            }
        }
        Err(_) => DirectoryStatus::NOTAREPO,
    }
}

#[derive(StructOpt)]
struct Config {
    #[structopt(parse(from_os_str))]
    target_dir: Option<PathBuf>,
}
