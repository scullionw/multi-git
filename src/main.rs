use git2::{Repository, StatusOptions};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    let current_dir = env::current_dir()?;
    let target = config.target_dir.as_ref().unwrap_or(&current_dir).as_path();

    if target.is_dir() {
        if let Ok(repo) = Repository::open(target) {
            repo.show()?
        } else {
            for entry in fs::read_dir(target)? {
                let path = entry?.path();
                if let Ok(repo) = Repository::open(&path) {
                    repo.show()?
                } else {
                    println!("{:<20}{:<10}", "", extract_name(path.as_path()))
                }
            }
        }
    } else {
        return Err(format!("{} is not a directory!", target.display()).into());
    }

    Ok(())
}

fn extract_name(path: &Path) -> Cow<str> {
    path.iter()
        .last()
        .expect("Path should have atleast 1 component")
        .to_string_lossy()
}

enum RepoStatus {
    Clean,
    Dirty,
}

trait StatusExt {
    fn is_synced(&self) -> Result<bool, Box<dyn Error>>;
    fn status(&self) -> Result<RepoStatus, Box<dyn Error>>;
    fn show(&self) -> Result<(), Box<dyn Error>>;
}

impl StatusExt for Repository {
    fn is_synced(&self) -> Result<bool, Box<dyn Error>> {
        let output = Command::new("git")
            .args(&[
                "--git-dir",
                &self.path().to_string_lossy(),
                "log",
                "--branches",
                "--not",
                "--remotes",
            ])
            .output()?;

        if output.status.success() {
            Ok(output.stdout.is_empty() && !self.remotes()?.is_empty())
        } else {
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            Err("git command failed.".into())
        }
    }

    fn status(&self) -> Result<RepoStatus, Box<dyn Error>> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false);
        opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = self.statuses(Some(&mut opts))?;
        if statuses.iter().count() == 0 {
            Ok(RepoStatus::Clean)
        } else {
            Ok(RepoStatus::Dirty)
        }
    }

    fn show(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        match self.status()? {
            RepoStatus::Clean => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                write!(&mut stdout, "{:<10}", "Clean")?;
            }
            RepoStatus::Dirty => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                write!(&mut stdout, "{:<10}", "Commit!")?;
            }
        }

        if self.is_synced()? {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(&mut stdout, "{:<10}", "Synced")?;
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut stdout, "{:<10}", "Push!")?;
        }

        stdout.reset()?;
        let path = self.path().parent().expect("Parent of .git cannot be root");
        println!("{:<10}", extract_name(path));

        Ok(())
    }
}

#[derive(StructOpt)]
struct Config {
    #[structopt(parse(from_os_str))]
    target_dir: Option<PathBuf>,
}
