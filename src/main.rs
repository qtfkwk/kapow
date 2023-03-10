use clap::Parser;
use dtg_lib::{tz, Dtg};
use execute::{shell, Execute};
use lazy_static::lazy_static;
use pager::Pager;
use regex::{Captures, Regex};
use std::io::BufRead;
use std::process::Stdio;

/// Print a message to stderr and exit with the given code
macro_rules! exit {
    ($code:expr, $($x:tt)*) => ({
        eprintln!($($x)*);
        std::process::exit($code);
    });
}

/// KAPOW!
#[derive(Parser)]
#[command(version, max_term_width = 80)]
struct Cli {
    /// Print readme
    #[arg(short, long, conflicts_with = "input_files")]
    readme: bool,

    /// Input file(s)
    #[arg(required = true)]
    input_files: Vec<std::path::PathBuf>,
}

lazy_static! {
    /// Regular expression for `!now` directive
    static ref NOW: Regex = Regex::new(r"`!now:([^`]*)`").unwrap();
}

/// Main function
fn main() -> Result<(), String> {
    let cli = Cli::parse();
    if cli.readme {
        Pager::with_pager("bat -pl md").setup();
        print!("{}", include_str!("../README.md"));
        std::process::exit(0);
    }
    let d = Dtg::now();
    let now = d.rfc_3339();
    let now_x = d.x_format();
    let now_local = d.default(&tz("local").ok());
    let mut command_q = vec![];
    let original_dir = std::env::current_dir().expect("current directory");
    for input_file in &cli.input_files {
        match std::fs::File::open(input_file) {
            Ok(f) => {
                let f = std::io::BufReader::new(f);
                let dir = input_file.parent().expect("input file parent");
                std::env::set_current_dir(dir)
                    .unwrap_or_else(|e| panic!("Could not change directory to {dir:?}: {e}"));
                for line in f.lines() {
                    let line = line.unwrap();
                    if !command_q.is_empty() {
                        // !run:command \
                        // args
                        if let Some(command) = line.strip_suffix('\\') {
                            command_q.push(command.to_string());
                        } else {
                            command_q.push(line.to_string());
                            run(command_q.drain(..).collect::<String>());
                        }
                    } else if let Some(command) = line.strip_prefix("!run:") {
                        // !run:command
                        if let Some(command) = command.strip_suffix('\\') {
                            command_q.push(command.to_string());
                        } else {
                            run(command);
                        }
                    } else if let Some(path) = line.strip_prefix("!inc:") {
                        // !inc:path
                        match std::fs::File::open(path) {
                            Ok(f) => {
                                let f = std::io::BufReader::new(f);
                                for line in f.lines() {
                                    let line = line.unwrap();
                                    println!("{line}");
                                }
                            }
                            Err(e) => {
                                exit!(102, "ERROR: Could not read included file {path:?}: {e}");
                            }
                        }
                    } else if line.contains("`!now") {
                        // !now
                        let line = line
                            .replace("`!now`", &now)
                            .replace("`!now:local`", &now_local)
                            .replace("`!now:x`", &now_x);
                        let line = NOW.replace_all(&line, |c: &Captures| {
                            if c[1].contains(':') {
                                let (t, f) = c[1].split_once(':').unwrap();
                                d.format(&Some(dtg_lib::Format::custom(f)), &tz(t).ok())
                            } else {
                                d.default(&tz(&c[1]).ok())
                            }
                        });
                        let line = line.replace("`\\!now", "`!now");
                        println!("{line}");
                    } else {
                        let line = line.replace("`\\!now", "`!now");
                        println!("{line}");
                    }
                }
                std::env::set_current_dir(&original_dir)
                    .unwrap_or_else(|e| panic!("Could not change directory to {dir:?}: {e}"));
            }
            Err(e) => {
                exit!(101, "ERROR: Could not read input file {input_file:?}: {e}");
            }
        }
    }
    Ok(())
}

/// Run a command and return its stdout, stderr, and exit code
fn pipe<T: AsRef<std::ffi::OsStr> + std::fmt::Display>(
    command: T,
) -> (String, String, Option<i32>) {
    let mut child = shell(command);
    child.stdout(Stdio::piped());
    child.stderr(Stdio::piped());
    let output = child.execute_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let code = output.status.code();
    (stdout, stderr, code)
}

/// Run a command and print its stderr and stdout
fn run<T: AsRef<std::ffi::OsStr> + std::fmt::Display>(command: T) {
    let (stdout, stderr, _code) = pipe(command);
    print!("{stderr}{stdout}");
}
