



`RDTSC` & `RDTSCP`


`mach_absolute_time()`

```asm
0000000000001271        pushq   %rbp
0000000000001272        movq    %rsp, %rbp
0000000000001275        movabsq $0x7fffffe00050, %rsi           ## imm = 0x7FFFFFE00050
000000000000127f        movl    0x18(%rsi), %r8d
0000000000001283        testl   %r8d, %r8d
0000000000001286        je      0x127f
0000000000001288        lfence
000000000000128b        rdtsc
000000000000128d        lfence
0000000000001290        shlq    $0x20, %rdx
0000000000001294        orq     %rdx, %rax
0000000000001297        movl    0xc(%rsi), %ecx
000000000000129a        andl    $0x1f, %ecx
000000000000129d        subq    (%rsi), %rax
00000000000012a0        shlq    %cl, %rax
00000000000012a3        movl    0x8(%rsi), %ecx
00000000000012a6        mulq    %rcx
00000000000012a9        shrdq   $0x20, %rdx, %rax
00000000000012ae        addq    0x10(%rsi), %rax
00000000000012b2        cmpl    0x18(%rsi), %r8d
00000000000012b6        jne     0x127f
00000000000012b8        popq    %rbp
00000000000012b9        retq
00000000000012ba        addb    %al, (%rax)
```

