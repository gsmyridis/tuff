# Tuff

High resolution profiling made easy.
This crate provides ultra-low-overhead primitives for measuring elapsed time and CPU cycles across operating systems and architectures.
It also includes a convenient high-level profiler with reporting features.
The goal is to easily, and accurately profile subsets of your program and get useful information to improve performance.

**NOTE:** The crate is unstable depending on the features that are you use. Not all instructions are supported in all CPUs.

# Usage

```rust
    tuff::profile_block! { ["label", 0]
        // Code
    }

    tuff::profile_block! { ["label"]
        // Code
    }

    #[tuff::profile_function]
    fn some_funtion() {
        // Code
    }

    #[tuff::profile_function(0)]
    fn some_funtion() {
        // Code
    }
```

The difference between the two macros is that one takes only a label, while the other one also includes a number, the index to the `ProfileAnchor` node, which collects the information. The former one is easier to use, avoids any "collisions", but adds some overhead, since it searches for the index based on the call-site (file, line and column).
So if you use the both variants, there is a chance that the results will not make much sense.
A good practice would be to choose indexes with high numbers maybe start at 50, or 100, depending on the callsite numbers you have.

# Feature Flags

- coarse
- os
- cpu-counter
- cpu-counter-serialized
- m-experimental

Conceptually what we want to do is simple. But, because of time evolution, variance between hardware vendors, or operating systems, it becomes more complicated.

## Notes on profiling

It is crucial to note that the act of measuring performance affects the perfomance of the system, almost always detiorating it.

# Time measurement

When profiling a program we always want to measure how long a program, or subset of it, is running.
But there are different measures of code execution time, and different sources to retrieve these measures with different resolutions.
For example, we could use a measure of wall-clock time (normal, every day time), or use number of CPU cycles as a measure.
Ideally, we would use number of cycles as it is more representative of the code performance, but wall-clock time is also useful because we can intuit it better, and ultimately it is what a usual end user is interested in minimizing.

## CPU Counters

Modern CPUs contain hardware counters that record time or cycles.

### Timestamp Counter in `x86`

