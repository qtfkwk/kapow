# About

Kapow is a template processor that provides the following directives to support injecting file
contents, command output, current date/time, etc in generated files.

## Block directives

Directive | Description
---|---
`!inc:path` | Include file contents; path is relative to its containing file
`!run:command` | Run command and insert stdout

* Block directives must be placed at the beginning of a line.
* Long commands can be backslash-wrapped.

## Span directives

Directive | Example | Description
---|---|---
`` `\!now` `` | `!now` | Current date/time in UTC / RFC 3339
`` `\!now:local` `` | `!now:local` | Current date/time in local timezone
`` `\!now:local:%A %H:%M` `` | `!now:local:%A %H:%M` | Current date/time in local timezone and custom format
`` `\!now:MST7MDT` `` | `!now:MST7MDT` | Current date/time in custom timezone
`` `\!now:MST7MDT:%A %H:%M` `` | `!now:MST7MDT:%A %H:%M` | Current date/time in custom timezone and format
`` `\!now:US/Hawaii` `` | `!now:US/Hawaii` | Current date/time in custom locale
`` `\!now:US/Hawaii:%A %H:%M` `` | `!now:US/Hawaii:%A %H:%M` | Current date/time in custom locale and format
`` `\!now:UTC:%A %H:%M` `` | `!now:UTC:%A %H:%M` | Current date/time in UTC and custom format
`` `\!now:x` `` | `!now:x` | Current date/time in "x" format
`` `\!today` `` | `!today` | Current date in UTC / RFC 3339
`` `\!today:local` `` | `!today:local` | Current date in local timezone
`` `\!today:MST7MDT` `` | `!today:MST7MDT` | Current date in custom timezone
`` `\!today:MST7MDT:%v` `` | `!today:MST7MDT:%v` | Current date in custom timezone and format
`` `\!today:US/Hawaii` `` | `!today:US/Hawaii` | Current date in custom locale
`` `\!today:US/Hawaii:%x` `` | `!today:US/Hawaii:%x` | Current date in custom locale and format
`` `\!today:UTC:%A` `` | `!today:UTC:%A` | Current date in UTC and custom format

* Span directives must be placed inside a code span and may appear zero or more times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Usage

!inc:VERSION.md
!inc:USAGE.md
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
    * `` `\!now` `` (all variants)

# Changelog

* 1.0.0 (2023-02-21): Initial release
* 1.0.1 (2023-02-21): Update dependencies
* 2.0.0 (2023-02-22): Include stderr in `!run:` output;
  enable backslash-wrapping long commands;
  include whitespace in included files;
  process path of included files relative to the containing file instead of the current directory;
  input and included file processed more efficiently via `BufReader` instead of `read_to_string`;
  improved error handling
* 2.1.0 (2023-02-24): Use `bat -pl md` as readme pager if have it installed; update dependencies
* 2.1.1 (2023-02-24): Fix readme
* 2.1.2 (2023-02-24): Fix readme
* 2.2.0 (2023-03-10): Change to the directory of each input file in order to process included file
  paths and commands to be run relative to the input file path;
  run commands via the shell to enable more advanced commands and simplify usage;
  update dependencies
* 2.2.1 (2023-03-10): Remove shlex dependency
* 2.2.2 (2023-03-10): Fix readme; error if no input file(s) provided
* 2.2.3 (2023-03-10): Fix confict with readme option
* 2.2.4 (2023-03-10): Fix readme
* 2.3.0 (2023-03-11): Add `\!today` span directive; improved exit macro; change directory function;
  where clauses; fix watch task; fix readme
* 2.3.1 (2023-03-11): Fix readme
* 2.4.0 (2023-03-13): Enable processing from stdin if no input files provided or input file is `-`;
  update dependencies

# Development

```bash
cargo install bat cargo-edit cargo-make cargo-outdated dtg kapow \
miniserve
```

