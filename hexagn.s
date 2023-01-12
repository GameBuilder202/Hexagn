.global main
_Hx4mainint8:
    sd s0, (sp)
    add s0, sp, zero
.L0:
    li s10, 5
    li s8, 1
    li s9, 1
    add s11, s10, zero
    mul s10, s9, s11
    add s11, s8, s10
    add sp, s0, zero
    ret
