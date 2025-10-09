## Time stamp counter

### `RDTSC` & `RDTSCP`

## ARM

Similarly on ARM there

## MacOS

For a long time `machTimestamp` has been in nanoseconds, so the Mach clock ticks occured once every nanosecond.
However, with the new Apple Silicon Macs, this is not the case any more.
The hardware clocks, Mach precision time, are affected by this change, but not local reference clocks.

On intel processors, that tick has been one nanosecond long, so working out a time interval has been all too easy.

What we should be doing is apply a conversion factor to the difference in clock ticks, to convert from ticks
to nanoseconds.

For MacOS the numerator and denominator of `mach_timebase_info` are the same, and have been omitted.
The correction may be large and different from model to model.

https://developer.apple.com/documentation/driverkit/mach_timebase_info_t
https://developer.apple.com/library/archive/qa/qa1398/_index.html
https://developer.apple.com/documentation/apple-silicon/addressing-architectural-differences-in-your-macos-code
https://eclecticlight.co/2020/09/08/changing-the-clock-in-apple-silicon-macs/
https://eclecticlight.co/2017/02/23/so-many-times-the-clocks-in-your-mac/

### `mach_absolute_time`

`x86_64`

```shell
otool -tv /usr/lib/system/libsystem_kernel.dylib | grep "_mach_absolute_time:" -A 24
```

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

`AARCH64`

```shell
otool -tv /usr/lib/system/libsystem_kernel.dylib | grep "_mach_absolute_time:" -A 33
```

```asm
0000000000001378        movk    x3, #0x0, lsl #48
000000000000137c        movk    x3, #0xf, lsl #32
0000000000001380        movk    x3, #0xffff, lsl #16
0000000000001384        movk    x3, #0xc088
0000000000001388        ldrb    w2, [x3, #0x8]
000000000000138c        cmp     x2, #0x0
0000000000001390        b.eq    _mach_absolute_time_kernel
0000000000001394        cmp     x2, #0x2
0000000000001398        b.eq    0x13c4
000000000000139c        cmp     x2, #0x3
00000000000013a0        b.eq    0x13e0
00000000000013a4        isb
00000000000013a8        ldr     x1, [x3]
00000000000013ac        mrs     x0, CNTVCT_EL0
00000000000013b0        ldr     x2, [x3]
00000000000013b4        cmp     x1, x2
00000000000013b8        b.ne    0x13a8
00000000000013bc        add     x0, x0, x1
00000000000013c0        ret
00000000000013c4        ldr     x1, [x3]
00000000000013c8        mrs     x0, CNTVCTSS_EL0
00000000000013cc        ldr     x2, [x3]
00000000000013d0        cmp     x1, x2
00000000000013d4        b.ne    0x13c4
00000000000013d8        add     x0, x0, x1
00000000000013dc        ret
00000000000013e0        ldr     x1, [x3]
00000000000013e4        mrs     x0, S3_4_C15_C10_6
00000000000013e8        ldr     x2, [x3]
00000000000013ec        cmp     x1, x2
00000000000013f0        b.ne    0x13e0
00000000000013f4        add     x0, x0, x1
00000000000013f8        ret
```
