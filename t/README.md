!inc:ABOUT.md
## Block directives

Directive | Description
---|---
`!inc:path` | Include file contents; path is relative to its containing file
`!run:command` | Run command and insert stdout
`!start:name` - `!stop:name`| Optional content included only if `name` is provided in `-f` value

!start:comment
THIS IS A COMMENT AND DOES NOT APPEAR IN THE OUTPUT UNLESS WE RUN:
`kapow -f comment ...`

!stop:comment
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
`` `\!elapsed` `` | `!elapsed` | Processing time
`` `\!now` `` | `!now` | Current date/time in UTC / RFC 3339
`` `\!now:local` `` | `!now:local` | Current date/time in local timezone
`` `\!now:local:%A %H:%M` `` | `!now:local:%A %H:%M` | Current date/time in \
local timezone and custom format
`` `\!now:MST7MDT` `` | `!now:MST7MDT` | Current date/time in custom timezone
`` `\!now:MST7MDT:%A %H:%M` `` | `!now:MST7MDT:%A %H:%M` | Current date/time \
in custom timezone and format
`` `\!now:US/Hawaii` `` | `!now:US/Hawaii` | Current date/time in custom locale
`` `\!now:US/Hawaii:%A %H:%M` `` | `!now:US/Hawaii:%A %H:%M` | Current \
date/time in custom locale and format
`` `\!now:UTC:%A %H:%M` `` | `!now:UTC:%A %H:%M` | Current date/time in UTC \
and custom format
`` `\!now:x` `` | `!now:x` | Current date/time in "x" format
`` `\!today` `` | `!today` | Current date in UTC / RFC 3339
`` `\!today:local` `` | `!today:local` | Current date in local timezone
`` `\!today:MST7MDT` `` | `!today:MST7MDT` | Current date in custom timezone
`` `\!today:MST7MDT:%v` `` | `!today:MST7MDT:%v` | Current date in custom \
timezone and format
`` `\!today:US/Hawaii` `` | `!today:US/Hawaii` | Current date in custom locale
`` `\!today:US/Hawaii:%x` `` | `!today:US/Hawaii:%x` | Current date in custom \
locale and format
`` `\!today:UTC:%A` `` | `!today:UTC:%A` | Current date in UTC and custom format

* Span directives must be placed inside a code span and may appear zero or more
  times in any line.
* Disable processing a span directive by escaping `!` with a backslash: `\!`.

## Other features

* Escaped wrap: End a line with a backslask `\` and it will be *unwrapped* in
  the output; this enables an author to wrap long lines in the source but have
  them be unwrapped by kapow.
  If a backslash needs to be maintained, just use two backslashes `\\`.

# Usage

```text
$ kapow -h
!run:../target/release/kapow -h
```

```text
$ kapow -V
!run:../target/release/kapow -V
```

# Install

```bash
cargo install kapow bat
```

NOTE: Installing [`bat`] is optional, but if installed, kapow uses it for syntax
highlighting and paging (see the `-p`, `-P`, `-H`, `-l`, and `-r` options); also
it's a nice utility to have around.

[`bat`]: https://crates.io/crates/bat

# Usage

!inc:VERSION.md
!inc:USAGE.md
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
    * `` `\!now` `` (all variants)

# Changelog

Please find the [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/kapow/blob/main/CHANGELOG.md
[repository]: https://github.com/qtfkwk/kapow/

# Development

```bash
cargo install b3sum cargo-audit cargo-edit cargo-make \\
cargo-outdated dtg kapow miniserve
```

