	.text
	.file	"hexagn"
	.globl	somefunc                        # -- Begin function somefunc
	.p2align	4, 0x90
	.type	somefunc,@function
somefunc:                               # @somefunc
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$69420, %eax                    # imm = 0x10F2C
	retq
.Lfunc_end0:
	.size	somefunc, .Lfunc_end0-somefunc
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$72, %edi
	callq	putchar@PLT
	movl	$101, %edi
	callq	putchar@PLT
	movl	$108, %edi
	callq	putchar@PLT
	movl	$108, %edi
	callq	putchar@PLT
	movl	$111, %edi
	callq	putchar@PLT
	movl	$32, %edi
	callq	putchar@PLT
	movl	$119, %edi
	callq	putchar@PLT
	movl	$111, %edi
	callq	putchar@PLT
	movl	$114, %edi
	callq	putchar@PLT
	movl	$108, %edi
	callq	putchar@PLT
	movl	$100, %edi
	callq	putchar@PLT
	movl	$33, %edi
	callq	putchar@PLT
	movl	$10, %edi
	callq	putchar@PLT
	callq	somefunc@PLT
	movl	$69, 4(%rsp)
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
