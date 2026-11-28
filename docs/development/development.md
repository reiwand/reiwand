# Development

Build dependencies (Debian/Ubuntu, with GStreamer 1.24 or later, as on Ubuntu 24.04):
`libpipewire-0.3-dev libclang-dev pkg-config libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
qt6-base-dev qt6-declarative-dev`. For the tests: `mesa-vulkan-drivers pipewire wireplumber
dbus-x11 gnome-keyring gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-pipewire
gstreamer1.0-tools`, `sway grim foot` for the Sway one and `xvfb x11-utils x11-xserver-utils
xdotool imagemagick openbox` for the X11 one.

```sh
cargo test --workspace                   # unit, render and video tests
scripts/private-audio.sh cargo test --workspace  # same, with a PipeWire of its own and a mute output
cargo build -p reiwand-pipewire --example probe
cargo build -p reiwandd -p reiwand-ctl && cargo build -p reiwandd --example timedate
cp target/debug/examples/probe target/debug/
scripts/integration-test.sh target/debug # D-Bus + PipeWire + daemon end to end
scripts/regression.sh target/debug       # regression net: images, per-frame counters and idle cost (docs/development/regression-net.md)
scripts/sway-test.sh target/debug        # layer-shell surfaces in a Sway without a screen
scripts/x11-test.sh target/debug         # desktop windows in an Xvfb
cargo build -p reiwand-server && cargo build -p reiwand-android --example phone
cp target/debug/examples/phone target/debug/
scripts/package.sh                       # generates dist/reiwand-*-x86_64.AppImage
scripts/translations.sh update [language]  # updates po/ with the texts of the QML, the validator, the editor and the reiwand-ctl help
target/debug/reiwand-app --check            # loads all the app's QML and fails on any warning
target/debug/reiwand-app --check scene.* compose.page   # only the check units that match
target/debug/reiwand-app --check --list     # names the check units, runs none
target/debug/reiwand-app --check scene.* compose.page   # only the check units that match
target/debug/reiwand-app --check --list     # names the check units, runs none
scripts/screenshots.sh [OUT]               # captures the app pages as a new user sees them, into docs/images
```

Changes reach `master` through pull requests. Each pull request runs the *Checks* workflow
(`.github/workflows/checks.yml`): static checks, Clippy, the unit and render tests and, when the
phone code changed, an Android build; every job stops at 20 minutes. The *Release* workflow
(`.github/workflows/ci.yml`) runs only on `master`: it builds and publishes. The full test set,
with the end-to-end, compositor and Flatpak tests, is the *Tests* workflow
(`.github/workflows/tests.yml`), which is launched by hand.

`reiwand-app --check` goes through each page with test data, opens the details of an item, the
image editor and the scene editor, and fails on any QML warning or if it does not finish in 240 s.
In the scene editor it opens and uses the curve dialog, the timeline menu with its panel, the
mask, particle and force tools; in a temporary folder it creates a static repository with test
keys from the *Static repository* tab (dialogs *New repository…*, *Root…*, *Revoke* and
*Publish…*), adds a package, revokes and withdraws the revocation of a version, publishes and
checks that the page is drawn. It also goes through each page with the keyboard only (Tab, Enter,
space and Escape) and fails if a control cannot be reached with Tab, if the library items cannot
be chosen or applied, if the side menu does not open with Enter or close with Escape, or if the
mouse wheel changes a dropdown or a slider that has not been pressed. Each part of the scene
editor check removes what it placed on the page when it ends, and, once the editor has drawn a
frame after the checks, it fails if anything outside the workbench is drawn over it or if what a
panel lays out goes past the group that holds it. With the `offscreen`
platform (the default) its windows open on a screen of 1600 × 900. The QML files follow the
`qmlformat` format of Qt 6.12.

