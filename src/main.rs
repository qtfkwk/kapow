use clap::Parser;
use dtg_lib::{tz, Dtg, Format};
use execute::{shell, Execute};
use lazy_static::lazy_static;
use pager::Pager;
use regex::{Captures, Regex};
use std::io::BufRead;
use std::process::Stdio;

/**
Optionally print a message to stderr and exit with the given code
*/
macro_rules! exit {
    ($code:expr) => ({
        std::process::exit($code);
    });
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
    #[arg(default_value = "-")]
    input_files: Vec<std::path::PathBuf>,
}

lazy_static! {
    /// Regular expression for `!now` directive
    static ref NOW: Regex = Regex::new(r"`!now:([^`]*)`").unwrap();

    /// Regular expression for `!today` directive
    static ref TODAY: Regex = Regex::new(r"`!today:([^`]*)`").unwrap();
}

/**
Main function
*/
fn main() -> Result<(), String> {
    let cli = Cli::parse();
    if cli.readme {
        Pager::with_pager("bat -pl md").setup();
        print!("{}", include_str!("../README.md"));
        exit!(0);
    }
    let d = Dtg::now();
    let now = d.rfc_3339();
    let now_x = d.x_format();
    let local_tz = tz("local").unwrap();
    let now_local = d.default(&Some(local_tz));
    let today = d.format(&Some(Format::custom("%Y-%m-%d")), &None);
    let today_local = d.format(&Some(Format::custom("%Y-%m-%d")), &Some(local_tz));
    let mut command_q = vec![];
    let original_dir = std::env::current_dir().expect("current directory");
    for input_file in &cli.input_files {
        if input_file == std::path::Path::new("-") {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                process_line(
                    line,
                    &mut command_q,
                    &d,
                    &now,
                    &now_local,
                    &now_x,
                    &today,
                    &today_local,
                );
            }
            continue;
        }
        match std::fs::File::open(input_file) {
            Ok(f) => {
                let f = std::io::BufReader::new(f);
                let dir = input_file.parent().expect("input file parent");
                cd(dir);
                for line in f.lines() {
                    process_line(
                        line,
                        &mut command_q,
                        &d,
                        &now,
                        &now_local,
                        &now_x,
                        &today,
                        &today_local,
                    );
                }
                cd(&original_dir);
            }
            Err(e) => {
                exit!(101, "ERROR: Could not read input file {input_file:?}: {e}");
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn process_line(
    line: std::io::Result<String>,
    command_q: &mut Vec<String>,
    d: &Dtg,
    now: &str,
    now_local: &str,
    now_x: &str,
    today: &str,
    today_local: &str,
) {
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
            .replace("`!now`", now)
            .replace("`!now:local`", now_local)
            .replace("`!now:x`", now_x);
        let line = NOW.replace_all(&line, |c: &Captures| {
            if c[1].contains(':') {
                let (t, f) = c[1].split_once(':').unwrap();
                d.format(&Some(Format::custom(f)), &tz(t).ok())
            } else {
                d.default(&tz(&c[1]).ok())
            }
        });
        let line = line.replace("`\\!now", "`!now");
        println!("{line}");
    } else if line.contains("`!today") {
        // !today
        let line = line
            .replace("`!today`", today)
            .replace("`!today:local`", today_local);
        let line = TODAY.replace_all(&line, |c: &Captures| {
            if c[1].contains(':') {
                let (t, f) = c[1].split_once(':').unwrap();
                d.format(&Some(Format::custom(f)), &tz(t).ok())
            } else {
                d.format(&Some(Format::custom("%Y-%m-%d")), &tz(&c[1]).ok())
            }
        });
        let line = line.replace("`\\!today", "`!today");
        println!("{line}");
    } else {
        let line = line
            .replace("`\\!now", "`!now")
            .replace("`\\!today", "`!today");
        println!("{line}");
    }
}

/// Run a command and return its stdout, stderr, and exit code
fn pipe<T>(command: T) -> (String, String, Option<i32>)
where
    T: AsRef<std::ffi::OsStr> + std::fmt::Display,
{
    let mut child = shell(command);
    child.stdout(Stdio::piped());
    child.stderr(Stdio::piped());
    let output = child.execute_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let code = output.status.code();
    (stdout, stderr, code)
}

/**
Run a command and print its stderr and stdout
*/
fn run<T>(command: T)
where
    T: AsRef<std::ffi::OsStr> + std::fmt::Display,
{
    let (stdout, stderr, _code) = pipe(command);
    print!("{stderr}{stdout}");
}

/**
Change directory
*/
fn cd(dir: &std::path::Path) {
    std::env::set_current_dir(dir)
        .unwrap_or_else(|e| exit!(103, "ERROR: Could not change directory to {dir:?}: {e}"));
}
