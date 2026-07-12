	.globl	_main
_main:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	w9, #24
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #10
	mul	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w9, [sp, #0]
	mov	w10, #5
	sdiv	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	mov	w10, #3
	mul	w9, w9, w10
	str	w9, [sp, #16]
	ldr	w9, [sp, #8]
	ldr	w10, [sp, #16]
	add	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w9, [sp, #20]
	mov	w10, #1
	sub	w9, w9, w10
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	str	w9, [sp, #4]
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
