	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w9, #10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #5
	add	w9, w9, w10
	str	w9, [sp, #4]
	ldr	w9, [sp, #4]
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #3
	sub	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	str	w9, [sp, #0]
	ldr	w0, [sp, #0]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
