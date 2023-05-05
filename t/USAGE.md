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

