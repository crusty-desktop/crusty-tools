# Guard for running just without args. just list recipes
_default:
    just --list

# Run lints and format code
check:
    set -e
    cargo clippy --all
    cargo fmt --all --check
    cargo fmt --all
    # Check files
    fd -e sh -t f -X shellcheck

test:
    cargo nextest run
