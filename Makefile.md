# build

* clippy
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `src/**/*.rs`

```
cargo build --release

kapow t/VERSION.t.md >t/VERSION.md
kapow t/USAGE.t.md >t/USAGE.md

kapow {0} >{target}
```

# clippy

```
cargo clippy -- -D clippy::all
```

# test

```
cargo test
```

# check

```
cargo outdated --exit-code 1
cargo audit
```

# update

```
cargo upgrade --incompatible
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall $(toml get -r Cargo.toml package.name)
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo kapow tokei toml-cli
```

# clean

```
cargo clean

rm -f .watch1 .watch2
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# publish

```
cargo publish
git push
git push --tags
```

# full

* update
* check
* build
* install

