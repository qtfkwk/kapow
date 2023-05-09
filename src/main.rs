use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use dtg_lib::{tz, Dtg, Format};
use lazy_static::lazy_static;
use pager::Pager;
use regex::{Captures, Regex};
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

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

    /// Wrap !run directive columns
    #[arg(short, default_value = "0")]
    wrap: usize,

    /// Wrap !run directive continuation
    #[arg(short, value_name = "STRING", default_value = "\\")]
    continuation: String,

    /// Ignore !run directive failures
    #[arg(short = 'k')]
    ignore_run_fail: bool,

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

    static ref FENCE_RE: Regex = Regex::new(r"^(\s*(```+|~~~+))").unwrap();
}

/**
Main function
*/
fn main() {
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
    let mut fence = None;
    let mut command_q = vec![];
    let original_dir = std::env::current_dir().expect("current directory");
    for input_file in &cli.input_files {
        if input_file == std::path::Path::new("-") {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                let line = line.unwrap();
                fence = update_fence(&line, fence);
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
                    &fence,
                    cli.ignore_run_fail,
                    cli.wrap,
                    &cli.continuation,
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
                    fence = update_fence(&line, fence);
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
                        &fence,
                        cli.ignore_run_fail,
                        cli.wrap,
                        &cli.continuation,
                    );
                }
                cd(&original_dir);
            }
            Err(e) => {
                exit!(101, "ERROR: Could not read input file {input_file:?}: {e}");
            }
        }
    }
}

fn update_fence(line: &str, fence: Option<String>) -> Option<String> {
    if let Some(m) = FENCE_RE.find(line) {
        if let Some(s) = &fence {
            if m.as_str() == s {
                return None;
            }
        } else {
            return Some(m.as_str().to_owned());
        }
    }
    fence
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
    fence: &Option<String>,
    ignore_run_fail: bool,
    wrap: usize,
    continuation: &str,
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
                run(
                    command_q.drain(..).collect::<String>(),
                    fence,
                    ignore_run_fail,
                    wrap,
                    continuation,
                );
            }
            break;
        } else if let Some(command) = line.strip_prefix("!run:") {
            // !run:command
            if let Some(command) = command.strip_suffix('\\') {
                command_q.push(command.to_string());
            } else {
                run(command, fence, ignore_run_fail, wrap, continuation);
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
fn run<T: AsRef<str>>(
    command: T,
    fence: &Option<String>,
    ignore_run_fail: bool,
    wrap: usize,
    continuation: &str,
) {
    let command = command.as_ref();
    let (stdout, stderr, code) = pipe(command);
    let stderr = term_hard_wrap(&stderr, wrap, continuation);
    let stdout = term_hard_wrap(&stdout, wrap, continuation);
    print!("{stderr}{stdout}");
    if !ignore_run_fail && code != Some(0) {
        if let Some(s) = fence {
            println!("{s}");
        }
        println!("\n---\n\nERROR: Command `{command}` exited with error code `{code:?}`\n");
        exit!(104);
    }
}

/**
Change directory
*/
fn cd(dir: &std::path::Path) {
    if !["", "."].iter().any(|x| std::path::Path::new(x) == dir) {
        std::env::set_current_dir(dir)
            .unwrap_or_else(|e| exit!(103, "ERROR: Could not change directory to {dir:?}: {e}"));
    }
}

/**
Wrap text with ANSI color codes to a fixed number of columns
*/
fn term_hard_wrap(s: &str, width: usize, continuation: &str) -> String {
    if width == 0 {
        return s.to_string();
    }

    // Input without ANSI color codes to graphemes
    let b = String::from_utf8(strip_ansi_escapes::strip(s.as_bytes()).unwrap()).unwrap();
    let mut gb = b.graphemes(true).collect::<Vec<_>>();

    // Input graphemes
    let mut ga = s.graphemes(true).collect::<Vec<_>>();

    // Continuation graphemes
    let gc = continuation.graphemes(true).collect::<Vec<_>>();
    let cw = gc.len();
    let cwp = cw + 1;
    let w = width - cw; // initial max width; leave space for the continuation

    let mut r = String::new(); // result
    let mut l = 0; // current column
    let mut ca; // current grapheme from `ga`
    let mut cb; // current grapheme from `gb`

    // Internal function update state
    fn update(ca: &str, r: &mut String, l: &mut usize, counts: bool) {
        if ca == "\t" {
            // Tab counts as a color code for some reason, so it always counts
            // against the line length.
            // `8 - *l % 8` is he number of spaces from the current column to
            // the nearest multiple of 8.
            for _ in 0..(8 - *l % 8) {
                *l += 1;
                r.push(' ');
            }
        } else {
            // Not a tab, so it only counts if not a color code.
            if counts {
                *l = if ca == "\n" { 0 } else { *l + 1 };
            }
            r.push_str(ca);
        }
    }

    while !ga.is_empty() {
        // Process line until the initial max width
        while l < w {
            if ga.is_empty() {
                break;
            }
            ca = ga.remove(0);
            cb = gb.remove(0);

            // Process extra ANSI color code graphemes
            while ca != cb && !ga.is_empty() {
                update(ca, &mut r, &mut l, false);
                ca = ga.remove(0);
            }

            // Process regular graphemes
            update(ca, &mut r, &mut l, true);
        }

        // Process the last character(s) of the line
        let gal = ga.len();
        if gal == cw || (gal == cwp && ga[1] == "\n") {
            // Continuation width characters left in the input or the current line, so just push
            // them
            r.push_str(ga.remove(0));
        } else if gal != 0 {
            // Unless done, insert the continuation
            r.push_str(continuation);
            r.push('\n');
        }
        l = 0;
    }

    r
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
