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
	.globl	_triple
_triple:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	w0, [sp, #0]
	ldr	w0, [sp, #0]
	ldr	w1, [sp, #0]
	bl	_add
	str	w0, [sp, #4]
	ldr	w0, [sp, #0]
	ldr	w1, [sp, #4]
	bl	_add
	str	w0, [sp, #8]
	ldr	w0, [sp, #8]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w0, #7
	bl	_triple
	str	w0, [sp, #0]
	ldr	w0, [sp, #0]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
