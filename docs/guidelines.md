# Reiwand guidelines

These are the principles Reiwand is built on. Every change follows them, together with the rules in
[CONTRIBUTING.md](../CONTRIBUTING.md). When a change would go against one of them, open a proposal
first instead of a pull request.

## Product

- **An open, own format and a portable core.** Rust and wgpu are shared by Linux and Android. The
  pieces that present the wallpaper, detect windows, capture audio and decode video can be swapped.
- **Secure by default.** Untrusted content runs in isolated processes. Web wallpapers have no network
  unless the user allows it. Repositories are signed. There is no telemetry.
- **Licences.** The code is GPL-3.0-or-later. The project's own assets are CC0, CC-BY or OFL. Image
  providers are used only when their terms allow it, and their credit is always visible.
- **Efficient everywhere.** Wallpapers must be light on battery and resources, because they also run
  on phones. The scene editor must work on as many machines as possible, modest ones included.
- **What the user sets wins.**
  - The quality level the user picks is applied as it is, also in scenes that are too heavy. The
    safety stop acts only in automatic mode, and the app shows a visible cost notice.
  - Any effect or setting with automatic logic gives way to the user's explicit value, over battery
    and cost; at most it shows a visible cost notice.
  - On battery the suggested frame rate is 60 fps; the user can lower it. Lower quality reduces
    resolution and effects, not frames.
- **Sharp and still beats moving and blurry.** At the lowest quality a scene is drawn sharp, even if
  that means it moves less. Effects may declare a cheaper variant for the low quality levels.
- **One way to write each value.** A format or specification admits a single spelling for every
  value: no short forms, aliases or alternative readings.
- **Repositories, not servers.**
  - The app adds only repositories, through one visible action, "Add a repository": the user pastes
    the repository's address.
  - A public repository is added without questions. A private one asks for an account when it
    connects.
  - A server can host several repositories, and the app can use repositories from several servers.
  - Trust comes from the secure connection to the server, with no fingerprints typed by hand. The app
    warns when a repository's signature changes.
  - Accounts belong to a repository, and a person can have several. Publishing asks for the target
    repository and, if no account is linked to it, asks for one right there. Adding a repository from
    a server where an account already exists offers to use that account or another one.
- **Everything lives locally.**
  - There is no automatic sync between devices. Moving settings to another device means exporting a
    file and importing it there, in the app and with `reiwand-ctl export` and `import`.
  - The export holds the added repositories, photo providers, lists, favourites, installed wallpapers
    (repository ones as a reference that is downloaded again) and the settings that make sense on
    another machine.
  - The user's own wallpapers go only if ticked. Passwords and keys go only if ticked, encrypted with a
    password chosen when exporting. Passwords never travel between devices any other way.
  - Import shows what can be brought, with ticks. It adds and never replaces; for lists with the same
    name it asks whether to merge them or keep both.
- **Presets that live on another device.** When a preset or package that is local to another device
  arrives through an import, the device shows "this preset or package is local to another device;
  publish it to have it here" instead of failing silently.
- **Licences of what an author edits.** Editing a copy of a work with a no-derivatives licence, or
  with no licence, warns the author and asks for confirmation.
- **Mature content** on a self-hosted server is labelled and blurred until the viewer confirms; it is
  not rejected. There is no age question.
- **Depth and parallax.**
  - Reading depth removes the stretching at horizons and edges between depth layers by filling what is
    uncovered instead of stretching pixels.
  - The depth model (Depth Anything V2 Small, through ONNX Runtime, in a separate process) is an
    optional download with consent per device. The filling model is optional too: on by default, and
    the user can switch it off in the app, on the phone and in `reiwand-ctl`.
  - Without the models, the neighbouring background is extended or blurred, never warped. The user may
    also supply their own depth maps. Layer parallax in scenes uses no AI.
- **Puppet physics follows the frame rate**: its step equals the frame interval, with sub-steps, not
  a fixed step per battery state.

## Look and use

- Reiwand has its own look on every surface (app, tray, phone, server pages). It does not follow the
  system colour scheme, but it does not reach into system dialogs either: the file chooser is always
  the desktop's own (the portal), as are printing and permission dialogs.
- The interface says "repository" everywhere: in the app, the phone, the command line, the
  documentation and the formats.
- The name is Reiwand and the icon is the block-built R whose loose tile is a red seal, with one
  variant for dark backgrounds and one for light ones.
- Groups of horizontal buttons are aligned and of one size, even when one has a filled background and
  the others do not. Icons are exactly aligned with their text.
