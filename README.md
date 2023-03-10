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
`` `!now` `` | 2023-03-10T17:50:15Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Fri 10 Mar 2023 12:50:15 EST | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Friday 12:50 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Fri 10 Mar 2023 10:50:15 MST | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Friday 10:50 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Fri 10 Mar 2023 07:50:15 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Friday 07:50 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Friday 17:50 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh29HoF | Current date/time in "x" format

* Span directives must be placed inside a code span and may appear zero or more times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Usage

```text
$ kapow -V
kapow 2.2.3
```

```text
$ kapow -h
KAPOW!

Usage: kapow [OPTIONS] <INPUT_FILES>...

Arguments:
  <INPUT_FILES>...  Input file(s)

Options:
  -r, --readme   Print readme
  -h, --help     Print help
  -V, --version  Print version
```

# Example

See the `readme` task in `Makefile.toml`:

* Generates `t/version/VERSION.md` from `t/version/VERSION.t.md`
    * `!run:../target/release/kapow -V`
* Generates `t/usage/USAGE.md` from `t/usage/USAGE.t.md`
    * `!run:../target/release/kapow -h`
* Generates `README.md` from `t/README.md`
    * `!inc:version/VERSION.md`
    * `!inc:usage/USAGE.md`
    * `` `!now` `` (all variants)

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

# Development

```bash
cargo install bat cargo-edit cargo-make cargo-outdated dtg kapow \
miniserve
```

