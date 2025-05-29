	.globl _main
_main:
	.cfi_startproc
	mov w1, #25
	mov w2, #3
	sdiv w3, w1, w2
	msub w0, w3, w2, w1
	ret
	.cfi_endproc
