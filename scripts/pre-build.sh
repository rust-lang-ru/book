#!/bin/bash

set -eu

SCRIPTS_DIR="`dirname ${0}`"
ROOT="`realpath ${SCRIPTS_DIR}/../`"
pushd $ROOT
mv rustbook-ru book-ru
popd
#
