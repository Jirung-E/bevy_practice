#!/usr/bin/env bash

set -uo pipefail

seconds_per_example="${SECONDS_PER_EXAMPLE:-3}"
if [[ ! "$seconds_per_example" =~ ^[1-9][0-9]*$ ]]; then
    echo "SECONDS_PER_EXAMPLE must be a positive integer." >&2
    exit 2
fi
workspace="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
log_directory="$workspace/target/smoke-logs"
mkdir -p "$log_directory"

cd "$workspace"

metadata_file="$(mktemp)"
trap 'rm -f "$metadata_file"' EXIT
cargo metadata --format-version 1 --no-deps > "$metadata_file"

binaries=()
while IFS= read -r row; do
    binaries+=("$row")
done < <(
    python3 - "$metadata_file" "$@" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as metadata_file:
    metadata = json.load(metadata_file)

selected = set(sys.argv[2:])
rows = []
for package in metadata["packages"]:
    if selected and package["name"] not in selected:
        continue
    for target in package["targets"]:
        if "bin" in target["kind"]:
            rows.append((package["name"], target["name"]))

for package, binary in sorted(rows):
    print(f"{package}\t{binary}")
PY
)

issues=0
current_package=""
index=0

for row in "${binaries[@]}"; do
    package="${row%%$'\t'*}"
    binary="${row#*$'\t'}"
    index=$((index + 1))

    if [[ "$package" != "$current_package" ]]; then
        current_package="$package"
        echo "Building package $current_package..."
        cargo build --package "$current_package" --bins || exit 1
    fi

    executable="$workspace/target/debug/$binary"
    stdout_log="$log_directory/$binary.stdout.log"
    stderr_log="$log_directory/$binary.stderr.log"

    if [[ ! -x "$executable" ]]; then
        echo "[$index/${#binaries[@]}] MISSING $binary"
        issues=$((issues + 1))
        continue
    fi

    "$executable" >"$stdout_log" 2>"$stderr_log" &
    pid=$!
    deadline=$((SECONDS + seconds_per_example))

    while kill -0 "$pid" 2>/dev/null && (( SECONDS < deadline )); do
        sleep 0.1
    done

    exit_code=0
    if kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
    else
        wait "$pid" || exit_code=$?
    fi

    problem_lines="$(
        grep -Ehi ' (WARN|ERROR) [a-z0-9_:]+:|panicked at|fatal error|stack backtrace|error\[B[0-9]+\]' \
            "$stdout_log" "$stderr_log" || true
    )"

    status="OK"
    if (( exit_code != 0 )); then
        echo "$binary: non-zero exit code $exit_code"
        issues=$((issues + 1))
        status="ISSUE"
    fi
    if [[ -n "$problem_lines" ]]; then
        echo "$problem_lines"
        issues=$((issues + 1))
        status="ISSUE"
    fi

    echo "[$index/${#binaries[@]}] $status $binary"
done

echo
echo "Examples=${#binaries[@]}"
echo "Issues=$issues"

(( issues == 0 ))
