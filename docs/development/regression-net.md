# Regression net

A single command says whether the program still draws the same, as fast and resting the same:

```sh
cargo build -p reiwandd -p reiwand-ctl && cargo build -p reiwand-pipewire --example probe
cp target/debug/examples/probe target/debug/
scripts/regression.sh target/debug          # checks
scripts/regression.sh --bless target/debug  # saves the new values on purpose
```

It runs without network, without a real GPU and without the session bus: the render is the
software one (lavapipe; the command picks its Vulkan driver if installed) and the daemon uses a bus
and a PipeWire of its own, like `scripts/integration-test.sh`. With a real GPU and without lavapipe
the images part is skipped with a notice. In the *Tests* workflow (started by hand) it runs inside the "End-to-end test" job and,
if it fails, uploads the difference images as the `regression-net` artifact.

`--bless` never runs by itself: it is used when a change alters on purpose what is drawn or what it
costs, and the commit carries the new images and numbers so they get reviewed.

## Reference images

`REIWAND_NET=1 cargo test -p reiwand-ctl --test net` draws each scene of `scenes/` (packed as
`.wpak`, with its scripts) at 160×90 at seconds 0, 1 and 5, with wall clock, seed and inputs
fixed, and also a still image, the same image with depth relief (three shifts) and a video
(seconds 0, 1 and 5 of `tests/fixtures/quadrants-64x32.webm`). Each case is a sheet with its frames
in `apps/reiwand-ctl/tests/net/pictures/`; it also draws the first frame of each scene twice and
fails if they do not come out equal.

An image fails when more than 0.4 % of its pixels deviate by more than 24 in some channel or the
mean difference exceeds 1.5. It then writes `NAME-got.png` (what was drawn), `NAME-expected.png`
(the reference) and `NAME-difference.png` (the difference times four) in `target/tmp/net/`, or in
the folder of `REIWAND_NET_OUT`.

## Frame times

The same test stores in `apps/reiwand-ctl/tests/net/counters.json`, per case:

| number | what it is |
|---|---|
| `needed` | frames that had to be drawn in 70 at 60 frames per second (`reiwand-ctl scene bench`); 1 in a still scene |
| `passes`, `draws`, `textures` | passes, draw calls and textures of the last frame |
| `created`, `compiled` | resources and pipelines created after warm-up: must be 0 |
| `particles`, `steps` | particle slots and simulation steps |
| `allocations` | memory allocations per stable frame on the drawing thread |
| `ms` | mean wall time per frame |

The counters do not depend on the machine: they fail if they grow more than 10 % (at least 1) over
what is stored. `ms` does depend: it only fails above eight times what is stored plus 20 ms, to
catch a change of order of magnitude and not machine noise.

Memory allocations include those of the driver, so the file also stores which driver measured them
(`adapter`). The test always uses Vulkan; with another driver (another version of lavapipe, for
example in the CI) the allocations only fail if they exceed double plus 50, and the test says so on
starting. Without `REIWAND_NET` the test does nothing: a `cargo test` of several packages together
activates features of other crates and changes the allocations, so the net runs only with the own
build of `scripts/regression.sh`. The test also sets `RUST_LIB_BACKTRACE=0`: in a debug build the
GPU keeps a stack trace on each lock if `RUST_BACKTRACE` is set, and that would add allocations that
are not the program's. Panics still show their trace.

## Idle cost

`scripts/integration-idle.sh` starts a private daemon with a virtual 160×90 output and watches it
with `probe`, as a compositor would, and measures for 3 s the frames drawn on the output and the
times the daemon wakes up (the sum of context switches of all its threads in `/proc`) in three
cases: a still image, a scene covered by a window and a paused scene. Beforehand it checks that the
scene is drawn when seen. The caps are in `apps/reiwand-ctl/tests/net/idle.txt` (`case frames
wakeups`); `--bless` stores the measured frames and double the wakeups plus 20.

The phone's engine is not part of this net: its tests check when it pauses
(`crates/reiwand-android/src/power/tests.rs`) and at what pace it draws (`power/pace.rs`), but do
not count frames or wakeups at idle; that is measured by `scripts/android-power.sh` on a phone.
