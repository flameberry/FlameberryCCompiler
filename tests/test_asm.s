	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w9, #24
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	str	w9, [sp, #4]
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
