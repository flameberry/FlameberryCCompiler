# IR and ARM64 Assembly Generation

After semantic analysis, fbcc lowers the AST of each function into a small linear IR
(three-address style), and then turns that IR into ARM64 assembly for macOS
(Apple silicon). This page walks through both stages using `tests/test_ir.c`, which
exercises the whole backend scope: functions with parameters, calls, recursion,
`if`/`else`, `while`, `for`, local variables, reassignment, comparisons and arithmetic.

## The input program

`tests/test_ir.c` (expected process exit code: 59):

```c
// Exercises the full backend scope: functions + params, recursion, if/else,
// while, for, local variables + reassignment, comparisons, and arithmetic
// (+ - * % ). Everything is `int` so it fits the initial codegen scope.
//
// Expected process exit code: 59

int add(int a, int b) {
	return a + b;
}

int factorial(int n) {
	if (n <= 1)
		return 1;
	return n * factorial(n - 1);
}

int sum_to(int n) {
	int total = 0;
	for (int i = 1; i <= n; i = i + 1)
		total = total + i;
	return total;
}

int max(int a, int b) {
	if (a > b)
		return a;
	else
		return b;
}

int main() {
	int a = 6;
	int b = 4;

	int s = add(a, b);	  // 10
	int f = factorial(4); // 24
	int t = sum_to(5);	  // 1+2+3+4+5 = 15
	int m = max(s, t);	  // 15

	int result = 0;
	int i = 0;
	while (i < 3) {
		result = result + m; // 15 * 3 = 45
		i = i + 1;
	}

	result = result + f - s; // 45 + 24 - 10 = 59
	return result % 256;	 // 59
}
```

## The IR

Generated with:

```sh
cargo run -p cli -- --dump-ir tests/test_ir.c
```

Each function becomes a flat list of instructions. `rN` names are stack slots (there
is no register allocation yet), `LN` are jump labels, and `[frame=N]` is the byte size
of the function's stack frame, 16-byte aligned as required by the platform.

```
func add(r0, r1) [frame=32]:
    r2 = r0 + r1
    ret r2

func factorial(r0) [frame=48]:
    r1 = r0 <= 1
    r2 = r1 != 0
    jz r2, L0
    ret 1
L0:
    r3 = r0 - 1
    r4 = call factorial(r3)
    r5 = r0 * r4
    ret r5

func sum_to(r0) [frame=48]:
    r1 = 0
    r2 = 1
L1:
    r3 = r2 <= r0
    r4 = r3 != 0
    jz r4, L2
    r5 = r1 + r2
    r1 = r5
    r6 = r2 + 1
    r2 = r6
    jmp L1
L2:
    ret r1

func max(r0, r1) [frame=32]:
    r2 = r0 > r1
    r3 = r2 != 0
    jz r3, L3
    ret r0
    jmp L4
L3:
    ret r1
L4:

func main() [frame=96]:
    r0 = 6
    r1 = 4
    r3 = call add(r0, r1)
    r2 = r3
    r5 = call factorial(4)
    r4 = r5
    r7 = call sum_to(5)
    r6 = r7
    r9 = call max(r2, r6)
    r8 = r9
    r10 = 0
    r11 = 0
L5:
    r12 = r11 < 3
    r13 = r12 != 0
    jz r13, L6
    r14 = r10 + r8
    r10 = r14
    r15 = r11 + 1
    r11 = r15
    jmp L5
L6:
    r16 = r10 + r4
    r17 = r16 - r2
    r10 = r17
    r18 = r10 % 256
    ret r18
```

A few things worth noticing:

- Comparisons produce a value (`r1 = r0 <= 1`), which is then tested against zero and
  branched on with `jz`. This is how `if`, `while` and `for` are all built.
- Loops are just labels and jumps: the `for` in `sum_to` becomes a test at `L1`,
  a body, and a `jmp L1` back.
- `ret` can appear anywhere in a function, not only at the end — see `factorial`.

## The assembly

Generated with:

```sh
cargo run -p cli -- --dump-asm tests/test_ir.c   # print to stdout
cargo run -p cli -- --emit-asm tests/test_ir.c   # write tests/test_ir.s
```

