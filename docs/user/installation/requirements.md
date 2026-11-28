# Requirements

What the system needs for Reiwand to run, and which parts come from the system and which come with
Reiwand. With the Flatpak nothing more is needed.

## What the system needs

| Requirement | Details |
|-------------|---------|
| glibc 2.39 or later | For the release binaries. They are built on Ubuntu 24.04 and do not run on Debian 12, which ships 2.36. A local build needs the glibc of the system where it was made. The install warns if the one on this system is older. |
| KDE Plasma 6.4 or later | |
| PipeWire | |
| GStreamer 1.24 or later | With the base and good plugins and the PipeWire one (see below). |
| fontconfig, libepoxy, libva | |
| The OpenGL libraries of libglvnd | `libGL`, `libEGL`, `libOpenGL` and `libGLESv2`; on Debian and Ubuntu `libgl1`, `libegl1`, `libopengl0` and `libgles2`. |
| A Vulkan or OpenGL driver | |

The install warns if one of those libraries is missing.

### GStreamer and sound

The GStreamer PipeWire plugin carries the sound of videos and scenes since its version 1.2.

- Without it, or with an older one, the sound goes to the default output.
- If there is no sound output yet, the wallpaper sounds as soon as one appears.

The install warns if it does not find GStreamer 1.24 or later, and the RPM package requires it.

> **Note:** GStreamer and PipeWire always come from the system, because their plugins and the sound
> server have to match it.

## Plasma parts: the desktop's Qt

The wallpaper plugin and the tray icon run inside Plasma with the desktop's own Qt. They use the
QML modules of Kirigami, KPipeWire, KWindowSystem and plasma-workspace.

On Debian and Ubuntu they come in:

- `qml6-module-org-kde-kirigami`
- `qml6-module-org-kde-pipewire`
- `qml6-module-org-kde-kwindowsystem`
- `plasma-workspace`

## The app and the web helper: their own Qt

The desktop app and the web helper do not use the system's Qt. The AppImage and the RPM carry:

- Qt 6.12 with Qt Quick and Qt WebEngine;
- Kirigami 6.32;
- KPipeWire 6.8 (for the live previews of a screen in the app).

So the app and the web wallpapers also work where the system Qt is older (such as Ubuntu 24.04,
with Qt 6.4), and nothing has to be installed for them.

For those previews the app uses PipeWire, libepoxy and VA-API (`libva`) of the system.

The app draws its controls with a style of its own, the same everywhere.
