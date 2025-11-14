# Changelog

* 1.0.0 (2023-02-21): Initial release
* 1.0.1 (2023-02-21): Update dependencies
* 2.0.0 (2023-02-22): Include stderr in `!run:` output; enable backslash-wrapping long commands; include whitespace in included files; process path of included files relative to the containing file instead of the current directory; input and included file processed more efficiently via `BufReader` instead of `read_to_string`; improved error handling
* 2.1.0 (2023-02-24): Use `bat -pl md` as readme pager if have it installed; update dependencies
* 2.1.1 (2023-02-24): Fix readme
* 2.1.2 (2023-02-24): Fix readme
* 2.2.0 (2023-03-10): Change to the directory of each input file in order to process included file paths and commands to be run relative to the input file path; run commands via the shell to enable more advanced commands and simplify usage; update dependencies
* 2.2.1 (2023-03-10): Remove shlex dependency
* 2.2.2 (2023-03-10): Fix readme; error if no input file(s) provided
* 2.2.3 (2023-03-10): Fix confict with readme option
* 2.2.4 (2023-03-10): Fix readme
* 2.3.0 (2023-03-11): Add `\!today` span directive; improved exit macro; change directory function; where clauses; fix watch task; fix readme
* 2.3.1 (2023-03-11): Fix readme
* 2.4.0 (2023-03-13): Enable processing from stdin if no input files provided or input file is `-`; update dependencies
* 2.5.0 (2023-05-02): Ignore shebang line; fold command output with ANSI color codes to 66 columns; add `\!elapsed` directive; fix issue when more than 1 different span directives are on a line; improve readme; update dependencies
* 2.6.0 (2023-05-03): Improved [`bat`] integration and added `-p`, `-P`, `-H`, `-l` options; improve readme; `CHANGELOG.md`; eliminate `execute` dependency; update dependencies
* 2.6.1 (2023-05-03): Fix changelog, help
* 2.6.2 (2023-05-04): Fix bug: `ERROR: Could not change directory to "": No such file or directory (os error 2)` when running `kapow file.ext`; update dependencies
* 2.7.0 (2023-05-05): Add new default behaviour to exit with an error if a `!run` directive fails and `-k` to *keep going* instead; update the Errors section of the readme
* 2.7.1 (2023-05-08): Fix bug folding command output to columns when output contains wide Unicode characters; update dependencies
* 2.8.0 (2023-05-09): Refactor fold command output to columns feature and add the `-w` and `-c` options to control it; scrub the readme
* 2.8.1 (2023-05-09): Scrub readme; update dependencies
* 2.8.2 (2023-05-09): Set default wrap !run directive columns to 0 (don't wrap)
* 2.9.0 (2023-05-10): Add `-L` option to list syntax highlight languages via [`bat`]; enable unbuffered output for !run directives if not wrapping
* 2.9.1 (2023-05-10): Update dependencies
* 2.10.0 (2023-05-12): Use [`termwrap`] crate and remove `strip-ansi-escapes` and `unicode-segmentation` dependencies; update dependencies
* 2.12.0 (2023-10-27): Update dependencies; add escaped wrap feature
* 2.13.0 (2023-10-28): Add details about escaped wrap and CLI usage to readme; add example `build.md` with shebang kapow
* 2.14.0 (2023-11-01): Recursive include; update dependencies
* 2.15.0 (2023-11-01): Remove pager dependency on windows
* 2.16.0 (2023-11-01): Add `-f` option
* 2.16.1 (2023-11-01): Fix readme
* 2.17.0 (2023-11-03): Add relative image path feature and `-R` option to disable it; eliminate multiple empty lines; avoid calling page function on windows; fix readme
* 2.17.1 (2023-11-03): Fix issue with pager on windows
* 2.18.0 (2023-11-09): Fix issue with folder separators on windows; update dependencies
* 2.19.0 (2023-11-13): Convert cargo-make Makefile.toml to mkrs Makefile.md; update dependencies
    * 2.19.1 (2024-07-23): Update dependencies; fix makefile
* 2.20.0 (2024-08-22): Remove [`chrono`] dependency; add `commit` target to makefile; fix makefile; update dependencies
    * 2.20.1 (2024-08-23): Update dependencies; fix makefile
* 2.21.0 (2024-10-24): Update dependencies
    * 2.21.1 (2024-12-04): Update dependencies
    * 2.21.2 (2025-02-20): Update dependencies
    * 2.21.3 (2025-04-16): Update dependencies
* 2.22.0 (2025-08-28): Update dependencies; 2024 edition
    * 2.22.1 (2025-10-27): Update dependencies
    * 2.22.2 (2025-10-27): Use `pager2`
    * 2.22.3 (2025-11-12): Update dependencies; use [`clap-cargo`] `CLAP_STYLING`; clippy fixes
    * 2.22.4 (2025-11-14): Update dependencies; add `clippy::pedantic` to `cargo clippy` command in the `clippy` target in the makefile

[`bat`]: https://crates.io/crates/bat
[`clap-cargo`]: https://crates.io/crates/clap-cargo
[`termwrap`]: https://crates.io/crates/termwrap

