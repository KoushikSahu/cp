#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<EOF
Usage: $(basename "$0") -s <file> -b <file> -g <file>

Stress-test a C++ solution against a brute-force using a random input generator.
Runs indefinitely until a mismatch is found, then prints the differing outputs.

The script looks for files in the current directory.

The file extension is optional. If omitted, the script tries .cpp, .cc, and .cxx.

Options:
  -s, --solution   <file>  Solution source file
  -b, --brute      <file>  Brute-force source file
  -g, --generator  <file>  Random input generator source file
  -h, --help               Show this help text and exit

Environment:
  CXX                      C++ compiler to use (default: g++)
  CXXFLAGS                 Compiler flags (default: -std=c++20 -O2 -pipe)

Example:
  $(basename "$0") -s 2185E -b bruteforce -g generator
  $(basename "$0") -s sol.cpp -b brute.cpp -g gen.cpp
EOF
}

require_value() {
    if [[ $# -lt 2 ]]; then
        echo "Error: missing value for $1" >&2
        usage >&2
        exit 1
    fi
}

resolve_source() {
    local value="$1"
    local candidate=""

    if [[ "$value" = /* ]]; then
        candidate="$value"
    else
        candidate="$CURRENT_DIR/$value"
    fi

    if [[ -f "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
    fi

    if [[ "$value" != *.* ]]; then
        for ext in cpp cc cxx; do
            candidate="$CURRENT_DIR/$value.$ext"
            if [[ -f "$candidate" ]]; then
                printf '%s\n' "$candidate"
                return 0
            fi
        done
    fi

    echo "Error: could not find source file for '$value' in '$CURRENT_DIR'." >&2
    exit 1
}

compile_source() {
    local src="$1"
    local out="$2"
    echo "Compiling $(basename "$src") ..."
    "$CXX" "${CXXFLAGS_ARR[@]}" "$src" -o "$out"
}

SOLUTION=""
BRUTE=""
GENERATOR=""
CURRENT_DIR="$(pwd -P)"
CXX="${CXX:-g++}"

if [[ -n "${CXXFLAGS:-}" ]]; then
    read -r -a CXXFLAGS_ARR <<< "$CXXFLAGS"
else
    CXXFLAGS_ARR=(-std=c++20 -O2 -pipe)
fi

while [[ $# -gt 0 ]]; do
    case "$1" in
        -s|--solution)
            require_value "$@"
            SOLUTION="$2"
            shift 2
            ;;
        -b|--brute)
            require_value "$@"
            BRUTE="$2"
            shift 2
            ;;
        -g|--generator)
            require_value "$@"
            GENERATOR="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [[ -z "$SOLUTION" || -z "$BRUTE" || -z "$GENERATOR" ]]; then
    echo "Error: -s, -b, and -g are all required." >&2
    usage >&2
    exit 1
fi

if ! command -v "$CXX" >/dev/null 2>&1; then
    echo "Error: compiler '$CXX' was not found in PATH." >&2
    exit 1
fi

SOLUTION_SRC="$(resolve_source "$SOLUTION")"
BRUTE_SRC="$(resolve_source "$BRUTE")"
GENERATOR_SRC="$(resolve_source "$GENERATOR")"

TMPDIR_CUSTOM=$(mktemp -d)
trap 'rm -rf "$TMPDIR_CUSTOM"' EXIT

GEN_BIN="$TMPDIR_CUSTOM/generator"
BRUTE_BIN="$TMPDIR_CUSTOM/brute"
SOL_BIN="$TMPDIR_CUSTOM/solution"

INPUT="$TMPDIR_CUSTOM/input.txt"
OUT_BRUTE="$TMPDIR_CUSTOM/brute.txt"
OUT_SOL="$TMPDIR_CUSTOM/sol.txt"

echo "Building binaries in $CURRENT_DIR ..."
compile_source "$GENERATOR_SRC" "$GEN_BIN"
compile_source "$BRUTE_SRC" "$BRUTE_BIN"
compile_source "$SOLUTION_SRC" "$SOL_BIN"

echo "Running indefinitely until a mismatch is found (Ctrl+C to stop)..."

i=0
while true; do
    i=$((i + 1))

    if ! "$GEN_BIN" > "$INPUT"; then
        echo ""
        echo "Generator failed on test $i" >&2
        exit 1
    fi

    if ! "$BRUTE_BIN" < "$INPUT" > "$OUT_BRUTE"; then
        echo ""
        echo "Brute force failed on test $i" >&2
        echo "--- Input ---" >&2
        cat "$INPUT" >&2
        exit 1
    fi

    if ! "$SOL_BIN" < "$INPUT" > "$OUT_SOL"; then
        echo ""
        echo "Solution failed on test $i" >&2
        echo "--- Input ---" >&2
        cat "$INPUT" >&2
        exit 1
    fi

    if ! diff -q "$OUT_BRUTE" "$OUT_SOL" > /dev/null 2>&1; then
        echo ""
        echo "MISMATCH on test $i"
        echo "--- Input ---"
        cat "$INPUT"
        echo "--- Brute force output ($(basename "$BRUTE_SRC")) ---"
        cat "$OUT_BRUTE"
        echo "--- Solution output ($(basename "$SOLUTION_SRC")) ---"
        cat "$OUT_SOL"
        echo "--- Diff (brute vs solution) ---"
        diff "$OUT_BRUTE" "$OUT_SOL" || true
        exit 1
    fi

    printf "\rTests passed: %d" "$i"
done
