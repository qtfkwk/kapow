# Changelog

* 1.0.0 (2023-02-21): Initial release
* 1.0.1 (2023-02-21): Update dependencies
* 2.0.0 (2023-02-22): Include stderr in `!run:` output;
  enable backslash-wrapping long commands;
  include whitespace in included files;
  process path of included files relative to the containing file instead of the
  current directory; input and included file processed more efficiently via
  `BufReader` instead of `read_to_string`; improved error handling
* 2.1.0 (2023-02-24): Use `bat -pl md` as readme pager if have it installed;
  update dependencies
* 2.1.1 (2023-02-24): Fix readme
* 2.1.2 (2023-02-24): Fix readme
* 2.2.0 (2023-03-10): Change to the directory of each input file in order to
  process included file paths and commands to be run relative to the input file
  path; run commands via the shell to enable more advanced commands and simplify
  usage; update dependencies
* 2.2.1 (2023-03-10): Remove shlex dependency
* 2.2.2 (2023-03-10): Fix readme; error if no input file(s) provided
* 2.2.3 (2023-03-10): Fix confict with readme option
* 2.2.4 (2023-03-10): Fix readme
* 2.3.0 (2023-03-11): Add `\!today` span directive; improved exit macro; change
  directory function; where clauses; fix watch task; fix readme
* 2.3.1 (2023-03-11): Fix readme
* 2.4.0 (2023-03-13): Enable processing from stdin if no input files provided or
  input file is `-`; update dependencies
* 2.5.0 (2023-05-02): Ignore shebang line; fold command output with ANSI color
  codes to 66 columns; add `\!elapsed` directive; fix issue when more than 1
  different span directives are on a line; improve readme; update dependencies
* 2.6.0 (2023-05-03): Improved [`bat`] integration and added `-p`, `-P`, `-H`,
  `-l` options; improve readme; [`CHANGELOG.md`]; eliminate `execute`
  dependency; update dependencies
* 2.6.1 (2023-05-03): Fix changelog, help
* 2.6.2 (2023-05-04): Fix bug: `ERROR: Could not change directory to "": No such
  file or directory (os error 2)` when running `kapow file.ext`; update
  dependencies
* 2.7.0 (2023-05-05): Add new default behaviour to exit with an error if a
  `!run` directive fails and `-k` to *keep going* instead; update the Errors
  section of the readme

[`CHANGELOG.md`]: CHANGELOG.md
[`bat`]: https://crates.io/crates/bat

