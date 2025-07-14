#!/usr/bin/env bash

set -eu

find ./tests/cram -name '*.err' -exec bash -c 'mv $1 ${1//.err/.test}' bash {} \;