- The keyboard focus ring is never covered. Hovering a wallpaper behaves the same on every page.
- Every change that touches a screen checks the alignment and size of its button rows and the whole
  focus ring.
- The reason why a wallpaper is still (covered by a full-screen window, battery, locked screen, manual
  pause) is shown in the command-line state, not in the app.
- The server's pages carry its administration. Besides the admin view, the server has a public page,
  "Explore repositories", with each repository's address to copy or open in Reiwand. "My account"
  shows everyone with an account how to connect their phone.

## Code

- The amount of code must stay small enough for one person to understand and maintain. Prefer less
  code: remove parallel paths, layers and pieces few people use, not only duplicates and dead code.
  When adding something, pick the way that leaves the least code to keep. Removing something a user
  or an author can see is proposed first, with its size.
- Changes that alter how a scene is read or drawn (`reiwand-scene`, `reiwand-render`, the scene
  format) also run the picture regression tests. A removed spelling must be refused with an error,
  and a value that reaches the same reader another way (a property, a slider, a script) must keep
  working: check who else calls what you delete.
- Testing a part with a screen includes driving it live in a private instance (quick, repeated and
  out-of-order clicks and keys while something loads) and looking at the captures; unit tests alone
  are not enough. Every break found becomes a case of `reiwand-app --check`.
- A fault that cannot be reproduced is not reported as fixed: describe what was tried and what was
  ruled out, and add a diagnostic aid (a log or status line that names the cause).
- A new `scripts/integration-*.sh` is sourced into one shell with all the others: never reuse a global
  name another script sets (`output`, `outputs`, `daemon`, `serial`, `port`, `anchor`). Loop variables
  and helpers take a prefix from the script's topic.
- Authoring tools that exist are generalised and reused, not rebuilt: the clip timeline
  (`crates/reiwand-editor/src/clips`), pose handles, the weights brush with its heat view, and the
  bone and piece pickers of the inspector.

## Privacy in features

- What affects privacy is neither blocked in silence nor imposed. It is configured per wallpaper or
  per user, with a working default and a visible notice of what is exposed, and the same notice in
  the app, on the phone and in the command line.
- Web wallpapers:
  - They get the real time zone, language, WebGL and storage by default, with a notice and an option
    to restrict each one.
  - Consent is explicit at install, with the hosts listed.
  - Cursor plus network needs a double confirmation.
  - The field is `permissions.network`.
- Scheduling by location rounds the coordinates to 2 decimals when saving them (about 1 km).
- The service logs presence, file names and GPU only at debug level. Its systemd unit uses
  `UMask=0077`, `LimitCORE=0`, `NoNewPrivileges` and filesystem protections.
- Android: `android:allowBackup="false"`, and data extraction rules that exclude tokens, keys and the
  library. The token is wrapped with a non-exportable Android Keystore key.
- Downloads of models check a pinned SHA-256 and use a generic User-Agent.
- Provider credentials and author signing keys are stored in files only the user can read (mode
  0600), with no system keyring.

## Server

- It is easy to host with Docker:
  - one `docker compose up -d`;
  - configuration by environment variables with sensible suggested values;
  - a healthcheck and data in a volume;
  - a guided first admin account and simple backups;
  - an example behind a reverse proxy with automatic TLS.
- The image is published as `ghcr.io/reiwand/reiwand-server`. A local build from the repository
  (`deploy/compose.build.yaml`) stays available.
- The server checks the version of its data. If the data is from another version, it refuses to
  start and says so; nothing migrates it.

## Phone

- The phone has user features only: play, choose and tune wallpapers, install, rotate, account and
  permissions. It has no editor, no authoring tools, and no publishing or signing of repositories.
- It supports Android 12 and newer (`minSdk` no higher than 31). The app id is `io.github.reiwand`.
- It has parallax by gyroscope, Material You colours, video straight to the surface, partial audio,
  the same empty state and the same three settings blocks as the desktop. It shows the same privacy
  notices as the desktop and plays an author's own transitions.

## Editor scope

- The editor is functionally complete. A new authoring tool is added only when an author cannot do a
  function at all today. A missing convenience (one more handle, a cursor, a preview) in a function
  that can already be done is not a defect.
- What always counts: a click answers at once, previews are sharp, panels can be read, resized and
  moved like in an IDE, and right-click menus offer the actions that already exist.

## Releases

- CI publishes the releases on GitHub. A release happens only when the version rises; tags are
  immutable and named `v<SemVer>`. The Android APK is part of every release.
- The Flatpak is built and published by CI alongside the other packages.
