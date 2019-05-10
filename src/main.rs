use git2::{Repository, StatusOptions};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let config = Config::from_args();
    let current_dir = env::current_dir().unwrap();
    let target = config.target_dir.as_ref().unwrap_or(&current_dir).as_path();

    if target.is_dir() {
        match Repository::open(target) {
            Ok(repo) => repo.show(),
            Err(_) => {
                for entry in fs::read_dir(target).unwrap() {
                    let path = entry.unwrap().path();
                    match Repository::open(&path) {
                        Ok(repo) => repo.show(),
                        Err(_) => println!("{}", extract_name(path.as_path())),
                    }
                }
            }
        }
    } else {
        panic!("{} is not a directory!", target.display());
    }
}

fn extract_name(path: &Path) -> &str {
    path.iter().last().unwrap().to_str().unwrap()
}

enum RepoStatus {
    Clean,
    Dirty,
}

trait StatusExt {
    fn is_synced(&self) -> bool;
    fn status(&self) -> RepoStatus;
    fn show(&self);
}

impl StatusExt for Repository {
    fn is_synced(&self) -> bool {
        let mut git_path = self.path().to_path_buf();
        git_path.push(".git");
        let git_path = git_path.as_path().to_str().unwrap();
        let output = Command::new("git")
            .args(&[
                "--git-dir",
                git_path,
                "log",
                "--branches",
                "--not",
                "--remotes",
            ])
            .output()
            .unwrap();

        output.stdout.is_empty()
    }

    fn status(&self) -> RepoStatus {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false);
        opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = self.statuses(Some(&mut opts)).unwrap();
        if statuses.iter().count() == 0 {
            RepoStatus::Clean
        } else {
            RepoStatus::Dirty
        }
    }

    fn show(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        match self.status() {
            RepoStatus::Clean => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                write!(&mut stdout, "Clean ").unwrap();
            }
            RepoStatus::Dirty => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                write!(&mut stdout, "Dirty ").unwrap();
            }
        }

        if self.is_synced() {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            write!(&mut stdout, "Synced ").unwrap();
        } else {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                .unwrap();
            write!(&mut stdout, "PUSH ").unwrap();
        }

        stdout.reset().unwrap();
        let name = extract_name(self.workdir().unwrap());
        println!("{}", name);
    }
}

#[derive(StructOpt)]
struct Config {
    #[structopt(parse(from_os_str))]
    target_dir: Option<PathBuf>,
}
