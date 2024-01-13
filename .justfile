default:
  @just --version
  @just --list

publish:
  cargo build
  cargo .git/publish --registry crates-io
