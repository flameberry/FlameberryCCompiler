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
	.globl	_factorial
_factorial:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	w0, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #1
	subs	w9, w9, w10
	cset	w9, le
	str	w9, [sp, #4]
	ldr	w9, [sp, #4]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	cbz	w9, .L0
	mov	w0, #1
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
.L0:
	ldr	w9, [sp, #0]
	mov	w10, #1
	sub	w9, w9, w10
	str	w9, [sp, #12]
	ldr	w0, [sp, #12]
	bl	_factorial
	str	w0, [sp, #16]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #16]
	mul	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w0, [sp, #20]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.globl	_sum_to
_sum_to:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	w0, [sp, #0]
	mov	w9, #0
	str	w9, [sp, #4]
	mov	w9, #1
	str	w9, [sp, #8]
.L1:
	ldr	w9, [sp, #8]
	ldr	w10, [sp, #0]
	subs	w9, w9, w10
	cset	w9, le
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	cbz	w9, .L2
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #8]
	add	w9, w9, w10
	str	w9, [sp, #20]
	ldr	w9, [sp, #20]
	str	w9, [sp, #4]
.L3:
	ldr	w9, [sp, #8]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	str	w9, [sp, #8]
	b	.L1
.L2:
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.globl	_max
_max:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	w0, [sp, #0]
	str	w1, [sp, #4]
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #4]
	subs	w9, w9, w10
	cset	w9, gt
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	cbz	w9, .L4
	ldr	w0, [sp, #0]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	b	.L5
.L4:
	ldr	w0, [sp, #4]
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
.L5:
	.globl	_main
_main:
	sub	sp, sp, #96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	mov	w9, #6
	str	w9, [sp, #0]
	mov	w9, #4
	str	w9, [sp, #4]
	ldr	w0, [sp, #0]
	ldr	w1, [sp, #4]
	bl	_add
	str	w0, [sp, #12]
	ldr	w9, [sp, #12]
	str	w9, [sp, #8]
	mov	w0, #4
	bl	_factorial
	str	w0, [sp, #20]
	ldr	w9, [sp, #20]
	str	w9, [sp, #16]
	mov	w0, #5
	bl	_sum_to
	str	w0, [sp, #28]
	ldr	w9, [sp, #28]
	str	w9, [sp, #24]
	ldr	w0, [sp, #8]
	ldr	w1, [sp, #24]
	bl	_max
	str	w0, [sp, #36]
	ldr	w9, [sp, #36]
	str	w9, [sp, #32]
	mov	w9, #0
	str	w9, [sp, #40]
	mov	w9, #0
	str	w9, [sp, #44]
.L6:
	ldr	w9, [sp, #44]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #48]
	ldr	w9, [sp, #48]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #52]
	ldr	w9, [sp, #52]
	cbz	w9, .L7
	ldr	w9, [sp, #40]
	ldr	w10, [sp, #32]
	add	w9, w9, w10
	str	w9, [sp, #56]
	ldr	w9, [sp, #56]
	str	w9, [sp, #40]
	ldr	w9, [sp, #44]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #60]
	ldr	w9, [sp, #60]
	str	w9, [sp, #44]
	b	.L6
.L7:
	ldr	w9, [sp, #40]
	ldr	w10, [sp, #16]
	add	w9, w9, w10
	str	w9, [sp, #64]
	ldr	w9, [sp, #64]
	ldr	w10, [sp, #8]
	sub	w9, w9, w10
	str	w9, [sp, #68]
	ldr	w9, [sp, #68]
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	mov	w10, #256
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #72]
	ldr	w0, [sp, #72]
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	ret
