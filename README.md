# Reiwand

Live wallpapers for Linux and Android, free and with no account needed: images, videos and
scenes, made by you or found in repositories, a different one per screen, paused whenever nobody
can see them.

A daemon draws the wallpaper of each screen on the GPU and KDE Plasma 6 shows it through a
wallpaper plugin; on Sway, Hyprland, river, niri, COSMIC and X11 sessions the daemon draws it
itself. Everything is managed from the desktop app, from the terminal (`reiwand-ctl`) or from the
system tray icon.

## What it can do

- Images, looping videos, animated scenes, web wallpapers and 3D models as wallpaper, with transitions.
- A different wallpaper per screen, with lists that change on their own or with a keyboard shortcut.
- A wallpaper or list of its own for the lock screen, for every screen or one per screen.
- Automatic pause when a window covers the wallpaper, on battery or with the screen locked.
- Your own `.wpak` packages, with repositories, signatures and an optional server of your own.
- Built-in scene editor, to create wallpapers with layers, effects, particles, sound and scripts.
- RGB lighting, a media player and an Android app to take your wallpapers to your phone.

## Install

Every version is on the [*Releases* page](https://github.com/reiwand/reiwand/releases), with signed
checksums ([how to check them](docs/user/installation.md)).

### AppImage (x86_64)

Download `reiwand-x86_64.AppImage` and run:

```sh
chmod +x reiwand-x86_64.AppImage
./reiwand-x86_64.AppImage --install
```

It installs in your home directory, without root, and once a day installs a newer signed version
(`reiwand-ctl update auto off` stops it). To uninstall: `reiwand --uninstall`.

### Flatpak (x86_64 and ARM)

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak remote-add --user --if-not-exists reiwand https://reiwand.github.io/reiwand/flatpak/reiwand.flatpakrepo
flatpak install --user reiwand io.github.reiwand.Reiwand
flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand | sh
```

`flatpak update` brings each new version. More in [`docs/user/flatpak.md`](docs/user/flatpak.md).

### Android (12 or later, 64-bit)

Download `reiwand-android-arm64-v8a.apk` (or `reiwand-android.apk` if unsure) on the phone and open
it, or from a computer: `adb install -r reiwand-android-arm64-v8a.apk`. More in
[`docs/user/android.md`](docs/user/android.md).

The other desktops (Sway, Hyprland, river, niri, COSMIC, X11) and the RPM are covered in the pages
of the table below.

## Getting started

1. Open *reiwand* from the application launcher, or from the system tray icon.
2. Pick a wallpaper from the library (or add an image, a video or a package) and set it.
3. On Plasma, the wallpaper is shown through the plugin; the steps are on the installation page.
4. From the terminal, `reiwand-ctl --help` lists what can be done.

## Documentation

Guides for using Reiwand, making wallpapers and the reference specs: <https://reiwand.github.io/reiwand/docs/>.

## Privacy

No telemetry and no usage statistics. The program only connects to the internet for the wallpaper
providers, the repositories and the server of your own that you add, and, in the AppImage and the
phone app, for a daily check of the latest release on GitHub that you can turn off. Private mode
(`reiwand-ctl private on`) cuts the whole network at once. The details are in
[`docs/user/privacy.md`](docs/user/privacy.md).

## Contributing and security

How to propose changes, and the rules the code follows: [`CONTRIBUTING.md`](CONTRIBUTING.md) and
[`docs/guidelines.md`](docs/guidelines.md). To report a security problem privately:
[`SECURITY.md`](SECURITY.md).

## License

The program is free software under the GPL-3.0-or-later ([`LICENSE`](LICENSE)). What it carries
keeps its own licence:

| What | Licence |
| --- | --- |
| The example scenes in `scenes/` and the examples in `docs/authoring/examples/` | CC0 1.0 ([`scenes/LICENSE`](scenes/LICENSE), [`docs/authoring/examples/LICENSE`](docs/authoring/examples/LICENSE)) |
| The fonts Onest and Red Hat Display | SIL Open Font License 1.1 ([`apps/reiwand-app/look/fonts`](apps/reiwand-app/look/fonts)) |
| The Phosphor icons | MIT ([`apps/reiwand-app/look/icons/LICENSE-phosphor.txt`](apps/reiwand-app/look/icons/LICENSE-phosphor.txt)) |
| The shader translator in `vendor/naga` | MIT or Apache-2.0 ([`vendor/naga`](vendor/naga)) |
| The Rust and Android libraries | Each its own, listed in [`packaging/THIRD-PARTY-NOTICES`](packaging/THIRD-PARTY-NOTICES) and by `reiwand-ctl licenses` |

The models are not included: they are downloaded when you ask for them and each one is saved with
the text of its license. Depth Anything V2 Small (Apache-2.0), LaMa big-lama (Apache-2.0, the file
`lama_fp32.onnx` of the ONNX export published as `Carve/LaMa-ONNX` on huggingface.co, 208,044,816
bytes, SHA-256 `1faef530…68d6`, Apache-2.0 license declared on that page and in
[`advimman/lama`](https://github.com/advimman/lama/blob/main/LICENSE)) and ONNX Runtime (MIT).
`reiwand-ctl models list` and the settings show size, license and servers before each download.
