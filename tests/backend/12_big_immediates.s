	.globl	_main
_main:
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	w9, #4464
	movk	w9, #1, lsl #16
	str	w9, [sp, #0]
	mov	w9, #16960
	movk	w9, #15, lsl #16
	str	w9, [sp, #4]
	mov	w9, #65535
	movk	w9, #32767, lsl #16
	str	w9, [sp, #8]
	ldr	w9, [sp, #0]
	mov	w10, #256
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	str	w9, [sp, #12]
	ldr	w9, [sp, #4]
	mov	w10, #0
	movk	w10, #1, lsl #16
	sdiv	w9, w9, w10
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	str	w9, [sp, #20]
	ldr	w9, [sp, #8]
	mov	w10, #100
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #32]
	ldr	w9, [sp, #32]
	str	w9, [sp, #28]
	ldr	w9, [sp, #12]
	ldr	w10, [sp, #20]
	add	w9, w9, w10
	str	w9, [sp, #36]
	ldr	w9, [sp, #36]
	ldr	w10, [sp, #28]
	add	w9, w9, w10
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	mov	w10, #97
	sub	w9, w9, w10
	str	w9, [sp, #44]
	ldr	w0, [sp, #44]
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	ret
