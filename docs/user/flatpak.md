# Installation with Flatpak

The same *Releases* page has `reiwand.flatpak` (x86_64) and `reiwand-aarch64.flatpak` (64-bit
ARM), with the desktop app, the daemon, `reiwand-ctl`, the wallpaper plugin and the Plasma tray
icon. The runtime (`org.kde.Platform` 6.11, with Qt 6, Kirigami, the KDE desktop style and
GStreamer 1.26 with its codecs; the Flatpak is not built on one with GStreamer older than 1.24)
is downloaded from Flathub on install, so the `flathub` repository has to be added; the first
command adds it if missing:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user ./reiwand.flatpak
flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand | sh
```

Reiwand also has its own Flatpak repository, signed with its own key and published with every
release at `https://reiwand.github.io/reiwand/flatpak/`. Installing from it instead of the file
works on x86_64 and ARM alike, and `flatpak update` then brings each new version as with any other
remote:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak remote-add --user --if-not-exists reiwand https://reiwand.github.io/reiwand/flatpak/reiwand.flatpakrepo
flatpak install --user reiwand io.github.reiwand.Reiwand
flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand | sh
```

The bundles of the *Releases* page carry the address and the key of that repository too: a bundle
installed with `flatpak install ./reiwand.flatpak` adds the remote `reiwand-origin` and updates
from it with `flatpak update` (bundles of earlier versions do not; installing a new one over them
is enough). The repository keeps only the latest version; going back to an older one is done with
its bundle from the *Releases* page. The key's fingerprint is
`9C23 C7EF 7478 9850 1E4D 8EBF 0C9F 3D23 2716 55A8`, and its public part is
`packaging/flatpak/repo-key.asc` in the source code.

Flatpak cannot install the wallpaper plugin or the Plasma tray icon: the last command installs
them with `kpackagetool6` from the files of the Flatpak itself and creates the folder where
Flatpak mounts the shared edits, and leaves `~/.local/share/reiwand/edits/`, which Flatpak
creates readable by everyone, for your user only (without `| sh` it only shows the commands). It
has to be repeated after each update, and Plasma restarted: after a `flatpak update` (or an update in
Discover) the tray says *A new version of Reiwand is installed* and *Restart Reiwand* does all of it
at once (it copies the new plugin and tray, stops the old service and restarts Plasma). The plugin starts the daemon over
D-Bus when it needs it; there is no systemd service. On ARM, install `./reiwand-aarch64.flatpak`.
Do not install the Flatpak and the AppImage at the same time, because both register the same
D-Bus services: run `reiwand --uninstall` first.

The app shows up in the applications menu (`flatpak run io.github.reiwand.Reiwand` opens it from
the terminal) and the tray icon opens it with *Open Reiwand* and *Settings…*, as in the
installation without Flatpak.

The service runs with no window, so the desktop lists it among the apps running in the
background: in Plasma, a second Reiwand entry in the tray that says *Renders your wallpapers*.
Every sandboxed app without a window is listed there and it cannot be left out; *Quit Reiwand* in
it stops the service, and the wallpaper with it. To hide the entry, open the tray's *Entries*
settings and set it to *Disabled*. `reiwand-ctl` is inside too:

```sh
alias reiwand-ctl='flatpak run --command=reiwand-ctl io.github.reiwand.Reiwand'
reiwand-ctl --help
```

With the alias, `reiwand-ctl` only sees the folders the Flatpak sees (Pictures, Videos, Downloads
and those you add with `flatpak override`). Relative paths work when the current folder is one of
them or is inside one; from any other, `flatpak run` starts in the home folder and the relative
path points somewhere else, so it is best to use the full path: `reiwand-ctl set
~/Pictures/wallpaper.jpg` instead of `reiwand-ctl set wallpaper.jpg`.

The Flatpak is built on `io.qt.qtwebengine.BaseApp` (branch 6.11), which brings Qt WebEngine
outside the KDE runtime, and for that reason it shows web wallpapers (about 600 MB more on
install). The only thing it asks for is `--share=ipc`, which it already has. The helper checks
whether it can create user, process and network namespaces; if the Flatpak does not allow it, it
starts Qt WebEngine with its sandbox off (`QTWEBENGINE_DISABLE_SANDBOX`) and `reiwand-ctl state`
shows `reduced`: then the page is isolated only by the Flatpak itself.

Permissions: the GPU, the PipeWire socket, which publishes the streams, carries the sound of
videos and scenes and allows capturing sound only for scenes with permission (the PulseAudio one
is not needed), the Wayland socket, for the windows of the app and, in the daemon, only to read
the resolution of each screen (the daemon opens no windows), X11 only in an X11 session and for
the app, the D-Bus names of the app and the daemon, screen locking and battery, the system
keyring (Secret Service) for the passwords and keys of the providers, KWin to load the pointer
script only while a scene with permission or a parallax that follows the pointer uses it, desktop
notifications, only to ask you before giving a scene the sound or the pointer (inside the Flatpak
the daemon cannot see which program asks, so it always asks this way: the app, the tray and
`reiwand-ctl input grant` say that the question is in the notification, also when tapping "Allow"
on a scene's question, and the pointer position is only accepted from the KWin script), read
access to Pictures, Videos and Downloads, write access only to `~/.local/share/reiwand/edits/`,
and network. The network is only used for the wallpaper providers, the repositories and the server
of your own that you add, and for the depth model and ONNX Runtime when you ask for them. Time
zone changes arrive without permissions, through the file that Flatpak updates in the sandbox. To
use wallpapers from another folder:
`flatpak override --user --filesystem=/path:ro io.github.reiwand.Reiwand`.

Paths, with `~/.var/app/io.github.reiwand.Reiwand` abbreviated as `APP`: the configuration lives
in `APP/config/reiwand/`, the library, your own scenes and the depth maps in
`APP/data/reiwand/` and the cache in `APP/cache/reiwand/`. The edits of images that the Flatpak
can only read (all those in Pictures, Videos and Downloads) do not fit next to the image and go to
`~/.local/share/reiwand/edits/`, the same folder that a `reiwand-ctl` outside the Flatpak uses:
the write permission on it makes Flatpak mount it also at `APP/data/reiwand/edits/` as soon as
that folder exists (`reiwand-plasma` creates it), so the daemon, the app and any `reiwand-ctl` see
the same edits. Those saved by a version of the Flatpak without that permission stay hidden in
`APP/data/reiwand/edits/` (only inside the Flatpak; outside they are visible) and are recovered
with `mv -n APP/data/reiwand/edits/*.json ~/.local/share/reiwand/edits/`. To uninstall:
`flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand remove | sh` and
`flatpak uninstall --user io.github.reiwand.Reiwand`.
