# About

Kapow is a *template processor* that provides the following directives to
support injecting file contents, command output, current date/time, elapsed
time, etc in generated output.

It can be used in some different ways:

* Standard: Create a (Markdown) file with kapow directives then run
  `kapow [OPTIONS] path/to/file.ext`; optionally save the output via
  redirection: `kapow [OPTIONS] path/to/file.ext >output.ext`
* Shebang: Place `#!/usr/bin/env kapow` as the first line of a (Markdown) file,
  make it "executable" via `chmod +x file.ext`, then run via `./file.ext` (see
  note #3 under [block directives]).
  Use env's `-S` option if passing options to kapow, for example:
  `#!/usr/bin/env -S kapow -w 60`.

While kapow is designed around Markdown syntax, it can be used with any text
format that works with its directives.

[block directives]: #block-directives

## Block directives

Directive | Description
---|---
`!inc:path` | Include file contents; path is relative to its containing file
`!run:command` | Run command and insert stdout

Notes:

1. Block directives must be placed at the beginning of a line.
2. Long commands can be backslash-wrapped.
3. Included file paths and commands are processed from the directory where the
   source file is located when it is passed as an argument.
   However, if the source file is read on stdin or run via shebang, included
   file paths and commands are processed relative to the current directory.
   So, if any included files use relative paths or commands that depend on the
   current directory, it will be necessary to manually change to the source
   file's directory and then run via `./file.ext`.
4. Block directives are entirely replaced by their contents/output, so you are
   free to embed them inside or as Markdown syntax... for example, as listing
   contents, prepend a prompt showing the command, etc.
5. If a `!run` directive fails, kapow prints the error and stops processing,
   unless the user specifies the `-k` option.

## Span directives

Directive | Example | Description
---|---|---
`` `!elapsed` `` | 0s | Processing time
`` `!now` `` | 2023-05-09T20:26:06Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Tue 09 May 2023 16:26:06 EDT | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Tuesday 16:26 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Tue 09 May 2023 14:26:06 MDT | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Tuesday 14:26 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Tue 09 May 2023 10:26:06 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Tuesday 10:26 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Tuesday 20:26 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh48KQ6 | Current date/time in "x" format
`` `!today` `` | 2023-05-09 | Current date in UTC / RFC 3339
`` `!today:local` `` | 2023-05-09 | Current date in local timezone
`` `!today:MST7MDT` `` | 2023-05-09 | Current date in custom timezone
`` `!today:MST7MDT:%v` `` |  9-May-2023 | Current date in custom timezone and format
`` `!today:US/Hawaii` `` | 2023-05-09 | Current date in custom locale
`` `!today:US/Hawaii:%x` `` | 05/09/23 | Current date in custom locale and format
`` `!today:UTC:%A` `` | Tuesday | Current date in UTC and custom format

* Span directives must be placed inside a code span and may appear zero or more
  times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Install

```bash
cargo install kapow bat
```

NOTE: Installing [`bat`] is optional, but if installed, kapow uses it for syntax
highlighting and paging (see the `-p`, `-P`, `-H`, `-l`, and `-r` options); also
it's a nice utility to have around.

[`bat`]: https://crates.io/crates/bat

# Usage

```text
$ kapow -V
kapow 2.9.1
```

```text
$ kapow -h
KAPOW!

Usage: kapow [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Source file(s) [default: -]

Options:
  -p               Page output
  -P               Do not page output
  -H               Disable syntax highlighting
  -L               Display all syntax highlight languages
  -l <LANG>        Syntax higlight language [default: md]
  -w <WRAP>        Wrap !run directive columns [default: 0]
  -c <STRING>      Wrap !run directive continuation [default: \]
  -k               Ignore !run directive failures
  -r, --readme     Print readme
  -h, --help       Print help
  -V, --version    Print version
```

# Errors

Code | Description
---|---
101 | Could not read input file
102 | Could not read included file
103 | Could not change directory
104 | !run directive command failed

NOTE: The kapow process may not *appear* to have exited with these error codes
in "normal usage" because output is piped to [`bat`] as a pager if it is
installed and output is a TTY and unfortunately [`bat`] masks the error code.

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

Please find the [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/kapow/blob/main/CHANGELOG.md
[repository]: https://github.com/qtfkwk/kapow/

# Development

```bash
cargo install cargo-audit cargo-edit cargo-make cargo-outdated dtg \
kapow miniserve
```

