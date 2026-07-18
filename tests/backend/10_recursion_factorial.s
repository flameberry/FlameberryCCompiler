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
	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w0, #5
	bl	_factorial
	str	w0, [sp, #0]
	ldr	w0, [sp, #0]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
