#!/bin/bash

if [ "$#" != "2" ]
then
  echo "Usage: ./aoc.sh <year> <day>"
  echo "Support year: 2021"
  echo "Support day: 1-4"
fi

RUST_MIN_STACK=8388608 cargo test --lib aoc$1_day$2 -- --nocapture