The walk is cut into check units, which `REIWAND_APP_CHECK_SHARDS` processes share. Patterns after
`--check` (`*` for any run of characters, `?` for one, matching the whole name) run only the units
they match: `scene.empty`, `scene.particles`, `scene.parallax` and `scene.clock` are the scene
editor with each starter (the timing, timeline, mesh, script and media checks are in
`scene.empty`), `compose.page`, `compose.clip`, `compose.still` and `compose.preset` the editor of
a web page, video, picture and preset package, `page.*` the app's pages and `keys.*` the keyboard
walks. What sits outside every unit still runs: the app starts, the scene editor opens an empty
scene and checks its frame, workbench, bounds and bars. `--check --list` names the units the walk
reaches without running them. A pattern that matches no unit fails. Without patterns every unit
runs, as before; work is checked on its own area while it is made, and the whole walk runs before
it is merged.

The walk is cut into check units, which `REIWAND_APP_CHECK_SHARDS` processes share. Patterns after
`--check` (`*` for any run of characters, `?` for one, matching the whole name) run only the units
they match: `scene.empty`, `scene.particles`, `scene.parallax` and `scene.clock` are the scene
editor with each starter (the timing, timeline, mesh, script and media checks are in
`scene.empty`), `compose.page`, `compose.clip`, `compose.still` and `compose.preset` the editor of
a web page, video, picture and preset package, `page.*` the app's pages and `keys.*` the keyboard
walks. What sits outside every unit still runs: the app starts, the scene editor opens an empty
scene and checks its frame, workbench, bounds and bars. `--check --list` names the units the walk
reaches without running them. A pattern that matches no unit fails. Without patterns every unit
runs, as before; work is checked on its own area while it is made, and the whole walk runs before
it is merged.

The rules of the look (tones, type, spacing, components) for the desktop app, the phone app and the
server pages are in [design.md](design.md).

Android APK (with the SDK, NDK 28 or later and `cargo install cargo-ndk`):

```sh
rustup target add aarch64-linux-android x86_64-linux-android
ANDROID_NDK_HOME=/path/to/ndk android/gradlew -p android assembleDebug
adb install -r -d android/app/build/outputs/apk/debug/app-arm64-v8a-debug.apk
android/gradlew -p android assembleRelease -PtestSuffix=perf  # installs next to the normal app as "Reiwand perf", with the profileable manifest to measure with perfetto and simpleperf
android/gradlew -p android connectedDebugAndroidTest -Pabis=x86_64  # tests on an x86_64 emulator
scripts/android-power.sh -o power.csv  # power use, fps and memory per scenario on the phone
ANDROID_NDK_HOME=/path/to/ndk scripts/apk-check.sh android/app/build/outputs/apk/release/app-universal-release.apk
WANT="arm64-v8a " scripts/apk-check.sh android/app/build/outputs/apk/release/app-arm64-v8a-release.apk
```

Gradle makes one APK per architecture (`app-arm64-v8a-…`, `app-x86_64-…`) and, with more than one,
the universal (`app-universal-…`); the release one goes through R8, which removes the unused code
of the libraries and leaves the app's whole, which Rust calls over JNI. `scripts/apk-check.sh`,
which the CI runs on the three published APKs, checks that each one carries exactly the
architectures it should with its Rust library, that it carries no ONNX Runtime, that all the
libraries are aligned to 16 KB pages, that the APK is aligned for `zipalign`, that the Rust
library does not keep paths of the computer that built it and, with `scripts/apk-signer.sh`, that
its signature verifies (`apksigner`) with a single signer. With `CERT_SHA256` (the CI gives it the
variable `ANDROID_CERT_SHA256`: the SHA-256 of the release certificate, from `keytool -list -v
-keystore reiwand.keystore`) the signer must be that certificate; on `master` of the main
repository a missing variable fails the job, so a published APK is always the release key's.

`scripts/android-power.sh` goes through the power scenarios over adb (still image, scrolling,
covered wallpaper, screen off, power saving, high temperature, parallax with the phone still and
video, 30 minutes and 3 times each) and adds a row per measurement to the CSV. With Perfetto's
`trace_processor` in the `PATH` it uses the phone's power rails; if not, the batterystats
estimate. It is best to connect adb over Wi-Fi so that the cable does not charge the phone during
the measurement.

