#!/usr/bin/env sh
#
# This script splits a test suite into N roughly
# equally sized test suites using the provided
# output prefix.
#
# Usage:
#     ./ci-split.sh <input.suite> <N> <output-prefix>
#
# The naming of the split files will being in the form generated
# by `split` which is `<prefix><num>.suite` where num is in %02d
# format; for example, a prefix of "benchmarks/split-" will yield
# files with names like benchmarks/split-07.suite.

set -e

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <suite> <number splits> <output prefix>"
    exit 1
fi

SUITE="$1"
NUMBER_SPLITS="$2"
OUTPUT_PREFIX="$3"

ceil_div() {
    echo "($1 + $2 - 1) / $2" | bc
}

count_lines() {
    wc -l "$1" | awk '{print $1}'
}

tmpdir="$(mktemp -d)"
tmp_suite="${tmpdir}/input.suite"
grep -v '^\#' "${SUITE}" > "${tmp_suite}"
line_count=$(count_lines "${tmp_suite}")
lines_per_split=$(ceil_div "$line_count" "$NUMBER_SPLITS")
split -d -l "$lines_per_split" "${tmp_suite}" "${OUTPUT_PREFIX}"
rm -rf "$tmpdir"

echo "Generated Splits:"
for f in "${OUTPUT_PREFIX}"*; do
    # suite files must end with ".suite" to work correctly
    # with sigtglass, so do the rename as split isn't
    # able to do it directly.
    suited="${f}.suite"
    mv "$f" "${suited}"
    echo "- ${suited}"
done
