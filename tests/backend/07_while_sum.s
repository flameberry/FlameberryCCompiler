	.globl	_main
_main:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	w9, #1
	str	w9, [sp, #0]
	mov	w9, #0
	str	w9, [sp, #4]
.L0:
	ldr	w9, [sp, #0]
	mov	w10, #10
	subs	w9, w9, w10
	cset	w9, le
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	cbz	w9, .L1
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #0]
	add	w9, w9, w10
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w9, [sp, #20]
	str	w9, [sp, #0]
	b	.L0
.L1:
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
