# Contributing to Reiwand

Thank you for wanting to help. These are the rules Reiwand is written with. The principles behind
the product, its look and its scope are in [docs/guidelines.md](docs/guidelines.md); they apply to
every change. How to build and test is in [docs/development/development.md](docs/development/development.md).

## The parts

Reiwand is free software for live wallpapers, written in Rust, for Linux (Plasma first, also wlroots and
X11) and Android, with its own package format, `.wpak`.

| Part | Where |
|---|---|
| Service (daemon) | `apps/reiwandd` |
| Command line | `apps/reiwand-ctl` |
| Desktop app (Kirigami, cxx-qt) | `apps/reiwand-app`, QML in `apps/reiwand-app/qml` |
| Scene editor | `crates/reiwand-editor` and the app |
| Plasma tray applet and Plasma wallpaper type | `integrations/` |
| Self-hosted server | `apps/reiwand-server` |
| Android app | `android/`, `crates/reiwand-android` |

What each part does today is described in `docs/specs/*.md` (`scene.md`, `editor.md`, `dbus.md`, …).
`README.md` is long: search it rather than reading it whole.

## Languages

- Everything in the repository is written in English: code, identifiers, program messages, commit
  messages, the README and the documentation.
- The interface and the documentation may also be offered in other languages, as translations of the
  English text. Interface translations are updated with `scripts/translations.sh update <language>`
  (on Android, `values-<language>`); review every new string by hand so it reads naturally and
  completely.

## Code style

- No comments in code (Rust, WGSL, QML, Kotlin, scripts). Command-line help goes in attributes.
- At most 200 lines per file (`scripts/file-lengths.sh` checks it in the *Tests* workflow). When a
  file you touch would grow past that, split it by responsibility.
- Few files, each with one clear responsibility.
- The shortest readable syntax of each language: closures, iterators, `?`, `let-else`, expression
  bodies in Kotlin, arrow functions in JS.
- Rust: edition 2024 and the latest stable release.
- Dependencies only in the root `[workspace.dependencies]`, pinned, with a license compatible with
  GPL-3.0-or-later.
- Desktop app: all logic and state in Rust; QML is only the view. No hand-written C++, only the
  bridge cxx-qt generates.
- QML follows Qt 6 `qmlformat`, with no array destructuring. Every query reply checks that its owner
  is still alive (`reply => page && (page.x = reply)`).
- Do not leave `*.qml~` files behind (created by `qmlformat -i`): they break a translation test.

## Product rules

- **Every value is explicit.** No Reiwand format (scenes, packages, settings, export files) accepts
  omitted fields: the loader rejects the file and names the missing field. No `serde(default)` and
  no values hidden in code for user or author data. A "default" exists only in the editor and in
  templates, as a prefilled value; what is saved carries every value.
- **One version in the code.** The code knows only the current version of each format, stored data,
  route and API: no aliases of old routes, no fields kept for old clients, no readers of older
  formats, no two models side by side. A change that breaks something is a major version and brings
  no code that reads or converts the old one (there is no migration tool); data of another version is
  refused with a message that says so. The change that completes it ships in the same major version.
- **Everything a user can do is also in the command line.** Each user action (library, playback,
  settings, repositories, server) has its own `reiwand-ctl` subcommand and D-Bus method
  (`apps/reiwand-ctl/tests/parity.rs` checks it). Editor actions get no subcommand of their own:
  they are variants of `Action` in `crates/reiwand-editor/src/action.rs`, unit-tested in that
  crate, and are run with `reiwand-ctl scene act DIR ACTION JSON` (listed by
  `reiwand-ctl scene actions`). Dedicated subcommands exist only for create, import, preview,
  capture, check, pack, sign, publish and repository.
- **Real authoring tools.** A new feature for authors comes with its tool in the editor (on-canvas
  handles, pickers, brushes, curve or gradient editors, timelines, previews per quality level), not
  just an inspector field or hand-written JSON.
- **The phone has no editor.** Android gets user features only.
- **Keyboard.** In the app, every key the bar shows at the bottom right does exactly what it says,
  with no click first. In the editor, what a user expects: in a focused list the arrows move, Enter
  picks and Esc closes.
- **Battery and efficiency.** Zero work when nothing changes, no per-frame allocations, and what the
  user sets (quality, GPU, frames per second) is always respected.
- **Product privacy.** No telemetry, no network except what the user adds, and consent is asked at
  the moment something is used. Anything that affects privacy is shown to the user visibly, and the
  user decides.
- **A package never runs binaries of its own.**
- **Own formats only.** Reiwand does not read the formats of other wallpaper programs and does not
  name them in code, documentation or tests.
- Every feature is documented in `docs/specs` and tested: unit tests in its crate,
  `reiwand-app --check` for the app, and `scripts/integration-*.sh` where the service is involved.

## Testing without touching your own session

- Never run `reiwand-ctl` or the app against your session bus while testing: use the private
  instances the integration scripts set up. When you run `reiwand-ctl` or the service by hand, first
  `export REIWAND_PRIVATE_BUS_ONLY=1`.
- A private bus started by hand uses a configuration without the standard service directories, as
  the integration scripts do; otherwise it can start the service you have installed.
- Do not leave services, PipeWire, Xvfb or Sway running when you finish.
- Tests that sometimes fail on a loaded machine (rerun them alone before concluding):
  `reiwand-video heard.rs`, `reiwand-core stash.rs screens_of_one_package`, the inotify `listing`
  tests, `integration-streams.sh`, `integration-recovery.sh` and `reiwand-render --test timing`.

## Commits and versions

- [Conventional Commits](https://www.conventionalcommits.org/), in English, with one of these types:
  `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test`
  (optional scope, as in `feat(editor): …`). Translations go in `chore(i18n): …` or with the
  feature.
- The type decides the version (`GitVersion.yml`, strict semantic versioning):
  - `feat` → minor;
  - `fix` and `perf` → patch;
  - `!` or a `BREAKING CHANGE:` footer → major;
  - the other types publish no version.
- Major is anything that stops working as before for whoever uses it: stored data or package
  format, D-Bus, HTTP or server API, `reiwand-ctl` options or output, a promised behaviour that
  changes. Minor is a new feature that breaks nothing. Patch is only a fix that adds no feature.
- Pick the type by what changes for the user, not by size: a fix that adds an option is a `feat`,
  and a `feat` that breaks something carries `!`. When torn between two, take the larger.
- One commit per finished item. The message describes only the code change.
- Keep your own data out of the repository: no home folder paths, user or machine names, and no
  names of your GPU, CPU or monitor. Strip metadata from images, audio and other binaries before
  adding them (for example `exiftool -all= FILE`).

## Fixes

A fix is argued before any code changes, in four points (the pull request template asks for them):

1. **Why it is a defect:** which promise it breaks (a line of the specification, the documentation
   or the interface) or what fails, with the steps to see it.
2. **Why it is not a new feature:** where it is promised, or why what already exists cannot be used,
   or is unsafe, without the fix.
3. **Why it is needed:** who it happens to (author or user), in which flow, and what happens to them
   if it stays as it is.
4. **What it improves:** what that person can do or see after the fix, in a way that can be checked.

If any of the four does not hold, it is an improvement: open a proposal instead of a fix. A test
alone, a refactor, or a nicer message where the current one is already clear does not pass point 3
on its own.
