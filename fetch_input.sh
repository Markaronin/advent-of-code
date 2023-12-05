#!/bin/bash
set -eux

readonly YEAR="$1"
readonly DAY="$2"

curl "https://adventofcode.com/${YEAR}/day/${DAY}/input" \
    --fail \
    -H "Cookie: session=${SESSION_COOKIE}"\
    > "ac_${YEAR}_${DAY}/input"