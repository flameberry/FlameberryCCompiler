#!/bin/sh
# run_tests.sh — run every tests/backend/*.c through the full pipeline and
# compare the binary's exit code against the `// expect: N` header comment.
#
# Usage: ./run_tests.sh

# Build the compiler once up front; abort if it doesn't build.
cargo build -q -p cli || exit 1
fbcc="./target/debug/cli"

passed=0
failed=0

for src in tests/backend/*.c; do
	name=$(basename "$src")

	# Expected exit code from the `// expect: N` header.
	expected=$(sed -n 's|^// expect: \([0-9][0-9]*\).*|\1|p' "$src" | head -1)
	if [ -z "$expected" ]; then
		echo "FAIL  $name (no '// expect: N' header)"
		failed=$((failed + 1))
		continue
	fi

	asm="/tmp/fbcc-test.s"
	bin="/tmp/fbcc-test-bin"

	if ! "$fbcc" --emit-asm -o "$asm" "$src" >/dev/null 2>&1; then
		echo "FAIL  $name (compile error)"
		failed=$((failed + 1))
		continue
	fi

	if ! clang "$asm" -o "$bin" 2>/dev/null; then
		echo "FAIL  $name (assemble/link error)"
		failed=$((failed + 1))
		continue
	fi

	"$bin"
	actual=$?

	if [ "$actual" -eq "$expected" ]; then
		echo "PASS  $name (exit = $actual)"
		passed=$((passed + 1))
	else
		echo "FAIL  $name (expected $expected, got $actual)"
		failed=$((failed + 1))
	fi
done

rm -f /tmp/fbcc-test.s /tmp/fbcc-test-bin

echo "----------------"
echo "$passed passed, $failed failed"
[ "$failed" -eq 0 ]
