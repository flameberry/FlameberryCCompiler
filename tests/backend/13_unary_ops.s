	.globl	_main
_main:
	sub	sp, sp, #144
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	mov	w9, #5
	str	w9, [sp, #0]
	ldr	w9, [sp, #0]
	neg	w9, w9
	str	w9, [sp, #8]
	ldr	w9, [sp, #8]
	str	w9, [sp, #4]
	ldr	w9, [sp, #0]
	mvn	w9, w9
	str	w9, [sp, #16]
	ldr	w9, [sp, #16]
	str	w9, [sp, #12]
	ldr	w9, [sp, #0]
	subs	w9, w9, #0
	cset	w9, eq
	str	w9, [sp, #24]
	ldr	w9, [sp, #24]
	str	w9, [sp, #20]
	mov	w9, #0
	subs	w9, w9, #0
	cset	w9, eq
	str	w9, [sp, #32]
	ldr	w9, [sp, #32]
	str	w9, [sp, #28]
	ldr	w9, [sp, #0]
	neg	w9, w9
	str	w9, [sp, #40]
	ldr	w9, [sp, #40]
	neg	w9, w9
	str	w9, [sp, #44]
	ldr	w9, [sp, #44]
	str	w9, [sp, #36]
	ldr	w9, [sp, #0]
	subs	w9, w9, #0
	cset	w9, eq
	str	w9, [sp, #52]
	ldr	w9, [sp, #52]
	subs	w9, w9, #0
	cset	w9, eq
	str	w9, [sp, #56]
	ldr	w9, [sp, #56]
	str	w9, [sp, #48]
	ldr	w9, [sp, #0]
	str	w9, [sp, #60]
	mov	w9, #0
	str	w9, [sp, #64]
	ldr	w9, [sp, #64]
	subs	w9, w9, #0
	cset	w9, eq
	str	w9, [sp, #68]
	ldr	w9, [sp, #68]
	mov	w10, #0
	subs	w9, w9, w10
	cset	w9, ne
	str	w9, [sp, #72]
	ldr	w9, [sp, #72]
	cbz	w9, .L0
	mov	w9, #9
	neg	w9, w9
	str	w9, [sp, #76]
	ldr	w9, [sp, #76]
	mvn	w9, w9
	str	w9, [sp, #80]
	ldr	w9, [sp, #80]
	str	w9, [sp, #64]
.L0:
	ldr	w9, [sp, #4]
	ldr	w10, [sp, #12]
	add	w9, w9, w10
	str	w9, [sp, #88]
	ldr	w9, [sp, #88]
	ldr	w10, [sp, #20]
	add	w9, w9, w10
	str	w9, [sp, #92]
	ldr	w9, [sp, #92]
	ldr	w10, [sp, #28]
	add	w9, w9, w10
	str	w9, [sp, #96]
	ldr	w9, [sp, #96]
	ldr	w10, [sp, #36]
	add	w9, w9, w10
	str	w9, [sp, #100]
	ldr	w9, [sp, #100]
	ldr	w10, [sp, #48]
	add	w9, w9, w10
	str	w9, [sp, #104]
	ldr	w9, [sp, #104]
	ldr	w10, [sp, #60]
	add	w9, w9, w10
	str	w9, [sp, #108]
	ldr	w9, [sp, #108]
	ldr	w10, [sp, #64]
	add	w9, w9, w10
	str	w9, [sp, #112]
	ldr	w9, [sp, #112]
	str	w9, [sp, #84]
	ldr	w9, [sp, #84]
	mov	w10, #33
	add	w9, w9, w10
	str	w9, [sp, #116]
	ldr	w0, [sp, #116]
	ldp	x29, x30, [sp, #128]
	add	sp, sp, #144
	ret
