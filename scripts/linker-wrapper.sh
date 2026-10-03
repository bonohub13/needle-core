#!/usr/bin/env sh

TARGET_LINKER="$1"
DEFAULT_LINKER="/usr/bin/ld"

shift # use arguments after $1
if command -v ${TARGET_LINKER} > /dev/null
then
    ${TARGET_LINKER} $@
else
    ${DEFAULT_LINKER} $@
fi
