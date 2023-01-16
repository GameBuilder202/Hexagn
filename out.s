.intel_syntax
.extern putchar
.global main
main:
    push %rbp
    mov %rbp, %rsp
    push %rbx
    push %r12
    push %r13
    push %r14
    push %r15
.main.L0:
    pop %r15
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    mov %rsp, %rbp
    pop %rbp
    ret

