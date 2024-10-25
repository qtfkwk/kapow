use clap::{builder::Styles, Parser};
use dtg_lib::{tz, Dtg, Format};
use lazy_static::lazy_static;
use path_slash::PathExt;
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use termwrap::termwrap;
use which::which;

#[cfg(unix)]
use pager::Pager;

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

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

/// KAPOW!
#[derive(Parser)]
#[command(version, max_term_width = 80, styles = STYLES)]
struct Cli {
    /// Flags (comma-separated list of flags to enable)
    #[arg(short)]
    flags: Option<String>,

    /// Page output
    #[arg(short, conflicts_with = "no_page")]
    page: bool,

    /// Do not page output
    #[arg(short = 'P', conflicts_with = "page")]
    no_page: bool,

    /// Disable syntax highlighting
    #[arg(short = 'H', conflicts_with = "lang")]
    no_lang: bool,

    /// Display all syntax highlight languages
    #[arg(short = 'L')]
    list_languages: bool,

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

    /// Disable relative image paths
    #[arg(short = 'R')]
    no_relative_images: bool,

    /// Print readme
    #[arg(short, long)]
    readme: bool,

    /// Source file(s)
    #[arg(value_name = "PATH", default_value = "-")]
    input_files: Vec<PathBuf>,
}

lazy_static! {
    /// Regular expression for `!now` directive
    static ref NOW: Regex = Regex::new(r"`!now:([^`]*)`").unwrap();

    /// Regular expression for `!today` directive
    static ref TODAY: Regex = Regex::new(r"`!today:([^`]*)`").unwrap();

    /// Regular expression for a markdown code block fence
    static ref FENCE_RE: Regex = Regex::new(r"^(\s*(```+|~~~+))").unwrap();

    /// Regular expression for a markdown image `![alt](path "title")`
    static ref IMAGE: Regex = Regex::new(r#"!\[([^\]]*)\]\(([^\) ]+)( *"["]*"|)\)"#).unwrap();
}

/**
Main function
*/
fn main() {
    let cli = Cli::parse();
    if cli.readme {
        #[cfg(unix)]
        page(true, cli.no_page, cli.no_lang, &cli.lang);

        print!("{}", include_str!("../README.md"));
        exit!(0);
    }
    if cli.list_languages {
        if which("bat").is_ok() {
            run_("bat -L");
            exit!(0);
        } else {
            exit!(105, "ERROR: Could not find a `bat` executable in PATH");
        }
    }

    #[cfg(unix)]
    page(false, cli.page, cli.no_lang, &cli.lang);

    let active_flags = if let Some(flags) = cli.flags {
        flags
            .split(',')
            .map(|x| x.to_string())
            .collect::<HashSet<String>>()
    } else {
        HashSet::new()
    };
    let d = Dtg::now();
    let now = d.rfc_3339();
    let now_x = d.x_format();
    let local_tz = tz("local").unwrap();
    let now_local = d.default(&Some(local_tz.clone()));
    let today = d.format(&Some(Format::custom("%Y-%m-%d")), &None);
    let today_local = d.format(&Some(Format::custom("%Y-%m-%d")), &Some(local_tz));
    let mut fence = None;
    let mut command_q = vec![];
    let mut flags = vec![];
    let root_dir = std::env::current_dir().unwrap();
    let mut prev_line = None;
    for input_file in &cli.input_files {
        if input_file == Path::new("-") {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                let line = line.unwrap();
                fence = update_fence(&line, fence);
                prev_line = process_line(
                    &line,
                    &mut command_q,
                    &mut flags,
                    &active_flags,
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
                    !cli.no_relative_images,
                    &root_dir,
                    &root_dir,
                    &prev_line,
                );
            }
            continue;
        } else {
            (fence, prev_line) = process_file(
                input_file,
                fence,
                &mut command_q,
                &mut flags,
                &active_flags,
                &d,
                &now,
                &now_local,
                &now_x,
                &today,
                &today_local,
                cli.ignore_run_fail,
                cli.wrap,
                &cli.continuation,
                !cli.no_relative_images,
                &root_dir,
                &root_dir,
                &prev_line,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_file(
    input_file: &Path,
    fence: Option<String>,
    command_q: &mut Vec<String>,
    flags: &mut Vec<String>,
    active_flags: &HashSet<String>,
    d: &Dtg,
    now: &str,
    now_local: &str,
    now_x: &str,
    today: &str,
    today_local: &str,
    ignore_run_fail: bool,
    wrap: usize,
    continuation: &str,
    relative_images: bool,
    dir: &Path,
    root_dir: &Path,
    prev_line: &Option<String>,
) -> (Option<String>, Option<String>) {
    let mut fence = fence.clone();
    let mut prev_line = prev_line.clone();
    match std::fs::File::open(input_file) {
        Ok(f) => {
            let f = std::io::BufReader::new(f);
            let input_file = if input_file.is_relative() {
                dir.join(input_file)
            } else {
                input_file.to_owned()
            };
            let dir = input_file.parent().unwrap();
            let orig_dir = cd(dir);
            for (i, line) in f.lines().enumerate() {
                let line = line.unwrap();
                if i == 0 && line.starts_with("#!") {
                    continue;
                }
                fence = update_fence(&line, fence);
                prev_line = process_line(
                    &line,
                    command_q,
                    flags,
                    active_flags,
                    d,
                    now,
                    now_local,
                    now_x,
                    today,
                    today_local,
                    &fence,
                    ignore_run_fail,
                    wrap,
                    continuation,
                    relative_images,
                    dir,
                    root_dir,
                    &prev_line,
                );
            }
            cd(&orig_dir);
        }
        Err(e) => {
            exit!(101, "ERROR: Could not read input file {input_file:?}: {e}");
        }
    }
    (fence, prev_line)
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
    flags: &mut Vec<String>,
    active_flags: &HashSet<String>,
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
    relative_images: bool,
    dir: &Path,
    root_dir: &Path,
    prev_line: &Option<String>,
) -> Option<String> {
    let mut line = line.to_string();
    let mut prev_line = prev_line.clone();
    if !flags.is_empty() {
        let flag = flags.last().unwrap();
        if line != format!("!stop:{flag}") {
            return prev_line;
        }
    }
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
            #[allow(unused_assignments)]
            {
                (_, prev_line) = process_file(
                    &PathBuf::from(path),
                    fence.clone(),
                    command_q,
                    flags,
                    active_flags,
                    d,
                    now,
                    now_local,
                    now_x,
                    today,
                    today_local,
                    ignore_run_fail,
                    wrap,
                    continuation,
                    relative_images,
                    dir,
                    root_dir,
                    &prev_line,
                );
            }
            return prev_line;
        } else if let Some(flag) = line.strip_prefix("!start:") {
            let flag = flag.to_string();
            if !active_flags.contains(&flag) {
                flags.push(flag);
            }
            return prev_line;
        } else if let Some(flag) = line.strip_prefix("!stop:") {
            let flag = flag.to_string();
            if !active_flags.contains(&flag) {
                let last = flags.pop().unwrap();
                if flag != last {
                    panic!();
                }
            }
            return prev_line;
        // Span directives...
        } else if line.contains("`!elapsed`") {
            // !elapsed
            line = line.replace("`!elapsed`", &d.elapsed().unwrap().to_string());
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
            // Process image paths relative to their containing file
            if relative_images {
                let mut line2 = line.clone();
                for (full, [alt, url, title]) in IMAGE.captures_iter(&line).map(|c| c.extract()) {
                    if !url.starts_with("http") {
                        let new_url = dir
                            .join(url)
                            .strip_prefix(root_dir)
                            .unwrap()
                            .to_slash()
                            .unwrap()
                            .to_string();
                        let new_full = format!("![{alt}]({new_url}{title})");
                        line2 = line2.replace(full, &new_full);
                    }
                }
                line = line2;
            }

            // Process escaped span directives
            line = line
                .replace("`\\!elapsed", "`!elapsed")
                .replace("`\\!now", "`!now")
                .replace("`\\!today", "`!today");

            // Process escaped wraps
            line = if let Some(s) = line.strip_suffix("\\\\") {
                format!("{s}\\\n")
            } else if let Some(s) = line.strip_suffix('\\') {
                s.to_string()
            } else {
                format!("{line}\n")
            };

            // Eliminate multiple empty lines
            if prev_line.is_none() || prev_line.as_ref().unwrap() != "\n" || line != "\n" {
                print!("{line}");
            }

            break;
        }
    }
    Some(line)
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
Run a command and return its exit code
*/
fn run_<T: AsRef<str>>(command: T) -> Option<i32> {
    std::process::Command::new("sh")
        .args(["-c", command.as_ref()])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
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
    let code = if wrap == 0 {
        run_(command)
    } else {
        let (stdout, stderr, code) = pipe(command);
        let stderr = termwrap(&stderr, wrap, continuation);
        let stdout = termwrap(&stdout, wrap, continuation);
        print!("{stderr}{stdout}");
        code
    };
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
fn cd(dir: &Path) -> PathBuf {
    let r = std::env::current_dir().unwrap();
    if !["", "."].iter().any(|x| Path::new(x) == dir) {
        std::env::set_current_dir(dir)
            .unwrap_or_else(|e| exit!(103, "ERROR: Could not change directory to {dir:?}: {e}"));
    }
    r
}

#[cfg(unix)]
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
