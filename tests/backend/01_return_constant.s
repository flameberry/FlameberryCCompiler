	.globl	_main
_main:
	sub	sp, sp, #16
	stp	x29, x30, [sp, #0]
	add	x29, sp, #0
	mov	w0, #42
	ldp	x29, x30, [sp, #0]
	add	sp, sp, #16
	ret
