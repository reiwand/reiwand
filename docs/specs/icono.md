# Icono

El icono de Reiwand es una R hecha de bloques con una tesela suelta en rojo (el sello). Se dibuja en una sola versión de fondo oscuro (`io.github.reiwand`) que se lee igual en menús claros y oscuros.

- Escritorio: `packaging/icons/io.github.reiwand.svg` (escalable) y `packaging/icons/hicolor/NxN/apps/io.github.reiwand.png` (16, 22, 24, 32, 48, 64, 128, 256 y 512 px). A 16, 22 y 24 px la R va maciza y el sello es un cuadrado liso, dibujados a mano; no se escalan desde el SVG. Los instala `packaging/install.sh`, el RPM, el Flatpak y el tarball; `uninstall.sh` los quita. El `.desktop` ya apunta a `Icon=io.github.reiwand`.
- Bandeja de Plasma: `packaging/icons/io.github.reiwand-symbolic.svg`, de un solo color, que Plasma tiñe según el tema, y `io.github.reiwand-paused-symbolic.svg` (el mismo símbolo con una marca de pausa) mientras el fondo está en pausa. El complemento de fondos de Plasma usa `io.github.reiwand`. El Flatpak no lo lleva porque no instala la bandeja.
- Ventana de la app: el icono va dentro del ejecutable (`apps/reiwand-app/look/pictures/icon.svg`) y se pone con `QGuiApplication::setWindowIcon`, para la barra de título y X11; en Wayland Plasma además toma el de `Icon=` del `.desktop`.
- Web del servidor: `apps/reiwand-server/assets/icon.svg`, servido en `/static/icon.svg` y enlazado como favicon desde cada página (la política de contenido ya permite imágenes propias).
- Teléfono: icono adaptable (`android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml`) con fondo oscuro, frente de bloques con el sello rojo (`drawable/ic_launcher_foreground.xml`) y capa monocroma para iconos con tema de Android 13 o superior (`drawable/ic_launcher_monochrome.xml`). La miniatura del servicio de fondos usa el mismo icono.
