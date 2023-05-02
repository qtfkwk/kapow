# About

Kapow is a template processor that provides the following directives to support
injecting file contents, command output, current date/time, etc in generated
files.

It can be used in a few different ways:

* Standard: Create a (Markdown) file with Kapow directives then run
  `kapow path/to/file.ext`.
* Shebang: Place `#!/usr/bin/env kapow` as the first line of a (Markdown) file,
  make it "executable" via `chmod +x path/to/file.ext`, then run via
  `cd path/to; ./file.ext` (see also note #3 under [block directives]).

Kapow is generally designed around the idea of using Markdown output formatting,
but the format could be any text format that supports the desired syntax.

NOTE: Installing [`bat`] and piping `kapow` output to `bat -pl md` adds nice
syntax highlighting to Markdown and paging (or add `-P` to disable the pager).

[block directives]: #block-directives
[`bat`]: https://crates.io/crates/bat

## Block directives

Directive | Description
---|---
`!inc:path` | Include file contents; path is relative to its containing file
`!run:command` | Run command and insert stdout

Notes:

1. Block directives must be placed at the beginning of a line.
2. Long commands can be backslash-wrapped.
3. Keep in mind that file paths and commands are processed from the directory
   where the source file is located.
   That is unless the source file is stdin (including shebang usage), so
   changing to the directory where the source file is located is a good idea.

## Span directives

Directive | Example | Description
---|---|---
`` `!elapsed` `` | 0s | Processing time
`` `!now` `` | 2023-05-02T17:20:03Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Tue 02 May 2023 13:20:03 EDT | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Tuesday 13:20 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Tue 02 May 2023 11:20:03 MDT | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Tuesday 11:20 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Tue 02 May 2023 07:20:03 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Tuesday 07:20 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Tuesday 17:20 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh41HK3 | Current date/time in "x" format
`` `!today` `` | 2023-05-02 | Current date in UTC / RFC 3339
`` `!today:local` `` | 2023-05-02 | Current date in local timezone
`` `!today:MST7MDT` `` | 2023-05-02 | Current date in custom timezone
`` `!today:MST7MDT:%v` `` |  2-May-2023 | Current date in custom timezone and format
`` `!today:US/Hawaii` `` | 2023-05-02 | Current date in custom locale
`` `!today:US/Hawaii:%x` `` | 05/02/23 | Current date in custom locale and format
`` `!today:UTC:%A` `` | Tuesday | Current date in UTC and custom format

* Span directives must be placed inside a code span and may appear zero or more
  times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Usage

```text
$ kapow -V
kapow 2.5.0
```

```text
$ kapow -h
KAPOW!

Usage: kapow [OPTIONS] [INPUT_FILES]...

Arguments:
  [INPUT_FILES]...  Input file(s) [default: -]

Options:
  -r, --readme   Print readme
  -h, --help     Print help
  -V, --version  Print version
```

* The `-r` option uses [`bat`] for paging and syntax highlighting if you have it
  installed (optional).

# Errors

Code | Description
---|---
101 | Could not read input file
102 | Could not read included file
103 | Could not change directory

# Example

See the `readme` task in `Makefile.toml`:

* Generates `t/VERSION.md` from `t/VERSION.t.md`
    * `!run:../target/release/kapow -V`
* Generates `t/USAGE.md` from `t/USAGE.t.md`
    * `!run:../target/release/kapow -h`
* Generates `README.md` from `t/README.md`
    * `!inc:VERSION.md`
    * `!inc:USAGE.md`
    * `` `!now` `` (all variants)

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
* 2.3.0 (2023-03-11): Add `!today` span directive; improved exit macro; change
  directory function; where clauses; fix watch task; fix readme
* 2.3.1 (2023-03-11): Fix readme
* 2.4.0 (2023-03-13): Enable processing from stdin if no input files provided or
  input file is `-`; update dependencies
* 2.5.0 (2023-05-02): Ignore shebash line; fold command output with ANSI color
  codes to 66 columns; add `!elapsed` directive; fix issue when more than 1
  different span directives are on a line; improve readme; update dependencies

# Development

```bash
cargo install bat cargo-edit cargo-make cargo-outdated dtg kapow \
miniserve
```

