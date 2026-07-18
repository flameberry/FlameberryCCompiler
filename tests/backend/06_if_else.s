	.globl	_main
_main:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	w9, #5
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #4]
	ldr	w9, [sp, #4]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	cbz	w9, .L0
	mov	w0, #100
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	b	.L1
.L0:
	ldr	w9, [sp, #0]
	mov	w10, #5
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	cbz	w9, .L2
	mov	w0, #55
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	b	.L3
.L2:
	mov	w0, #0
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
.L3:
.L1:
