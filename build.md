#!/usr/bin/env -S kapow
# Update

```text
$ cargo upgrade --incompatible
!run:cargo upgrade --incompatible
```

```text
$ cargo update
!run:cargo update
```

```text
$ cargo outdated --exit-code 1
!run:cargo outdated --exit-code 1 --color=always
```

```text
$ cargo audit
!run:cargo audit --color=always
```

* Requires [`cargo-audit`], [`cargo-outdated`], and [`cargo-upgrade`] crates

[`cargo-audit`]: https://crates.io/crates/cargo-audit
[`cargo-outdated`]: https://crates.io/crates/cargo-outdated
[`cargo-upgrade`]: https://crates.io/crates/cargo-upgrade

# Debug build

```text
$ cargo build
!run:cargo build --color=always
!run:cargo build >/dev/null 2>&1
```

```text
$ cargo clippy
!run:cargo clippy --color=always
```

# Release build

```text
$ cargo build --release
!run:cargo build --release --color=always
!run:cargo run --release -- t/VERSION.t.md >t/VERSION.md
!run:cargo run --release -- t/USAGE.t.md >t/USAGE.md
!run:cargo run --release -- t/README.md >README.md
```

# Install

```text
$ cargo install --path .
!run:cargo install --path .
```

---

`!now` (`!elapsed`)

