	.globl	_main
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w9, #17
	mov	w10, #5
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #0]
	mov	w9, #23
	mov	w10, #4
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #4]
	add	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w0, [sp, #8]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
