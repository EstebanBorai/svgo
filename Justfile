default:
  just --list

# Tests
test:
  cargo test

# Runs formatting tool against Leptos source
web-fmt:
  leptosfmt ./crates/web/src/*.rs

# Runs Web UI for Development
web-dev:
  cd ./crates/web && trunk serve --config ./Trunk.toml

# Builds Web UI for Production
web-build:
  cd ./crates/web && trunk build --release --locked --config ./Trunk.toml
