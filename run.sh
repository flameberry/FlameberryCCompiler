#!/bin/sh
# run.sh <file.c> — compile to ARM64 asm, assemble+link with clang, run, show exit code.
#
# Usage: ./run.sh tests/test_ir_simple.c

src="$1"
if [ -z "$src" ]; then
	echo "usage: ./run.sh <file.c>" >&2
	exit 2
fi

asm="${src%.c}.s"
bin="/tmp/fbcc-bin"

cargo run -q -p cli -- --emit-asm -o "$asm" "$src" || exit 1

echo "----- $asm -----"
cat "$asm"
echo "----------------"

clang "$asm" -o "$bin" || exit 1

"$bin"
echo "exit = $?"
