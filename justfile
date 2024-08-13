set dotenv-load := true

alias v := verify
alias r := run

bt := '0'
log := "warn"

@_list:
    just --list --unsorted

run *args:
    cargo run -q -- {{args}}

install:
    cargo install --path .

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------


create_docs: install
    hy markdown > ./docs/src/chapter_1.md

dist:
    cargo dist init
    cargo dist generate

# Run the tests
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt

check_deps:
    cargo upgrades
    
install-tools:
    cargo binstall cargo-dist
    cargo install -f cargo-upgrades
    cargo install --locked cargo-outdated