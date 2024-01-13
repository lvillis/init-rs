default:
  @just --version
  @just --list

publish:
  cargo build
  cargo publish --registry crates-io
