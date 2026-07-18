	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w9, #3
	mov	w10, #4
	mul	w9, w9, w10
	str	w9, [sp, #0]
	mov	w9, #2
	ldr	w10, [sp, #0]
	add	w9, w9, w10
	str	w9, [sp, #4]
	mov	w9, #10
	mov	w10, #2
	sdiv	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #8]
	sub	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w0, [sp, #12]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
