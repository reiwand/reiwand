# Instalación con Flatpak

La misma página *Releases* trae `reiwand.flatpak` (x86_64) y `reiwand-aarch64.flatpak`
(ARM de 64 bits), con la app de escritorio, el daemon, `reiwand-ctl`, el plugin de fondo y el icono de
la bandeja de Plasma. El runtime (`org.kde.Platform` 6.11, con Qt 6, Kirigami, el estilo de
escritorio de KDE y GStreamer 1.26 con sus códecs; el Flatpak no se construye sobre uno con
GStreamer anterior a 1.24) se descarga de Flathub al instalarlo, así que el repositorio
`flathub` tiene que estar añadido; la primera orden lo añade si falta:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user ./reiwand.flatpak
flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand | sh
```

Flatpak no puede instalar el plugin de fondo ni el icono de la bandeja de Plasma: la última orden
los instala con `kpackagetool6` desde los archivos del propio Flatpak y crea la carpeta en la que
Flatpak monta las ediciones compartidas, y deja `~/.local/share/reiwand/edits/`, que
Flatpak crea legible por todos, solo para tu usuario (sin `| sh` solo muestra las órdenes). Hay que repetirla después de cada actualización y reiniciar Plasma. El plugin arranca el
daemon por D-Bus cuando lo necesita; no hay servicio de systemd. En ARM se instala
`./reiwand-aarch64.flatpak`. No instales a la vez el Flatpak y el `.tar.gz`, porque los dos
registran los mismos servicios D-Bus: ejecuta antes `./uninstall.sh`.

La app sale en el menú de aplicaciones (`flatpak run io.github.reiwand.Reiwand` la abre
desde la terminal) y el icono de la bandeja la abre con *Abrir reiwand* y *Ajustes…*, como en
la instalación sin Flatpak. `reiwand-ctl` también está dentro:

```sh
alias reiwand-ctl='flatpak run --command=reiwand-ctl io.github.reiwand.Reiwand'
reiwand-ctl --help
```

Con el alias, `reiwand-ctl` solo ve las carpetas que ve el Flatpak (Imágenes, Vídeos, Descargas y las
que añadas con `flatpak override`). Las rutas relativas valen cuando la carpeta actual es una de
ellas o está dentro; desde cualquier otra, `flatpak run` arranca en la carpeta personal y la ruta
relativa apunta a otro sitio, así que conviene usar la ruta completa: `reiwand-ctl set
~/Imágenes/fondo.jpg` en lugar de `reiwand-ctl set fondo.jpg`.

El Flatpak se construye sobre `io.qt.qtwebengine.BaseApp` (rama 6.11), que trae Qt WebEngine
fuera del runtime de KDE, y por eso muestra los fondos web (unos 600 MB más al instalar). Lo único
que pide es `--share=ipc`, que ya tiene. El ayudante comprueba si puede crear los espacios de nombres
de usuario, de procesos y de red; si el Flatpak no lo permite, arranca Qt WebEngine con su sandbox
apagado (`QTWEBENGINE_DISABLE_SANDBOX`) y `reiwand-ctl state` muestra `reduced`: entonces la
página queda aislada solo por el propio Flatpak.

Permisos: la GPU, el socket de PipeWire, que publica los streams, lleva el sonido de los vídeos y de
las escenas y permite capturar el sonido solo para las escenas con permiso (no hace falta el de
PulseAudio), el socket de Wayland, para las ventanas de la app y, en el daemon, solo para leer la
resolución de cada pantalla (el daemon no abre ventanas), X11 solo en una sesión X11 y para la app,
los nombres D-Bus de la app y del daemon, el bloqueo de pantalla y la batería, el llavero del sistema (Secret
Service) para las contraseñas y claves de los proveedores, KWin para cargar el script del puntero
solo mientras una escena con permiso o un paralaje que sigue al puntero lo usa, las notificaciones
del escritorio, solo para preguntarte antes de dar a una escena el sonido o el puntero (dentro del
Flatpak el daemon no puede ver qué programa se lo pide, así que siempre pregunta así: la app, la
bandeja y `reiwand-ctl input grant` avisan de que la pregunta está en la notificación, también al tocar «Permitir» en la pregunta de una escena, y la posición del puntero
solo la acepta del script de KWin), lectura de
Imágenes, Vídeos y Descargas, escritura solo en `~/.local/share/reiwand/edits/`, y red. La red
solo se usa para los proveedores de fondos, los repositorios y el servidor propio que añadas tú y
para el modelo de profundidad y ONNX Runtime cuando los pides. Los cambios de zona horaria llegan
sin permisos, por el fichero que Flatpak actualiza en el sandbox. Para usar fondos de otra carpeta:
`flatpak override --user --filesystem=/ruta:ro io.github.reiwand.Reiwand`.

Rutas, con `~/.var/app/io.github.reiwand.Reiwand` abreviado como `APP`: la configuración
vive en `APP/config/reiwand/`, la biblioteca, las escenas propias y los mapas de profundidad en
`APP/data/reiwand/` y la caché en `APP/cache/reiwand/`. Las ediciones de las imágenes que
el Flatpak solo puede leer (todas las de Imágenes, Vídeos y Descargas) no caben junto a la imagen y
van a `~/.local/share/reiwand/edits/`, la misma carpeta que usa un `reiwand-ctl` de fuera del
Flatpak: el permiso de escritura sobre ella hace que Flatpak la monte también en
`APP/data/reiwand/edits/` en cuanto esa carpeta existe (la crea `reiwand-plasma`), así que el daemon, la app y cualquier `reiwand-ctl` ven las mismas
ediciones. Las que guardó una versión del Flatpak sin ese permiso se quedan tapadas en
`APP/data/reiwand/edits/` (solo dentro del Flatpak; fuera se ven) y se recuperan con
`mv -n APP/data/reiwand/edits/*.json ~/.local/share/reiwand/edits/`. Para desinstalar:
`flatpak run --command=reiwand-plasma io.github.reiwand.Reiwand remove | sh` y
`flatpak uninstall --user io.github.reiwand.Reiwand`.
