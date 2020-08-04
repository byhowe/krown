use clap::{App, Arg, ArgMatches};
use std::path::PathBuf;

const APP_NAME: &'static str = "Krown";
const APP_VERSION: &'static str = "0.1.0";
const APP_AUTHOR: &'static str = "B. Howe <37745048+byhowe@users.noreply.github.com>";
const APP_ABOUT: &'static str = "A window manager for the Kings and Queens";

fn get_home_directory() -> PathBuf
{
  std::env::var_os("HOME")
    .and_then(|p| {
      if p.is_empty() {
        None
      } else {
        Some(PathBuf::from(p))
      }
    })
    .unwrap_or_else(|| {
      eprintln!("Fatal: $HOME variable is not set!");
      std::process::exit(1);
    })
}

fn get_xdg_config_directory() -> PathBuf
{
  std::env::var_os("XDG_CONFIG_HOME")
    .and_then(|p| {
      if p.is_empty() {
        Some(get_home_directory().join(".config"))
      } else {
        Some(PathBuf::from(p))
      }
    })
    .unwrap()
}

fn get_source_directory() -> PathBuf
{
  std::env::var_os("KROWN_SOURCE_DIR")
    .and_then(|p| {
      if p.is_empty() {
        None
      } else {
        Some(PathBuf::from(p))
      }
    })
    .or(Some(
      get_xdg_config_directory().join(APP_NAME.to_lowercase()),
    ))
    .map(|p| {
      p.canonicalize().unwrap_or_else(|_| {
        eprintln!("Error: Source directory '{}' does not exist!", p.display());
        std::process::exit(1);
      })
    })
    .unwrap()
}

pub(crate) struct ConsoleStatus
{
  pub(crate) success: bool,
  pub(crate) start: bool,
}

impl ConsoleStatus
{
  fn new() -> Self
  {
    Self {
      success: true,
      start: true,
    }
  }
}

pub(crate) struct Console
{
  source_directory_path: PathBuf,
  srcdir: bool,
  recompile: bool,
  check: bool,
}

impl Console
{
  pub(crate) fn new() -> Self
  {
    let matches: ArgMatches = App::new(APP_NAME)
      .version(APP_VERSION)
      .author(APP_AUTHOR)
      .about(APP_ABOUT)
      .arg(
        Arg::with_name("srcdir")
          .short("s")
          .long("src-dir")
          .help("Prints the path to the source code"),
      )
      .arg(
        Arg::with_name("recompile")
          .short("r")
          .long("recompile")
          .help("Recompiles the source code"),
      )
      .arg(
        Arg::with_name("check")
          .short("c")
          .long("check")
          .help("Analyzes the source code for errors"),
      )
      .get_matches();

    Self {
      source_directory_path: get_source_directory(),
      srcdir: matches.is_present("srcdir"),
      recompile: matches.is_present("recompile"),
      check: matches.is_present("check"),
    }
  }

  pub(crate) fn perform_actions(&self) -> ConsoleStatus
  {
    let mut status = ConsoleStatus::new();

    if self.srcdir || self.recompile || self.check {
      status.start = false;
    }

    if self.srcdir {
      println!("{}", self.source_directory_path.display());
    }

    if self.check {
      println!(
        "Analyzing the source code at '{}'.",
        self.source_directory_path.display()
      );

      status.success = status.success
        && if self.check() {
          println!("No errors were found in the source code.");
          true
        } else {
          println!("Errors were found in the source code.");
          false
        };
    }

    if self.recompile && if self.check { status.success } else { true } {
      println!(
        "Recompiling the source code at '{}'.",
        self.source_directory_path.display()
      );

      status.success = status.success
        && if self.recompile() {
          println!("{} compilation exited with success.", APP_NAME);
          true
        } else {
          println!("{} compilation exited with failure.", APP_NAME);
          false
        };
    }

    status
  }

  fn recompile(&self) -> bool
  {
    std::process::Command::new("cargo")
      .arg("install")
      .arg("--path=.")
      .current_dir(&self.source_directory_path)
      .status()
      .unwrap_or_else(|_| {
        eprintln!("Fatal: Failed to execute the cargo command!");
        std::process::exit(1);
      })
      .success()
  }

  fn check(&self) -> bool
  {
    std::process::Command::new("cargo")
      .arg("check")
      .current_dir(&self.source_directory_path)
      .status()
      .unwrap_or_else(|_| {
        println!("Fatal: Failed to execute the cargo command");
        std::process::exit(1);
      })
      .success()
  }
}
