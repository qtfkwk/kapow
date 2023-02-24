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
`` `!now` `` | 2023-02-24T19:22:36Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Fri 24 Feb 2023 14:22:36 EST | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Friday 14:22 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Fri 24 Feb 2023 12:22:36 MST | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Friday 12:22 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Fri 24 Feb 2023 09:22:36 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Friday 09:22 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Friday 19:22 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh1NJMa | Current date/time in "x" format

* Span directives must be placed inside a code span and may appear zero or more times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Usage

```text
$ kapow -V
kapow 2.1.0
```

```text
$ kapow -h
KAPOW!

Usage: kapow [OPTIONS] [INPUT_FILES]...

Arguments:
  [INPUT_FILES]...  Input file(s)

Options:
  -r, --readme   Print readme
  -h, --help     Print help
  -V, --version  Print version
```

# Example

See the `readme` task in `Makefile.toml`:

* Generates `t/version/VERSION.md` from `t/version/VERSION.template.md`
    * `!run:./target/release/kapow -V`
* Generates `t/usage/USAGE.md` from `t/usage/USAGE.template.md`
    * `!run:./target/release/kapow -h`
* Generates `README.md` from `t/README.template.md`
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
* 2.0.1 (2023-02-24): Use `bat -pl md` as readme pager if have it installed; update dependencies

# Development

```bash
cargo install cargo-edit cargo-make cargo-outdated dtg miniserve
```

