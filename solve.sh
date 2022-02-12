#!/bin/sh

set -ue

export BASE="$(cd $(dirname $0) && pwd)"
export PATH=$BASE/bin:$PATH

make all
aoc-rust $1 <inputs/$1
