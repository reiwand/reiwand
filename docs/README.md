# Documentation

Reiwand shows live wallpapers on Linux desktops and Android phones, and lets you make your own.
Start with [Get started](user/README.md), or open a section below. Each section has sub-levels;
the side navigation of the site shows the same tree.

## Get started

- [`user/README.md`](user/README.md): the app in five minutes: choosing a wallpaper, one per screen, pausing, the tray icon.

## Using Reiwand

### Desktop app

- [`user/app.md`](user/app.md): the desktop app, screen by screen.
- [`user/other-desktops.md`](user/other-desktops.md): Sway, Hyprland, river, niri, COSMIC and X11 sessions.
- [`user/keyboard-switching.md`](user/keyboard-switching.md): stepping through wallpapers with a shortcut.
- [`user/packages.md`](user/packages.md): rating, languages and credits of packages.
- [`user/web-wallpapers.md`](user/web-wallpapers.md): web wallpapers and what they allow.
- [`user/rgb-lighting.md`](user/rgb-lighting.md): RGB lighting.
- [`user/player.md`](user/player.md): the media player.
- [`user/privacy.md`](user/privacy.md): what the program does with the network and with your data.

### Phone

- [`user/phone.md`](user/phone.md): the phone next to the desktop.
- [`user/android.md`](user/android.md): the Android app.

### Command line

- [`user/command-line.md`](user/command-line.md): using it from the terminal with `reiwand-ctl`.

### Installation

- [`user/installation.md`](user/installation.md): installation without root, verifying downloads and versions.
- [`user/flatpak.md`](user/flatpak.md): installation with Flatpak.

## Your own server

- [`user/server.md`](user/server.md): install, repositories, accounts and moving the configuration.
- [`specs/server.md`](specs/server.md): server specification.
- [`specs/repo.md`](specs/repo.md): signed repositories.

## Creating wallpapers

### First scene

- [`authoring/README.md`](authoring/README.md): making a scene from scratch.
- [`authoring/start/first-scene.md`](authoring/start/first-scene.md): your first scene, step by step.
- [`authoring/scene.md`](authoring/scene.md): the scene file.

### Layers

- [`authoring/layers.md`](authoring/layers.md): every kind of layer.
- [`authoring/properties.md`](authoring/properties.md): properties the user can change.
- [`authoring/meshes.md`](authoring/meshes.md): meshes.
- [`authoring/3d.md`](authoring/3d.md): 3D scenes.
- [`authoring/transitions.md`](authoring/transitions.md): transitions.

### Effects

- [`authoring/effects.md`](authoring/effects.md): effects and own shaders.

### Particles and sound

- [`authoring/particles-and-sound.md`](authoring/particles-and-sound.md): particles, lights and sound layers.

### Scripts

- [`authoring/scripts-guide.md`](authoring/scripts-guide.md): a guide to scripts.
- [`authoring/scripts.md`](authoring/scripts.md): scripts.

### Web

- [`authoring/web-wallpapers.md`](authoring/web-wallpapers.md): creating a web wallpaper.

### Publishing

- [`authoring/package.md`](authoring/package.md): from the folder to a package that others install.
- [`authoring/cli.md`](authoring/cli.md): the creation commands of `reiwand-ctl`.
- [`authoring/provider-plugins.md`](authoring/provider-plugins.md): provider plugins.

### The editor

- [`authoring/editor.md`](authoring/editor.md): the scene editor.
- [`authoring/editor-features.md`](authoring/editor-features.md): what the editor can do.
- [`authoring/other-editor-packages.md`](authoring/other-editor-packages.md): other packages in the editor.
- [`authoring/editor-canvas.md`](authoring/editor-canvas.md): the canvas.
- [`authoring/editor-drawing.md`](authoring/editor-drawing.md): drawing.
- [`authoring/editor-effects.md`](authoring/editor-effects.md): effects.
- [`authoring/editor-animation.md`](authoring/editor-animation.md): animation.
- [`authoring/editor-particles-and-sound.md`](authoring/editor-particles-and-sound.md): particles and sound.
- [`authoring/editor-meshes.md`](authoring/editor-meshes.md): meshes.
- [`authoring/editor-3d.md`](authoring/editor-3d.md): 3D.
- [`authoring/editor-properties.md`](authoring/editor-properties.md): properties.

### Editor actions from the terminal

- [`authoring/actions.md`](authoring/actions.md): index of the actions.
- [`authoring/actions-layers.md`](authoring/actions-layers.md): layers.
- [`authoring/actions-effects.md`](authoring/actions-effects.md): effects.
- [`authoring/actions-meshes.md`](authoring/actions-meshes.md): meshes.
- [`authoring/actions-particles-and-lights.md`](authoring/actions-particles-and-lights.md): particles and lights.
- [`authoring/actions-3d.md`](authoring/actions-3d.md): 3D.
- [`authoring/actions-time-and-package.md`](authoring/actions-time-and-package.md): time and package.

### Examples

- [`authoring/coverage.md`](authoring/coverage.md): what each example shows.
- [`authoring/examples/ORIGIN.md`](authoring/examples/ORIGIN.md): origin of the example files.
- [`authoring/examples/`](authoring/examples): the example scenes.

## Reference

### Formats and interfaces

- [`specs/scene.md`](specs/scene.md): the scene format.
- [`specs/package.md`](specs/package.md): packages.
- [`specs/web.md`](specs/web.md): web wallpapers.
- [`specs/editor.md`](specs/editor.md): the editor.
- [`specs/dbus.md`](specs/dbus.md): the D-Bus interface.
- [`specs/export.md`](specs/export.md): export and import file.
- [`specs/explicit-values.md`](specs/explicit-values.md): explicit values.
- [`specs/provider-plugins.md`](specs/provider-plugins.md): provider plugins.

### The app and its screens

- [`specs/library.md`](specs/library.md): the library.
- [`specs/create.md`](specs/create.md): creating.
- [`specs/shell.md`](specs/shell.md): the shell.
- [`specs/look.md`](specs/look.md): the look.
- [`specs/palette.md`](specs/palette.md): the palette.
- [`specs/icon.md`](specs/icon.md): the icon.
- [`specs/settings-pieces.md`](specs/settings-pieces.md): settings pieces.
- [`specs/choose-pieces.md`](specs/choose-pieces.md): choose pieces.
- [`specs/phone-daily-use.md`](specs/phone-daily-use.md): the phone in daily use.

## Development

- [`development/development.md`](development/development.md): building, testing, the CI and how the program works.
- [`development/design.md`](development/design.md): the design guide.
- [`development/qml.md`](development/qml.md): known QML pitfalls.
- [`development/regression-net.md`](development/regression-net.md): the regression net.
- [`guidelines.md`](guidelines.md): the project guidelines.
