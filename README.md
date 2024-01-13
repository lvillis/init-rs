<div align=right>Table of Contents↗️</div>

<h1 align=center><code>init-rs</code></h1>

<p align=center>Use the world's best Cargo to install software, bypassing GitHub.</p>

<div align=center>
  <a href="https://crates.io/crates/init-rs">
    <img src="https://img.shields.io/crates/v/init-rs.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/init-rs">
    <img src="https://img.shields.io/github/repo-size/lvillis/init-rs?style=flat-square&color=328657" alt="crates.io version">
  </a>
  <a href="https://github.com/lvillis/init-rs/actions">
    <img src="https://github.com/lvillis/init-rs/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20init-rs!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>


---

When using GitHub to download open source software in some areas of Blue Planet, it may fail due to poor network
quality.
This project aims to install these software through cargo. Use the rust registry to improve network quality.

## Feature flags

| Name |                        Description                        | Version | Default? |
|:----:|:---------------------------------------------------------:|:-------:|:--------:|
| just |                 🤖 Just a command runner                  | 1.23.0  |    ✔️    |
|  fd  | 📂 A simple, fast and user-friendly alternative to 'find' |  9.0.0  |    ❌     |

## Installation

### install rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install init-rs
```

or

```shell
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```

### set registry

```shell
cat <<EOF | tee -a ~/.cargo/config
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
EOF
```

### install init-rs

```shell
cargo install init-rs
```

or

```shell
cargo install init-rs -f --features=just,fd
```

## Special thanks

[![Jetbrains Logo](https://krwu.github.io/img/jetbrains.svg)](https://www.jetbrains.com/?from=init-rs)

Thanks to [Jetbrains](https://www.jetbrains.com/?from=init-rs) for supporting this small open source project!