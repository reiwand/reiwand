# Passes, the check and compiling

On this page: passes, the shader check, where shaders are compiled and the variants built.

## Passes and the shader check

A package effect has at most 24 passes. Its shader is checked for the work it can do before anything
reaches the graphics driver, on a thread of its own with a 64 MiB stack, so that no shader can
exhaust the stack of the program that reads it. Before naga parses it, a scan of its text refuses
an expression more than 256 operations deep (`a + a + ...`, unary signs, member accesses, indices
that follow a bracket such as `a[0][0]`, `else if` chains and the brackets around them all count; a
number ends before a `.` that a letter follows), more than 64 nested brackets of any kind and more
than 4 096 arguments to calls and constructors in the whole shader; a `//` comment ends at every
line break WGSL knows (`\n`, `\r`, vertical tab, form feed, U+0085, U+2028, U+2029) and the
Unicode blank space WGSL allows counts as blank. The same limits hold again on the parsed shader,
whatever its text: an expression more than 256 operations deep (a named `let` starts a new one),
more than 64 nested blocks and more than 4 096 arguments and constructor parts are refused.
Then: at most 32 768 expressions; every loop counts an `i32` or `u32`
variable from a literal start to a limit that is a literal, a constant, an `override` with listed
`options`, or `min(x, <literal>)`, steps it once in the loop's `continuing` part by a literal or a
constant of at least 1 and never writes it elsewhere inside the loop (`for (var i = 0u; i < 16u;
i++)` qualifies; float counters, steps that come from an `override` or `min`, `while` loops, `break
if` and counters passed to functions or compared through a conversion do not); a conversion
such as `u32(m)` in a start or a limit counts only when the value it converts is known and fits the
new type unchanged, and `bitcast` never counts; the start, the limit and the value the counter has
when the loop ends must fit its type, so `i >= 0u` counting down, `i <= 4294967295u` or
`u32(-1i)` are refused; a loop runs at most 1 024 times. Every entry point is weighed in scalar
operations a pixel, nested loops and called functions included: one per scalar of a value an
expression computes, loads or stores and one per statement, a matrix product as rows × columns ×
the shared size, `sin`, `exp`, `sqrt`, `normalize` and the like as four, an integer division or
remainder as 32 and a float remainder as four per component, a texture read as 16 and
a call as all of the function it calls; the statements of a loop before its condition count once
more than its rounds, since they run once more. A read or a write at a computed index into a function or
`private` array or matrix that the shader writes element by element or passes by pointer costs
F × min(F, 1 024) / 32 more, where F is the size in bytes of all the arrays the shader indexes that
way together (32 768 for 1 KiB): such arrays leave the registers, and a scattered access to them
costs as much as thousands of operations; a read at a computed index into a `const`, into a
variable only ever written whole, or into an array value costs as much as a texture read;
a function never receives a pointer into a function variable or to an element picked that way. An
entry point, and the `material` or `particle` function of a stage effect, may run at most 1 048 576
operations a pixel, and may be at most 32 768 operations long once every call counts the whole
function it calls again, since drivers copy called functions into their callers; the passes of an
effect together, vertex entries included, are at most 131 072 operations long. Function and
`private` variables, every array and structure value the shader computes (call results, loads,
constructors and arguments included, all counted at once) and the deepest chain of calls may keep
at most 8 KiB a pixel. Across the whole shader, every function included even if nothing calls
it, the values that start at zero (variables without a value and empty constructors such as
`array<vec4<f32>, 64>()`) are at most 64 KiB together, since some drivers receive each of them
written out element by element. The
check is an early, readable refusal, not the only safeguard: the engine also builds every package
shader with a hidden counter in each of its loops, so any loop stops after 1 024 rounds each time
it starts, and every loop a pixel runs stops once the loops of that pixel have made 65 536 rounds
together, whatever the check concluded. Option values must fit the type of their `override`.
Targets are at most twice the layer size and never larger than the largest texture of the device,
and parameters fill 24 slots (`param(0u)` to `param(23u)`). The descriptor's `cost` counts at least
one unit for each pass that draws, times the square of its scale, nine times for a pass with a
`grid` (twice for a grid of one cell), whose operations and texture reads count nine times too, one unit for every four texture reads its shader can make a pixel and one unit for
every 512 operations, loops and the stage function of a `material` or `particle` effect included,
whatever the descriptor says; its numbers must be finite and
at least 0. A layer whose effects read `backdrop` adds two units for the copy, and four for a
`compose` effect. The engine shrinks every triangle a vertex entry draws, around its middle, to at
most three cells of the grid a side, so a grid that stretches its cells over the layer draws each
pixel nine times on average at most; a vertex entry that bends the grid moves its cells freely.

