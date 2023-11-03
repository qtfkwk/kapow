# About

![](about/logo.png)

!inc:about/paragraph1.md
It can be used in some different ways:

* Standard: Create a (Markdown) file with kapow directives then run
  `kapow [OPTIONS] path/to/file.ext`; optionally save the output via
  redirection: `kapow [OPTIONS] path/to/file.ext >output.ext`
* Shebang: Place `#!/usr/bin/env kapow` as the first line of a (Markdown) file,
  make it "executable" via `chmod +x file.ext`, then run via `./file.ext` (see
  note #3 under [block directives]).
  Use env's `-S` option if passing options to kapow, for example:
  `#!/usr/bin/env -S kapow -w 60`.
  See [`build.md`] for an example.

While kapow is designed around Markdown syntax, it can be used with any text
format that works with its directives.

[block directives]: #block-directives
[`build.md`]: build.md

