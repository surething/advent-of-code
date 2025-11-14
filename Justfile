# Justfile for managing Advent of Code activities.
#

# =============================== Main =============================== #

[doc("Choose a command.")]
default:
    @just --explain --choose

# =============================== Ops  =============================== #

[doc("Clean up.")]
clean:
    @echo "🧹 Cleaning up the project"
    cargo clean
    @echo "🧹 Cleaned up the project"

[doc("Build everything.")]
build:
    @echo "🔨 Building the project"
    cargo build --workspace
    @echo "🔨 Built the project"

[doc("Run tests.")]
test:
    @echo "🧪 Running tests"
    cargo test --workspace
    @echo "🧪 Tests completed"