## Where shaders are compiled

On the desktop, the daemon and its workers, the app with its editor, `reiwand-ctl` and `reiwand-server`
run that check (the scan of the text, naga's parse and validation, the limits above and the rewrite
that adds the loop counters) in a short-lived helper first: a copy of their own program
(`/proc/self/exe`, so it still works after an upgrade replaced the file) started with the hidden
argument `--reiwand-check-shader` and an empty environment, which limits itself to 1 GiB of address
space, 4 seconds of processor time and no core file (or the lower hard limits it inherited; when it
cannot lower them it refuses with `the checker could not limit itself: <error>`) before it reads the id, the descriptor and the
shader on its standard input, and answers with the hash of what it checked and its refusal, if any.
It is killed after 10 seconds. The daemon's workers, which lower their own priority, start the
helper from a thread that keeps the priority they started with. Only when the helper accepted exactly those files does the program
check them again itself (which also gives the descriptor its measures), and only effects that passed
that check are ever built for the graphics card. A refusal of the helper refuses the package with
the same reason; a helper that crashes, runs out of memory or processor time, takes too long or
does not start refuses it with `its shader could not be checked: the checker ...`, without a
strike, since the program itself never ran the check. Verdicts are kept by a SHA-256 of the program
version, the limits, the id, the descriptor and the shader, in memory and in `checked-effects`
beside `stopped-effects` (lines `<hash> ok` or `<hash> refused <reason as a JSON string>`, the
4 096 used last, rewritten under a lock and renamed into place, with up to 4 096 kept in memory), so the helper runs once for each version
of an effect, when it is first read, and a new release checks every effect again; failures of the
helper are kept nowhere, and the daemon reads the library packages they refused again when it starts
and on `RetryEffects`. The phone keeps the check in its own process, bounded by the limits of naga's
constant evaluation.

At start-up, the first opening of a foreign library is serialised in the whole process, because the
graphics driver and a codec plug-in take the dynamic loader's lock and the driver's own lock in
opposite orders, and two threads doing one each would wait for each other for ever (the daemon
starts the renderer and the first video on different threads). The one-time paths that may open such
a library run under one lock (`reiwand_core::loader::serialised`): creating the graphics instance,
asking for an adapter or a device, listing adapters, creating the first video element or pipeline,
and loading the depth model runtime. Starting video support also opens the plug-ins of hardware codecs
right then, under that lock, instead of on the first thread that needs one. The lock is never taken
per frame.

## Building passes and variants

The engine builds every pass and every variant a use may need when the scene loads, each inside an
error scope of the graphics driver, on up to four threads beside the one that draws, and waits for
them at most 10 seconds. On OpenGL ES, whose driver serves one context to one thread at a time, those
threads build each shader first in a context of their own on the same driver, and only a shader
that built there in time is built again in the context that draws, on its thread; the same goes for
the pipelines of particle shaders. So a slow or broken shader is left out on OpenGL ES exactly as
on Vulkan, and never holds the context that draws. Material shaders (`stage: "material"`) follow
the same rule when a 3D stage first draws them: the shader and the pipelines its draws need are
built on a thread beside the one that draws (on OpenGL ES first in a context of their own), waited
for at most the same 10 seconds, and one that is not built in time is left out with `its models are
shaded with pbr instead, since its shader took more than 10 s to build` and struck like a slow
effect; the stage then shades with `pbr` and does not try it again while it lives. A material
pipeline is never built on the drawing thread outside that wait. The package shaders a scene builds may be at most 262 144 operations long
together, each pass counted once for each variant built; the effects past that are left out. An
effect the driver refuses, or whose shader is not built in time, is left out and the layer is drawn
without it; so is every package effect drawn in a frame the driver rejects. Only the variants built
then are drawn: an option value a property binding does not map draws the built variant of the
lowest mapped value, and a use whose variant is missing is left out with `its shader was not built
for these option values` instead of building on the thread that draws. A program that ends cleanly
while a package shader still builds (the daemon on `SIGTERM`, its workers, the app, `reiwand-ctl`)
stops the other builds, waits at most a second for the driver and then exits at once, without the
exit handlers that would race the driver.
