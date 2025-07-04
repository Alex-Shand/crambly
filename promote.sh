#!/usr/bin/env bash

set -eu

find ./tests -name '*.err' -exec bash -c 'mv $1 ${1//.err/.test}' bash {} \;