The CI signs the APK with the first key of a PKCS12 keystore stored in base64 in the secret
`ANDROID_KEYSTORE`, with its password in `ANDROID_KEYSTORE_PASSWORD`. Both are secrets of the
environment `android-signing` (Settings, Environments), whose deployment branches are limited to
`master`, and not secrets of the repository: only the `Android APK` job of a push to `master` of the
main repository runs in that environment and passes them to its signing step, and GitHub refuses the
environment to a run of any other branch, even one whose workflow was changed to ask for it. Pull
requests and other branches build with a temporary key and say so; on `master` of the main
repository (what gets published) the job fails without the key. The keystore is removed from the
runner when the step ends. It is created once with:

```sh
keytool -genkeypair -keystore reiwand.keystore -storetype PKCS12 -alias reiwand \
  -keyalg RSA -keysize 4096 -validity 36500 -dname CN=reiwand
base64 -w0 reiwand.keystore
```

A local build uses that keystore with `REIWAND_KEYSTORE=/path/to/keystore` and
`REIWAND_KEYSTORE_PASSWORD`; if not, it is signed with the computer's debug key and before
installing it the published APK has to be removed with `adb uninstall io.github.reiwand`, which
also deletes the chosen image. Its version number is 1 and the CI's is that of each run, hence the
`-d` of `adb install`.

The desktop download is an AppImage. `scripts/package.sh` (with `REIWAND_BIN_DIR`, from binaries
already built) makes the AppDir by hand: the four binaries in `bin/`, the Plasma packages with
their translations, the KWin script, the example scenes, `packaging/` (unit, D-Bus services,
launcher, MIME type, icons), the notices and `packaging/appimage/` (`AppRun`, `integrate.sh` and
`remove.sh`). It squashes it with `mksquashfs` (zstd, owners root, every time set to
`SOURCE_DATE_EPOCH`, the date of the last commit) and puts in front the static type 2 runtime of
AppImage/type2-runtime, pinned by version and SHA-256 in `scripts/tools.sh`; there is no
appimagetool, linuxdeploy or update information. The app and the web helper carry their own Qt:
`scripts/qt-prefix.sh <dir>` makes a Qt prefix from the conda-forge packages listed with their
SHA-256 in `packaging/appimage/qt.lock` (Qt 6.12.1, Qt WebEngine, CMake and their libraries,
installed with a pinned micromamba) and builds extra-cmake-modules, Kirigami, KCoreAddons and KI18n
6.32 and KPipeWire 6.8.3 (the live previews of the app) from pinned commits; the CI keeps that prefix in its cache, builds `reiwand-app` and `reiwand-web` against it
(`QMAKE=<dir>/bin/qmake6`) and `package.sh` takes it in `REIWAND_QT_PREFIX`.
`scripts/qt-bundle.py` copies into `qt/` the plugins, the QML modules, `QtWebEngineProcess`, the
engine's resources and the licences, then every library those files and the two programs load
from the prefix, except the ones that must be the system's (glibc, libGL, EGL, libdrm, GBM,
Vulkan, ALSA, fontconfig, D-Bus, PipeWire, libepoxy and VA-API) and the recording module of
KPipeWire, which the app does not use, sets each RPATH relative to `qt/lib` and writes `qt.conf`.
The daemon and `reiwand-ctl` use no Qt. GStreamer and PipeWire stay the system's, and so does
the Qt of the Plasma plugins, which run inside plasmashell. KPipeWire is built against the system's
headers of PipeWire, libepoxy, libdrm, GBM, VA-API, FFmpeg and libmount (on Debian and Ubuntu
`libpipewire-0.3-dev libepoxy-dev libdrm-dev libgbm-dev libva-dev libavcodec-dev libavformat-dev
libavfilter-dev libswscale-dev libmount-dev`). To build it by hand:

