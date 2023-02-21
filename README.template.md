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
`` `\!now` `` | `!now` | Current date/time in UTC / RFC 3339
`` `\!now:local` `` | `!now:local` | Current date/time in local timezone
`` `\!now:local:%A %H:%M` `` | `!now:local:%A %H:%M` | Current date/time in local timezone and custom format
`` `\!now:MST7MDT` `` | `!now:MST7MDT` | Current date/time in custom timezone
`` `\!now:MST7MDT:%A %H:%M` `` | `!now:MST7MDT:%A %H:%M` | Current date/time in custom timezone and format
`` `\!now:US/Hawaii` `` | `!now:US/Hawaii` | Current date/time in custom locale
`` `\!now:US/Hawaii:%A %H:%M` `` | `!now:US/Hawaii:%A %H:%M` | Current date/time in custom locale and format
`` `\!now:UTC:%A %H:%M` `` | `!now:UTC:%A %H:%M` | Current date/time in UTC and custom format
`` `\!now:x` `` | `!now:x` | Current date/time in "x" format

* Span directives must be placed inside a code span and may appear zero or more times in any line
* Disable processing a span directive by escaping `!` with a backslash: `\!`

# Usage

!inc:VERSION.md

!inc:USAGE.md

# Example

`readme` task in `Makefile.toml`:

* Generate `VERSION.md` from `VERSION.template.md`
    * `!run:./target/release/kapow -V`
* Generate `USAGE.md` from `USAGE.template.md`
    * `!run:./target/release/kapow -h`
* Generate `README.md` from `README.template.md`
    * `!inc:VERSION.md`
    * `!inc:USAGE.md`
    * `` `\!now` ``

# Changelog

* 1.0.0 (2023-02-21): Initial release
* 1.0.1 (2023-02-21): Update dependencies

