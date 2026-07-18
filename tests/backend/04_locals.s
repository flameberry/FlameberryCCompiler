	.globl	_main
_main:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	w9, #5
	str	w9, [sp, #0]
	mov	w9, #7
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #4]
	mul	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #5
	sub	w9, w9, w10
	str	w9, [sp, #16]
	ldr	w0, [sp, #16]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
