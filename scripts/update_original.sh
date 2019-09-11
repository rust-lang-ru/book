#!/bin/bash

set -eu

SCRIPTS_DIR=`dirname "$0" | xargs realpath`
ROOT_DIR=`realpath "${SCRIPTS_DIR}/../"`
ORIGINAL_DIR="${ROOT_DIR}/rustbook-en"
ORIGINAL_LANG="en"
ORIGINAL_REPO="https://github.com/rust-lang/book.git"

source "${ROOT_DIR}/common-configs/scripts/update_original.sh"
