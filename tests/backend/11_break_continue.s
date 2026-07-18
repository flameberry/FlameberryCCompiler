	.globl	_main
_main:
	sub	sp, sp, #160
	stp	x29, x30, [sp, #144]
	add	x29, sp, #144
	mov	w9, #0
	str	w9, [sp, #0]
.L0:
	mov	w9, #1
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #4]
	ldr	w9, [sp, #4]
	cbz	w9, .L1
	ldr	w9, [sp, #0]
	mov	w10, #5
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #12]
	ldr	w9, [sp, #12]
	cbz	w9, .L2
	b	.L1
.L2:
	ldr	w9, [sp, #0]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	str	w9, [sp, #0]
	b	.L0
.L1:
	mov	w9, #0
	str	w9, [sp, #20]
	mov	w9, #0
	str	w9, [sp, #24]
.L3:
	ldr	w9, [sp, #24]
	mov	w10, #10
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #28]
	ldr	w9, [sp, #28]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #32]
	ldr	w9, [sp, #32]
	cbz	w9, .L4
	ldr	w9, [sp, #24]
	mov	w10, #2
	sdiv	w11, w9, w10
	msub	w9, w11, w10, w9
	str	w9, [sp, #36]
	ldr	w9, [sp, #36]
	mov	w10, #1
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #44]
	ldr	w9, [sp, #44]
	cbz	w9, .L6
	b	.L5
.L6:
	ldr	w9, [sp, #20]
	ldr	w10, [sp, #24]
	add	w9, w9, w10
	str	w9, [sp, #48]
	ldr	w9, [sp, #48]
	str	w9, [sp, #20]
.L5:
	ldr	w9, [sp, #24]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #52]
	ldr	w9, [sp, #52]
	str	w9, [sp, #24]
	b	.L3
.L4:
	mov	w9, #0
	str	w9, [sp, #56]
	mov	w9, #0
	str	w9, [sp, #60]
.L7:
	ldr	w9, [sp, #60]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #64]
	ldr	w9, [sp, #64]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #68]
	ldr	w9, [sp, #68]
	cbz	w9, .L8
.L10:
	ldr	w9, [sp, #72]
	mov	w10, #10
	subs	w9, w9, w10
	cset	w9, lt
	str	w9, [sp, #76]
	ldr	w9, [sp, #76]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #80]
	ldr	w9, [sp, #80]
	cbz	w9, .L11
	ldr	w9, [sp, #72]
	mov	w10, #2
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #84]
	ldr	w9, [sp, #84]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #88]
	ldr	w9, [sp, #88]
	cbz	w9, .L12
	b	.L11
.L12:
	ldr	w9, [sp, #72]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #92]
	ldr	w9, [sp, #92]
	str	w9, [sp, #72]
	b	.L10
.L11:
	ldr	w9, [sp, #60]
	mov	w10, #1
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #96]
	ldr	w9, [sp, #96]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #100]
	ldr	w9, [sp, #100]
	cbz	w9, .L13
	b	.L9
.L13:
	ldr	w9, [sp, #56]
	ldr	w10, [sp, #72]
	add	w9, w9, w10
	str	w9, [sp, #104]
	ldr	w9, [sp, #104]
	str	w9, [sp, #56]
.L9:
	ldr	w9, [sp, #60]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #108]
	ldr	w9, [sp, #108]
	str	w9, [sp, #60]
	b	.L7
.L8:
	mov	w9, #0
	str	w9, [sp, #112]
.L14:
	ldr	w9, [sp, #112]
	mov	w10, #1
	add	w9, w9, w10
	str	w9, [sp, #116]
	ldr	w9, [sp, #116]
	str	w9, [sp, #112]
	ldr	w9, [sp, #112]
	mov	w10, #3
	subs	w9, w9, w10
	cset	w9, eq
	str	w9, [sp, #120]
	ldr	w9, [sp, #120]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #124]
	ldr	w9, [sp, #124]
	cbz	w9, .L16
	b	.L15
.L16:
	b	.L14
.L15:
	ldr	w9, [sp, #0]
	ldr	w10, [sp, #20]
	add	w9, w9, w10
	str	w9, [sp, #128]
	ldr	w9, [sp, #128]
	ldr	w10, [sp, #56]
	add	w9, w9, w10
	str	w9, [sp, #132]
	ldr	w9, [sp, #132]
	ldr	w10, [sp, #112]
	add	w9, w9, w10
	str	w9, [sp, #136]
	ldr	w0, [sp, #136]
	ldp	x29, x30, [sp, #144]
	add	sp, sp, #160
	ret