```sh
scripts/qt-prefix.sh ~/qt                 # needs git, a C++ compiler, gettext and the headers above
QMAKE=~/qt/bin/qmake6 REIWAND_QT_PREFIX=~/qt scripts/package.sh  # builds the release binaries first
```
`AppRun` runs one of the programs (`reiwand-x86_64.AppImage reiwand-ctl …`), and `--install`
copies the AppDir to `~/.local/lib/reiwand/<version>`, switches the `current` link to it, removes
the other versions, links the commands in `~/.local/bin` (`reiwand` is `AppRun`) and writes the
unit, D-Bus services, launcher and Plasma packages pointing to those links. The service never runs
from the mounted image: the unit has `NoNewPrivileges`, which stops the setuid `fusermount`, and
the daemon trusts `reiwand-ctl` and the app only when they are the files next to its own
executable. An update installs the new AppImage the same way and restarts the service.
`scripts/appimage-check.sh dist/reiwand-*-x86_64.AppImage` (with `REIWAND_VERSION`), which the
CI runs, checks the owners and date of the image, the versions of the programs and of the Plasma
packages and that the binaries keep no paths of the build machine. The RPM is built from the
same AppImage: `scripts/rpm.sh dist/reiwand-*-x86_64.AppImage` unpacks it with `unsquashfs` and
installs its `bin/` and `qt/` in `/usr/lib64/reiwand`, with the commands linked in `/usr/bin`.

Local Flatpak (builds without network on the KDE 6.11 SDK, with Rust 1.99.1 from
static.rust-lang.org and the Cargo sources downloaded earlier by `flatpak-builder`, checked with
their SHA-256). Only the files that git tracks go in, with their uncommitted changes; those it does
not track, such as other working copies inside the folder, stay out. The SDK and the LLVM extension
come from Flathub:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
packaging/flatpak/sources.sh            # copies to target/flatpak-src and generates cargo-sources.json
flatpak-builder --user --install --install-deps-from=flathub --force-clean \
  --state-dir=target/flatpak-builder target/flatpak \
  packaging/flatpak/io.github.reiwand.Reiwand.yml
