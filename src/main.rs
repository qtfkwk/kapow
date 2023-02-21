use clap::Parser;
use dtg_lib::{tz, Dtg};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

/// KAPOW!
#[derive(Parser)]
#[command(version, max_term_width = 80)]
struct Cli {
    /// Print readme
    #[arg(short, long)]
    readme: bool,

    /// Input file(s)
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
        println!("{}", include_str!("../README.md"));
        std::process::exit(0);
    }
    let d = Dtg::now();
    let now = d.rfc_3339();
    let now_x = d.x_format();
    let now_local = d.default(&tz("local").ok());
    for input_file in &cli.input_files {
        let data = std::fs::read_to_string(input_file).unwrap();
        for line in data.lines() {
            if let Some(command) = line.strip_prefix("!run:") {
                // !run:command
                println!("{}", pipe(command).0.trim());
            } else if let Some(path) = line.strip_prefix("!inc:") {
                // !inc:path
                println!("{}", std::fs::read_to_string(path).unwrap().trim());
            } else {
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
            }
        }
    }
    Ok(())
}

/// Run a command and return its stdout, stderr, and exit code
fn pipe<T: AsRef<str>>(command: T) -> (String, String, Option<i32>) {
    let (program, args) = split(command);
    let child = std::process::Command::new(program)
        .args(&args)
        .output()
        .unwrap();
    let stdout = String::from_utf8(child.stdout).unwrap();
    let stderr = String::from_utf8(child.stderr).unwrap();
    let code = child.status.code();
    (stdout, stderr, code)
}

/// Split a command into program and arguments
///
/// * Resolve single and double-quoted arguments
fn split<T: AsRef<str>>(command: T) -> (String, Vec<String>) {
    let mut args = shlex::split(command.as_ref()).unwrap();
    let program = args.remove(0);
    (program, args)
}
