use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use dtg_lib::{tz, Dtg, Format};
use lazy_static::lazy_static;
use pager::Pager;
use regex::{Captures, Regex};
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

const WRAP: usize = 66;

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
    /// Page output
    #[arg(short, conflicts_with = "no_page")]
    page: bool,

    /// Do not page output
    #[arg(short = 'P', conflicts_with = "page")]
    no_page: bool,

    /// Disable syntax highlighting
    #[arg(short = 'H', conflicts_with = "lang")]
    no_lang: bool,

    /// Syntax higlight language
    #[arg(short, conflicts_with = "no_lang", default_value = "md")]
    lang: String,

    /// Print readme
    #[arg(short, long, conflicts_with = "input_files")]
    readme: bool,

    /// Source file(s)
    #[arg(value_name = "PATH", default_value = "-")]
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
        page(true, cli.no_page, cli.no_lang, &cli.lang);
        print!("{}", include_str!("../README.md"));
        exit!(0);
    }
    page(false, cli.page, cli.no_lang, &cli.lang);
    let start = Utc::now();
    let d = Dtg::from_dt(&Utc.timestamp_opt(start.timestamp(), 0).unwrap());
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
                let line = line.unwrap();
                process_line(
                    &line,
                    &mut command_q,
                    &start,
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
                for (i, line) in f.lines().enumerate() {
                    let line = line.unwrap();
                    if i == 0 && line.starts_with("#!") {
                        continue;
                    }
                    process_line(
                        &line,
                        &mut command_q,
                        &start,
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
    line: &str,
    command_q: &mut Vec<String>,
    start: &DateTime<Utc>,
    d: &Dtg,
    now: &str,
    now_local: &str,
    now_x: &str,
    today: &str,
    today_local: &str,
) {
    let mut line = line.to_string();
    loop {
        // Block directives...
        if !command_q.is_empty() {
            // !run:command \
            // args
            if let Some(command) = line.strip_suffix('\\') {
                command_q.push(command.to_string());
            } else {
                command_q.push(line.to_string());
                run(command_q.drain(..).collect::<String>());
            }
            break;
        } else if let Some(command) = line.strip_prefix("!run:") {
            // !run:command
            if let Some(command) = command.strip_suffix('\\') {
                command_q.push(command.to_string());
            } else {
                run(command);
            }
            break;
        } else if let Some(path) = line.strip_prefix("!inc:") {
            // !inc:path
            match std::fs::File::open(path) {
                Ok(f) => {
                    let f = std::io::BufReader::new(f);
                    for i in f.lines() {
                        println!("{}", i.unwrap());
                    }
                }
                Err(e) => {
                    exit!(102, "ERROR: Could not read included file {path:?}: {e}");
                }
            }
            break;
        // Span directives...
        } else if line.contains("`!elapsed") {
            // !elapsed
            line = line.replace("`!elapsed`", &human_duration(Utc::now() - *start));
        } else if line.contains("`!now") {
            // !now
            line = line
                .replace("`!now`", now)
                .replace("`!now:local`", now_local)
                .replace("`!now:x`", now_x);
            line = NOW
                .replace_all(&line, |c: &Captures| {
                    if c[1].contains(':') {
                        let (t, f) = c[1].split_once(':').unwrap();
                        d.format(&Some(Format::custom(f)), &tz(t).ok())
                    } else {
                        d.default(&tz(&c[1]).ok())
                    }
                })
                .to_string();
        } else if line.contains("`!today") {
            // !today
            line = line
                .replace("`!today`", today)
                .replace("`!today:local`", today_local);
            line = TODAY
                .replace_all(&line, |c: &Captures| {
                    if c[1].contains(':') {
                        let (t, f) = c[1].split_once(':').unwrap();
                        d.format(&Some(Format::custom(f)), &tz(t).ok())
                    } else {
                        d.format(&Some(Format::custom("%Y-%m-%d")), &tz(&c[1]).ok())
                    }
                })
                .to_string();
        // Done...
        } else {
            line = line
                .replace("`\\!elapsed", "`!elapsed")
                .replace("`\\!now", "`!now")
                .replace("`\\!today", "`!today");
            println!("{line}");
            break;
        }
    }
}

/**
Run a command and return its stdout, stderr, and exit code
*/
fn pipe<T: AsRef<str>>(command: T) -> (String, String, Option<i32>) {
    let child = std::process::Command::new("sh")
        .args(["-c", command.as_ref()])
        .output()
        .unwrap();
    let stdout = String::from_utf8(child.stdout).unwrap();
    let stderr = String::from_utf8(child.stderr).unwrap();
    let code = child.status.code();
    (stdout, stderr, code)
}

/**
Run a command and print its stderr and stdout
*/
fn run<T: AsRef<str>>(command: T) {
    let (stdout, stderr, _code) = pipe(command);
    let stderr = term_hard_wrap(&stderr, WRAP);
    let stdout = term_hard_wrap(&stdout, WRAP);
    print!("{stderr}{stdout}");
}

/**
Change directory
*/
fn cd(dir: &std::path::Path) {
    std::env::set_current_dir(dir)
        .unwrap_or_else(|e| exit!(103, "ERROR: Could not change directory to {dir:?}: {e}"));
}

/**
Wrap text with ANSI color codes to a fixed number of columns
*/
fn term_hard_wrap(s: &str, width: usize) -> String {
    fn len(s: &str) -> usize {
        std::str::from_utf8(&strip_ansi_escapes::strip(s.as_bytes()).unwrap())
            .unwrap()
            .grapheme_indices(true)
            .map(|(_offset, grapheme)| if grapheme == "\t" { 8 } else { 1 })
            .sum()
    }
    let w = width - 1;
    let mut r = vec![];
    for line in s.lines() {
        if len(line) <= width {
            r.push(format!("{line}\n"));
            continue;
        }
        let mut line = line;
        while len(line) > w {
            let mut i = w;
            let mut t = &line[..i];
            while len(t) < w {
                i += 1;
                t = &line[..i];
            }
            r.push(format!("{t}\\\n"));
            line = &line[i..];
        }
        r.push(format!("{line}\n"));
    }
    r.join("")
}

/**
Convert a Duration into a short human-readable string like `[Dd][Hh][Mm][Ss]`
*/
fn human_duration(duration: chrono::Duration) -> String {
    fn f(n: i64, abbr: &str) -> Option<String> {
        if n != 0 {
            Some(format!("{n}{abbr}"))
        } else if abbr == "s" {
            Some(String::from("0s"))
        } else {
            None
        }
    }
    [
        (duration.num_days(), "d"),
        (duration.num_hours(), "h"),
        (duration.num_minutes(), "m"),
        (duration.num_seconds(), "s"),
    ]
    .iter()
    .filter_map(|x| f(x.0, x.1))
    .collect::<Vec<String>>()
    .join("")
}

/**
Configure and set up the pager
*/
fn page(readme: bool, no_page: bool, no_lang: bool, lang: &str) {
    let mut pager = String::from("bat -p");
    if readme == no_page {
        pager.push('P');
    }
    if !(lang.is_empty() || no_lang) {
        pager.push_str("l ");
        pager.push_str(lang);
    }
    Pager::with_pager(&pager).setup();
}
