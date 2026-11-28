# Icon

The Reiwand icon is an R made of blocks with one loose tile in red (the seal). It is drawn in a
single dark-background version (`io.github.reiwand`) that reads the same in light and dark menus.

- Desktop: `packaging/icons/io.github.reiwand.svg` (scalable) and
  `packaging/icons/hicolor/NxN/apps/io.github.reiwand.png` (16, 22, 24, 32, 48, 64, 128, 256 and 512
  px). At 16, 22 and 24 px the R is solid and the seal is a plain square, drawn by hand; they are
  not scaled from the SVG. They are installed by the AppImage (`--install`), the RPM and the
  Flatpak; `reiwand --uninstall` removes the ones of the AppImage. The `.desktop` already points to
  `Icon=io.github.reiwand`.
- Plasma tray: `packaging/icons/io.github.reiwand-symbolic.svg`, single color, which Plasma tints
  according to the theme, and `io.github.reiwand-paused-symbolic.svg` (the same symbol with a pause
  mark) while the wallpaper is paused. The Plasma wallpaper plugin uses `io.github.reiwand`. The
  Flatpak does not carry it because it does not install the tray.
- App window: the icon goes inside the executable (`apps/reiwand-app/look/pictures/icon.svg`) and is
  set with `QGuiApplication::setWindowIcon`, for the title bar and X11; on Wayland Plasma also takes
  the one from `Icon=` of the `.desktop`.
- Server web: `apps/reiwand-server/assets/icon.svg`, served at `/static/icon.svg` and linked as a
  favicon from each page (the content policy already allows own images).
- Phone: adaptive icon (`android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml`) with a dark
  background, foreground of blocks with the red seal (`drawable/ic_launcher_foreground.xml`) and a
  monochrome layer for themed icons on Android 13 or later (`drawable/ic_launcher_monochrome.xml`).
  The wallpaper service thumbnail uses the same icon.
