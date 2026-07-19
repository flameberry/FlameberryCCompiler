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
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #3
	sub	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #4
	mul	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #2
	sdiv	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #17
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #2
	lsl	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #1
	asr	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #30
	and	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #3
	orr	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #5
	eor	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	mov	w10, #2
	mul	w9, w9, w10
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	ldr	w10, [sp, #4]
	add	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w0, [sp, #12]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
