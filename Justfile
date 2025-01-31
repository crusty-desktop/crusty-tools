# Guard for running just without args. just list recipes
_default:
    just --list

# Run clippy and format code
check: _shellcheck _format
    cargo clippy --all

# Check shell scripts
_shellcheck:
    fd -e sh -t f -X shellcheck

# Format code
_format:
    cargo fmt --all --check
    cargo fmt --all

test:
    cargo nextest run