Every variable and temporary lives in a stack slot; operands pass through the scratch
registers `w9`/`w10`/`w11`. Arguments and return values follow the Apple AAPCS64
calling convention (`w0`–`w7` for arguments, `w0` for the return value), so the output
links against the C runtime with `clang` and runs natively.

```asm
.globl	_add
_add:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	w0, [sp, #0]
	str	w1, [sp, #4]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #4]
	add	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w0, [sp, #8]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.globl	_factorial
_factorial:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	w0, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #1
	subs	w9, w9, w10
	cset	w9, le
	str	w9, [sp, #4]
	ldr	w9, [sp, #4]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	cbz	w9, .L0
	mov	w0, #1
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
.L0:
	ldr	w9, [sp, #0]
	mov	w10, #1
	sub	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w0, [sp, #12]
	bl	_factorial
	str	w0, [sp, #16]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #16]
	mul	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w0, [sp, #20]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.globl	_sum_to
_sum_to:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	w0, [sp, #0]
	mov	w9, #0
	str	w9, [sp, #4]
	mov	w9, #1
	str	w9, [sp, #8]
.L1:
	ldr	w9, [sp, #8]
	ldr	w10, [sp, #0]
	subs	w9, w9, w10
	cset	w9, le
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	cbz	w9, .L2
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #8]
	add	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w9, [sp, #20]
	str	w9, [sp, #4]
	ldr	w9, [sp, #8]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	str	w9, [sp, #8]
	b	.L1
.L2:
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.globl	_max
_max:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	w0, [sp, #0]
	str	w1, [sp, #4]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #4]
	subs	w9, w9, w10
	cset	w9, gt
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	cbz	w9, .L3
	ldr	w0, [sp, #0]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	b	.L4
.L3:
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
.L4:
	.globl	_main
_main:
	sub	sp, sp, #96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	mov	w9, #6
	str	w9, [sp, #0]
	mov	w9, #4
	str	w9, [sp, #4]
	ldr	w0, [sp, #0]
	ldr	w1, [sp, #4]
	bl	_add
	str	w0, [sp, #12]
	ldr	w9, [sp, #12]
	str	w9, [sp, #8]
	mov	w0, #4
	bl	_factorial
	str	w0, [sp, #20]
	ldr	w9, [sp, #20]
	str	w9, [sp, #16]
	mov	w0, #5
	bl	_sum_to
	str	w0, [sp, #28]
	ldr	w9, [sp, #28]
	str	w9, [sp, #24]
	ldr	w0, [sp, #8]
	ldr	w1, [sp, #24]
	bl	_max
	str	w0, [sp, #36]
	ldr	w9, [sp, #36]
	str	w9, [sp, #32]
	mov	w9, #0
	str	w9, [sp, #40]
	mov	w9, #0
	str	w9, [sp, #44]
.L5:
	ldr	w9, [sp, #44]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #48]
	ldr	w9, [sp, #48]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #52]
	ldr	w9, [sp, #52]
	cbz	w9, .L6
	ldr	w9, [sp, #40]
	ldr	w10, [sp, #32]
	add	w9, w9, w10
	str	w9, [sp, #56]
	ldr	w9, [sp, #56]
	str	w9, [sp, #40]
	ldr	w9, [sp, #44]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #60]
	ldr	w9, [sp, #60]
	str	w9, [sp, #44]
	b	.L5
.L6:
	ldr	w9, [sp, #40]
	ldr	w10, [sp, #16]
	add	w9, w9, w10
	str	w9, [sp, #64]
	ldr	w9, [sp, #64]
	ldr	w10, [sp, #8]
	sub	w9, w9, w10
	str	w9, [sp, #68]
	ldr	w9, [sp, #68]
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	mov	w10, #256
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #72]
	ldr	w0, [sp, #72]
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	ret
```

## Running it

The `run.sh` helper does the whole round trip — compile, assemble and link with
`clang`, run the binary and print its exit code:

```sh
$ ./run.sh tests/test_ir.c
...
exit = 59
```

The programs in `tests/backend/` are smaller test cases of the same shape; each one
states its expected exit code in a comment at the top.
