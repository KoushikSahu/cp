#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<EOF
Usage: $(basename "$0") -s <bin> -b <bin> -g <bin>

Stress-test a solution against a brute-force using a random input generator.
Runs indefinitely until a mismatch is found, then prints the differing outputs.

All three binaries must be cargo bin targets inside this workspace.
The '.rs' extension is not required (and should not be included).

Options:
  -s, --solution   <bin>   Cargo bin name for the solution
  -b, --brute      <bin>   Cargo bin name for the brute force
  -g, --generator  <bin>   Cargo bin name for the generator
  -h, --help               Show this help text and exit

Example:
  $(basename "$0") -s 2185E -b bruteforce -g generator
EOF
}

SOLUTION=""
BRUTE=""
GENERATOR=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        -s|--solution)
            SOLUTION="$2"; shift 2 ;;
        -b|--brute)
            BRUTE="$2"; shift 2 ;;
        -g|--generator)
            GENERATOR="$2"; shift 2 ;;
        -h|--help)
            usage; exit 0 ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit 1 ;;
    esac
done

if [[ -z "$SOLUTION" || -z "$BRUTE" || -z "$GENERATOR" ]]; then
    echo "Error: -s, -b, and -g are all required." >&2
    usage >&2
    exit 1
fi

WORKSPACE_MANIFEST="$(cargo locate-project --workspace --message-format plain 2>/dev/null)" || {
    echo "Error: run this from inside a Cargo workspace." >&2
    exit 1
}

WORKSPACE_ROOT="$(dirname "$WORKSPACE_MANIFEST")"
TARGET_DIR="${CARGO_TARGET_DIR:-$WORKSPACE_ROOT/target}"

echo "Building binaries: $GENERATOR, $BRUTE, $SOLUTION ..."
cargo build --release --manifest-path "$WORKSPACE_MANIFEST" --bin "$GENERATOR" --bin "$BRUTE" --bin "$SOLUTION"

GEN_BIN="$TARGET_DIR/release/$GENERATOR"
BRUTE_BIN="$TARGET_DIR/release/$BRUTE"
SOL_BIN="$TARGET_DIR/release/$SOLUTION"

TMPDIR_CUSTOM=$(mktemp -d)
trap 'rm -rf "$TMPDIR_CUSTOM"' EXIT

INPUT="$TMPDIR_CUSTOM/input.txt"
OUT_BRUTE="$TMPDIR_CUSTOM/brute.txt"
OUT_SOL="$TMPDIR_CUSTOM/sol.txt"

echo "Running indefinitely until a mismatch is found (Ctrl+C to stop)..."

i=0
while true; do
    i=$((i + 1))
    "$GEN_BIN"   > "$INPUT"
    "$BRUTE_BIN" < "$INPUT" > "$OUT_BRUTE"
    "$SOL_BIN"   < "$INPUT" > "$OUT_SOL"

    if ! diff -q "$OUT_BRUTE" "$OUT_SOL" > /dev/null 2>&1; then
        echo ""
        echo "MISMATCH on test $i"
        echo "--- Input ---"
        cat "$INPUT"
        echo "--- Brute force output ($BRUTE) ---"
        cat "$OUT_BRUTE"
        echo "--- Solution output ($SOLUTION) ---"
        cat "$OUT_SOL"
        echo "--- Diff (brute vs solution) ---"
        diff "$OUT_BRUTE" "$OUT_SOL" || true
        exit 1
    fi

    printf "\rTests passed: %d" "$i"
done
