# About

Kapow is a template processor that provides the following directives to support
injecting file contents, command output, current date/time, elapsed time, etc in
generated output.

It can be used in some different ways:

* Standard: Create a (Markdown) file with Kapow directives then run
  `kapow path/to/file.ext`; optionally save the output via redirection:
  `kapow path/to/file.ext >output.ext`
* Shebang: Place `#!/usr/bin/env kapow` as the first line of a (Markdown) file,
  make it "executable" via `chmod +x file.ext`, then run via `./file.ext` (see
  note #3 under [block directives]).

Kapow is designed around Markdown syntax, but can be used with any text format
that works with its directives.

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
   free to embed them inside of or as Markdown syntax... for example, as
   listing contents, prepend a prompt showing the command, etc.

## Span directives

Directive | Example | Description
---|---|---
`` `!elapsed` `` | 0s | Processing time
`` `!now` `` | 2023-05-08T16:51:45Z | Current date/time in UTC / RFC 3339
`` `!now:local` `` | Mon 08 May 2023 12:51:45 EDT | Current date/time in local timezone
`` `!now:local:%A %H:%M` `` | Monday 12:51 | Current date/time in local timezone and custom format
`` `!now:MST7MDT` `` | Mon 08 May 2023 10:51:45 MDT | Current date/time in custom timezone
`` `!now:MST7MDT:%A %H:%M` `` | Monday 10:51 | Current date/time in custom timezone and format
`` `!now:US/Hawaii` `` | Mon 08 May 2023 06:51:45 HST | Current date/time in custom locale
`` `!now:US/Hawaii:%A %H:%M` `` | Monday 06:51 | Current date/time in custom locale and format
`` `!now:UTC:%A %H:%M` `` | Monday 16:51 | Current date/time in UTC and custom format
`` `!now:x` `` | Xh47Gpj | Current date/time in "x" format
`` `!today` `` | 2023-05-08 | Current date in UTC / RFC 3339
`` `!today:local` `` | 2023-05-08 | Current date in local timezone
`` `!today:MST7MDT` `` | 2023-05-08 | Current date in custom timezone
`` `!today:MST7MDT:%v` `` |  8-May-2023 | Current date in custom timezone and format
`` `!today:US/Hawaii` `` | 2023-05-08 | Current date in custom locale
`` `!today:US/Hawaii:%x` `` | 05/08/23 | Current date in custom locale and format
`` `!today:UTC:%A` `` | Monday | Current date in UTC and custom format

* Span directives must be placed inside a code span and may appear zero or more
  times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

# Install

```bash
cargo install kapow bat
```

NOTE: If [`bat`] is installed, Kapow uses it for syntax highlighting and paging
(see the `-p`, `-P`, `-H`, `-l` options).

[`bat`]: https://crates.io/crates/bat

# Usage

```text
$ kapow -V
kapow 2.7.1
```

```text
$ kapow -h
KAPOW!

Usage: kapow [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Source file(s) [default: -]

Options:
  -p             Page output
  -P             Do not page output
  -H             Disable syntax highlighting
  -l <LANG>      Syntax higlight language [default: md]
  -k             Ignore !run directive failures
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
104 | Run directive command failed

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

# Development

```bash
cargo install cargo-edit cargo-make cargo-outdated dtg kapow \
miniserve
```