In the past, before [Intel Pentium](https://en.wikipedia.org/wiki/Pentium), processors did not keep time.
The system relied on external timers such as [the real-time clock](https://en.wikipedia.org/wiki/Real-time_clock) or [programmable interval timer](https://en.wikipedia.org/wiki/Programmable_interval_timer).
Reading these timers required I/O operations and gave coarse resolution (milliseconds).
There were no CPU instructions to obtain high-resolution timestamps.

Intel introduced the `RDTSC` instruction ([Read Timestamp Counter](https://en.wikipedia.org/wiki/Time_Stamp_Counter)), and its build-in Timestamp Counter (TSC), with the Pentium.
`RDTSC` reads the TSC -- a 64-bit register that counts CPU cycles since reset.
The instruction loads the low 32 bits in `EAX` and the high 32 bits in `EDX`, giving a monotonically increasing 64-bit value.
In 64-bit systems (`x86-64`), for backwards compatibility, it still loads the values in `RAX` and `RDX` correspondigly, clearing the 32 higher bits.

For a long time after its introduction TSC incremented once per CPU cycle.
This was ideal for profiling as it captured very acurately how many CPU cycles elapsed during the execution of some code block, without any OS overhead.
If you "sandwitch" a block of code between two `RDTSC` calls, the difference approximates the cycles taken:

```asm
RDTSC ; Read TSC at start
      ; Execute instructions
RDTSC ; Read TSC at end
      ; Measure the difference
```

Unfortunately, with the instroduction of multi-core processors, this became more complicated.
On multi-core or multi-cpu systems, the TSC values on different cores may not be synchronized.
Programs measuring time on one core and finishing on another can get negative or inconsistent results.
Additionally, power management features such as [SpeedStep](https://en.wikipedia.org/wiki/SpeedStep) or [Turbo Busting](https://www.intel.com/content/www/us/en/gaming/resources/turbo-boost.html) change the CPU frequency, so the counter's tick rate changes.
Each core may run at a different frequency, leading to inconsistent cycle measurements.

To solve these issues, Intel and AMD instroduced [constant](https://aakinshin.net/vignettes/tsc/#:~:text=Generation%202%3A%20Constant%20TSC) and [invariant](https://aakinshin.net/vignettes/tsc/#:~:text=Generation%203%3A%20Invariant%20TSC) TSCs.
Constant TSC increments at a fixed rate regardless of core clock changes; however it might stop in deep sleep states.
Invariant TSC runs at a constant rate in all power-management states.

This means that `RDTSC` reads a monotonically increasing counter with constant frequency, which is essentially a high-resolution wall clock.

<!--TODO:CPUs still offer ways to get the number of cycles, but not

We can still count cycles (RDPRU?) RDPMC if you can do some priviliged operations. But RDTSC is the only thing you can count on calling in userspace.
The other instructions would depend on the specific CPU, whether the OS allows you to call these instructions, some might need specific drivers etc.
RDTSC is not the best thing to use for profiling, it's a high resolution wall clock, but you can always count on.
RDTSC is that, basically a wall clock, but we would like to convert it to units we are used to, like milliseconds etc.
The expected things would be to also have an instruction that gives you the TSC counter, but not all vendors have it or document it at least.
CPUID 15h?
How to detect constant_tsc and nonstop_tsc flags in /proc/cpuinfo in Linux. How else?
A way to do it is use an OS process with known wall clock time, time it in TSC ticks and estimate the frequency.
QueryPerformanceCounter, QueryPerformanceFrequency-->

#### Additional References

- Section 21.7 in [Intel64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

### Generic Timer in ARM

The [Generic Timer](https://tc.gts3.org/cs3210/2020/spring/r/aarch64-generic-timer.pdf) provides a standardized timer framework for Arm cores.
It includes a System Counter and set of per-core timers.
The System Counter is an always-on device, which provides a fixed frequency incrementing system count.
The system count value is broadcast to all the cores in the system, giving the cores a common view of the passage of time.
The system count value increments with [frequency](https://developer.arm.com/documentation/ka005977/1-0/?lang=en) typically in the range of 1MHz to 50MHz, or 1GHz (effective) more recently.

Each core has two types of timers, a physical and a virtual.
Physical timers, compare against the counter value of the System Counter.
This value is referred to as the physical count and is reported by `CNTPCT_EL0`.
Virtual timers compare against a virtual count, which is calculated as the physical count minus some offset, and is reported by `CNTVCT_EL0`.
This offset can be specified in a register which is only accessible to EL2 and EL3.

Similarly to `RDTSC`, reads to `CNTPCT_EL0` or `CNTVCT_EL0` can be made speculatively.
This means that they can be read out of order regarding the program flow.
This could be important depending on your usecase.
We can serialize the read instruction with an `ISB` fence, like in the following code:

```asm
loop:
  LDR X1, [X2]
  CBZ X1, loop
  ISB
  MRS X1, CNTPCT_EL0
```

`CNTFRQ_EL0` reports the frequency of the system count.
However, this register is not populated by hardware.
The register is write-able at the highest implemented Exception level (EL) and readable at all Exception levels.
Firmware, typically running at EL3, populates this register as part of early system initialization.
Higher-level software, like an operating system, can then use the register to get the frequency.

#### Additional References

- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0406/cd/?lang=en)

## OS Clocks

Operating systems provide stable and high-resolution ways to measure time.
These system clocks typically rely on hardware counters (like TSC, or the System Counter) that increment at a constant rate.

### MacOS

MacOS provides several APIs for retrieving monotonic clock values, with the most common low-level interfaces being [`mach_absolute_time`](https://developer.apple.com/documentation/driverkit/mach_absolute_time) and [`mach_continuous_time`](https://developer.apple.com/documentation/driverkit/mach_continuous_time).
Both `mach_continuous_time` and `mach_absolute_time` return current value of a clock that increments monotonically in tick units.
The difference is that the former includes any the time the system might have slept, while the latter does not.
Therefore, `mach_continuous_time` is important for keeping time, while `mach_absolute_time` is more appropriate for performance profiling.

It’s important to note that the value returned by the Mach time functions is expressed in clock ticks, not nanoseconds.
Historically, `machTimestamp` values effectively represented nanoseconds, because on Intel-based Macs each Mach clock tick corresponded to one nanosecond.
However, on Apple Silicon systems, this is no longer true.
The tick frequency of the underlying hardware timer—used for Mach’s high-precision timekeeping—now differs from one nanosecond per tick.

On Intel processors, converting time intervals was straightforward since one tick equaled one nanosecond.
On Apple Silicon, you must instead apply a conversion factor to [convert](https://developer.apple.com/documentation/apple-silicon/addressing-architectural-differences-in-your-macos-code#Apply-Timebase-Information-to-Mach-Absolute-Time-Values) from ticks to nanoseconds.
This conversion factor is provided by the [`mach_timebase_info`](https://developer.apple.com/documentation/driverkit/mach_timebase_info_t) structure.
While on macOS the numerator and denominator are often equal (making the factor effectively one), on other systems—or future models—these values may differ significantly.
As a result, the correction can vary across hardware models and may be substantial.

Additionally, macOS implements the standard POSIX time APIs (clock_gettime, etc.), but internally these functions call into the Mach time services for their implementation.
Therefore, in `tuff` we use `mach_absolute_time()` directly to avoid any additional overhead.

<!--#### Additional References

https://developer.apple.com/library/archive/qa/qa1398/_index.html
https://eclecticlight.co/2020/09/08/changing-the-clock-in-apple-silicon-macs/
https://eclecticlight.co/2017/02/23/so-many-times-the-clocks-in-your-mac/-->

<!--### `mach_absolute_time`

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
```-->