```

With `REIWAND_VERSION`, `sources.sh` puts that version in the copy (without it, that of
`Cargo.toml`) and `packaging/flatpak/releases.sh` writes the `<releases>` list into the metainfo of
the copy: that version with the date of the last commit and the news and fixes since the previous
tag, and the ten versions published before, so `appstreamcli` no longer warns of
`releases-info-missing`. The CI builds the Flatpak for x86_64 and for aarch64, the latter on a
GitHub ARM runner (free on public repositories), and `packaging/flatpak/check.sh` installs each
package and checks the versions, `reiwand-app --check` without a screen, the exported files
(launcher, icon, metainfo and D-Bus services), the `reiwand-plasma` commands and that an edit made
inside the Flatpak reaches `~/.local/share/reiwand/edits/`. It installs on the system and touches
that folder, so outside the CI it is best to use it in a disposable container. The daemon inside
the sandbox is tested by another job on the runner without a container, with the package already
built: `scripts/private-audio.sh packaging/flatpak/serve-check.sh` (the build image has no sound
server, and the daemon always needs one).

When a version is published, the *Flatpak repository on Pages* job adds both bundles to the
Flatpak (OSTree) repository on the `gh-pages` branch with `scripts/flatpak-repo.sh`: it signs the
commits and the summary with the key in the `FLATPAK_GPG_PRIVATE_KEY` secret (it refuses any key
other than the one in `packaging/flatpak/repo-key.asc`), keeps only the newest commit of each
architecture plus a static delta from the previous one, writes `reiwand.flatpakrepo` and
`io.github.reiwand.Reiwand.flatpakref`, and rebuilds the bundles with the repository's address and
key so they update from it. `scripts/pages-publish.sh` pushes the branch as a single commit (the
objects of older versions do not stay in its history) and leaves the rest of the site alone. The
release then attaches those bundles; if the job fails, it attaches the bundles as built. Pages
serves the `gh-pages` branch (repository settings, *Pages*, *Deploy from a branch*). With each published version the *Documentation on Pages* job
also builds this documentation as a website with `scripts/docs-site.sh` (`docs-site.py` and
pandoc; each page of `docs/` becomes an HTML page at the same path, links between pages follow and
show the title of the page they lead to, links to files outside `docs/` go to GitHub; side
navigation by the sections of `docs/README.md`, breadcrumbs, previous and next page, a table of
contents per page and a search over a `search.json` index built with it, in the fonts and tones of
`design.md`, dark or light as the system asks; template, styles, search and link filter in
`packaging/docs/`) into the `docs/`
folder of the same branch, with `index.html` at the root sending to it. Both jobs share the
concurrency group `pages`, so they never push the branch at the same time. The *Documentation site*
job of the *Tests* workflow builds the site the same way, so a page that breaks it shows before a
release.

All GitHub actions are pinned by the full SHA of their commit, container images by their digest and
the tools that the CI downloads (`scripts/tools.sh`: actionlint, the AppImage runtime, cosign,
micromamba and syft) by version and SHA-256. The *Tests* workflow, which runs only when started by
hand, passes actionlint, with shellcheck, over all the workflows. When publishing a
version, `scripts/release.sh` generates with syft the SBOM of the files that git tracks and creates
the version as a draft; once the Flatpak bundles are built, `scripts/release-finish.sh` adds them,
signs `SHA256SUMS` once with cosign without a key of its own (GitHub OIDC; the job only has write
permission on contents and to request the token) and, with `scripts/sign-sums.sh`, with the release
key of the `FLATPAK_GPG_PRIVATE_KEY` secret (`SHA256SUMS.asc`, which `reiwand --update` checks; it
refuses any other key), and publishes it, so no updater ever sees checksums and a signature that do
not match (without the bundles, the version is published without them). `scripts/server-image.sh`
builds the server image with a
pinned BuildKit, attaches the SBOM and the provenance, pushes it to GHCR, signs its digest and adds
the attestation with the SBOM of the sources. The first image creates the `reiwand-server` package
as private: it has to be made public once in its GitHub settings.

Web wallpapers are tested where Qt WebEngine exists: the *Web wallpapers* job of the *Tests*
workflow (started by hand) runs, in an
Arch container with `qt6-webengine`, the helper test (`apps/reiwand-web/tests/page.rs`: the
`window.reiwand` interface, the FPS limit, the pause, that the page does not reach the network,
other packages or files of the computer) and `scripts/web-test.sh` with the service, `reiwand-ctl`
and a page of one color. On Ubuntu, `scripts/integration-test.sh` checks that the wallpaper is
rejected with its reason. `REIWAND_REQUIRE_WEB=1` makes both fail if the engine is missing. The same
job runs `scripts/video-test.sh` (`scripts/integration-video.sh`) in that Arch container, with
PipeWire and `gst-plugin-pipewire` 1.6: it is the only test of the direct sound branch through
`pipewiresink` 1.2 or later (the report that there is no audio output, a null sink, the volume and
the silence), because on Ubuntu 24.04 the plugin is 1.0; `REIWAND_REQUIRE_PIPEWIRE_SOUND=1` makes it
fail if the plugin is older than 1.2. On the phone, `WebPageTest` (instrumented tests on the
emulator) shows a test page with the WebView and checks its values, the color it draws, the pause,
the frame limit, what each privacy setting hides, the deletion of its data, the chosen files,
clicks only with permission and that it does not reach the network, other packages or files of the
phone.

The *Tests* workflow also runs `scripts/plasma-test.sh` in a container with Plasma 6: it installs
the package with `--appimage-extract-and-run --install` of the AppImage, starts a Wayland session without a screen (KWin and plasmashell over virtual
DRM devices), activates the plugin, checks the pixels of a capture, loads the tray icon, opens the
app on its *Settings* page from the tray button and with `--page settings`, checks on a separate
bus that the tray only offers the app where it is installed and, with `qmltestrunner` from
`qt6-declarative` (`tests/tray-row.qml`), that the thumbnail of a screen, in the tray and in the
app, stays blurred with an adult package until it is pressed with the mouse or keyboard, and that
the tray warns if the wallpaper flashes. It also runs `scripts/sway-test.sh` in a Sway without a
screen: it captures each output with `grim`, connects and removes a screen and checks that the
video pauses under a full-screen window and with the screen off (Sway draws in software; with
`REIWAND_SWAY_RENDERER=gles2` it draws with the graphics card, which needs a driver that does not
present on shared-memory surfaces), and `scripts/x11-test.sh` in an Xvfb with two monitors, the
root window background and the same pause.

| Path | Contents |
|------|----------|
| `crates/reiwand-core` | Configuration, fit modes, D-Bus names |
| `crates/reiwand-render` | wgpu renderer, windowless or over Wayland, X11 and Android surfaces, with scenes and YUV video conversion |
| `crates/reiwand-depth` | Depth maps: model on demand, computation with ONNX Runtime and cache |
| `crates/reiwand-scene` | Scene model: layers, bound values, timelines and effects, without a GPU, and its JSON schema |
| `crates/reiwand-editor` | Scene editor shared by the app and `reiwand-ctl scene`: operations, history, starting points, inspector and scene folders |
| `crates/reiwand-pipewire` | Export of PipeWire video streams |
| `crates/reiwand-video` | Looping video playback and scene sound with GStreamer |
| `crates/reiwand-mix` | Mixer of the sound layers of a scene (lists, pauses, fades, panning and alignment with the scene), shared by the desktop and Android |
| `crates/reiwand-wayland` | Layer-shell surfaces and X11 desktop windows, scale, pace and occlusion |
| `crates/reiwand-audio` | Sound capture with PipeWire and its analysis into bands for scenes, also from the spectrum Android gives |
| `crates/reiwand-script` | Scene scripts in JavaScript, isolated in one process per scene with CPU and memory limits, its `reiwand` module and `reiwand/math` |
| `crates/reiwand-account` | Client of the server of your own for accounts: code sign-in, certificates pinned per origin and the accounts file, shared by the desktop and Android |
| `crates/reiwand-carry` | The `.reiwand` file to export and import the configuration: container, encryption, limits against hostile input and merge rules |
| `crates/reiwand-fill` | Tests only: writes the omitted fields of the scenes, manifests, edits and short descriptors in tests; the integration scripts use it with `cargo build -p reiwandd --example complete` |
| `crates/reiwand-android` | Android live wallpaper in Rust: JNI, shared GPU, one surface per wallpaper, library, lists, settings, scenes, parallax, depth maps, repositories with per-repository accounts, pinned certificates and exporting and importing the configuration |
| `apps/reiwandd` | Daemon |
| `apps/reiwand-ctl` | CLI |
| `apps/reiwand-app` | Desktop app (Qt Quick and Kirigami over Rust with cxx-qt) |
| `apps/reiwand-server` | Server of your own: repository, catalog, uploads, moderation and accounts |
| `android` | Android app: Gradle project, wallpaper service, video player and Compose interface in Kotlin |
| `integrations/plasma-wallpaper` | Plasma 6 wallpaper plugin |
| `integrations/plasma-tray` | Management icon in the Plasma 6 tray |
| `integrations/kwin-pointer` | KWin script that sends the pointer position while a scene or a parallax uses it |
| `po` | gettext translations of the Plasma packages and the app |
| `packaging` | systemd service, D-Bus activation, launcher and icon, the AppImage's entry point and install scripts, RPM, Flatpak |
| `deploy` | Nginx example in front of the server and a `compose.yaml` variant that builds the image from the sources |
| `docs` | Specifications of the D-Bus interface, packages and their scenes, the scene editor, signed repositories and the server |

## How it works

```
 Plasma (wallpaper plugin, QML)                   reiwandd (daemon, Rust)
 ┌──────────────────────────┐   D-Bus             ┌───────────────────────────┐
 │ RequestStream(screen,    │ ──────────────────► │ renders with wgpu         │
 │   width, height)         │ ◄── node, serial ── │ exports a PipeWire stream │
 │ PipeWireSourceItem       │ ◄══ PipeWire video ═│ per screen                │
 └──────────────────────────┘                     └───────────────────────────┘
                                   reiwand-ctl (CLI) ──► D-Bus
```

- `reiwandd` renders on the GPU without a window of its own and publishes one PipeWire stream per
  screen.
- The Plasma plugin asks for a stream of the exact size of its screen and shows it with KPipeWire,
  without covering desktop icons or widgets.
- `reiwand-ctl` and the desktop app control the daemon: each action is in both. The tray icon
  offers the most frequent ones and opens the app.
- Videos are decoded with GStreamer and pause when a maximized or full-screen window covers the
  screen, on battery or with the screen locked; with the screen locked they never sound, even if
  they are not paused.
