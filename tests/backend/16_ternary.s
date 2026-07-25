	.globl	_main
_main:
	sub	sp, sp, #128
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	mov	w9, #5
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, gt
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	cbz	w9, .L0
	mov	w9, #10
	str	w9, [sp, #16]
	b	.L1
.L0:
	mov	w9, #20
	str	w9, [sp, #16]
.L1:
	ldr	w9, [sp, #16]
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, gt
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #28]
	ldr	w9, [sp, #28]
	cbz	w9, .L2
	ldr	w9, [sp, #0]
	mov	w10, #2
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #36]
	ldr	w9, [sp, #36]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	cbz	w9, .L3
	mov	w9, #1
	str	w9, [sp, #44]
	b	.L4
.L3:
	mov	w9, #2
	str	w9, [sp, #44]
.L4:
	ldr	w9, [sp, #44]
	str	w9, [sp, #32]
	b	.L5
.L2:
	mov	w9, #3
	str	w9, [sp, #32]
.L5:
	ldr	w9, [sp, #32]
	str	w9, [sp, #20]
	ldr	w9, [sp, #0]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #52]
	ldr	w9, [sp, #52]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #56]
	ldr	w9, [sp, #56]
	cbz	w9, .L6
	mov	w9, #100
	str	w9, [sp, #60]
	b	.L7
.L6:
	mov	w9, #7
	str	w9, [sp, #60]
.L7:
	ldr	w9, [sp, #60]
	str	w9, [sp, #48]
	mov	w9, #0
	str	w9, [sp, #64]
	mov	w9, #0
	str	w9, [sp, #68]
	ldr	w9, [sp, #0]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, gt
	str	w9, [sp, #72]
	ldr	w9, [sp, #72]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #76]
	ldr	w9, [sp, #76]
	cbz	w9, .L8
	mov	w9, #4
	str	w9, [sp, #64]
	ldr	w9, [sp, #64]
	str	w9, [sp, #80]
	b	.L9
.L8:
	mov	w9, #9
	str	w9, [sp, #68]
	ldr	w9, [sp, #68]
	str	w9, [sp, #80]
.L9:
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #20]
	add	w9, w9, w10
	str	w9, [sp, #84]
	ldr	w9, [sp, #84]
	ldr	w10, [sp, #48]
	add	w9, w9, w10
	str	w9, [sp, #88]
	ldr	w9, [sp, #88]
	ldr	w10, [sp, #64]
	add	w9, w9, w10
	str	w9, [sp, #92]
	ldr	w9, [sp, #92]
	ldr	w10, [sp, #68]
	add	w9, w9, w10
	str	w9, [sp, #96]
	ldr	w0, [sp, #96]
	ldp	x29, x30, [sp, #112]
	add	sp, sp, #128
	ret
