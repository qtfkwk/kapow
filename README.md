# About

Kapow is a template processor that provides the following directives to support injecting file
contents, command output, current date/time, etc in generated files.

## Block directives

Directive | Description
---|---
`!inc:path` | Include file contents
`!run:command` | Run command and insert stdout

* Block directives must be placed at the beginning of a line

## Span directives

Directive | Example | Description
---|---|---
`` `!now` `` | 2023-02-21T15:26:23Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Tue 21 Feb 2023 10:26:23 EST | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Tuesday 10:26 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Tue 21 Feb 2023 08:26:23 MST | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Tuesday 08:26 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Tue 21 Feb 2023 05:26:23 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Tuesday 05:26 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Tuesday 15:26 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh1KFQN | Current date/time in "x" format

* Span directives must be placed inside a code span and may appear zero or more times in any line
* Disable processing a span directive by escaping `!` with a backslash: `\!`

# Usage

```text
$ kapow -V
kapow 1.0.0
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

`readme` task in `Makefile.toml`:

* Generate `VERSION.md` from `VERSION.template.md`
    * `!run:./target/release/kapow -V`
* Generate `USAGE.md` from `USAGE.template.md`
    * `!run:./target/release/kapow -h`
* Generate `README.md` from `README.template.md`
    * `!inc:VERSION.md`
    * `!inc:USAGE.md`
    * `` `!now` ``

