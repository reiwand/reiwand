# Desarrollo

Dependencias de compilación (Debian/Ubuntu, con GStreamer 1.24 o posterior, como en Ubuntu 24.04):
`libpipewire-0.3-dev libclang-dev pkg-config libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
qt6-base-dev qt6-declarative-dev`. Para las
pruebas: `mesa-vulkan-drivers pipewire wireplumber dbus-x11 gnome-keyring gstreamer1.0-plugins-base
gstreamer1.0-plugins-good gstreamer1.0-pipewire gstreamer1.0-tools`, `sway grim foot` para la de
Sway y `xvfb x11-utils x11-xserver-utils xdotool imagemagick openbox` para la de X11.

```sh
cargo test --workspace                   # pruebas unitarias, de render y de vídeo
scripts/private-audio.sh cargo test --workspace  # igual, con un PipeWire propio y una salida muda
cargo build -p reiwand-pipewire --example probe
cargo build -p reiwandd -p reiwand-ctl && cargo build -p reiwandd --example timedate
cp target/debug/examples/probe target/debug/
scripts/integration-test.sh target/debug # D-Bus + PipeWire + daemon de punta a punta
scripts/regression.sh target/debug       # red de regresión: imágenes, contadores por cuadro y coste en reposo (docs/desarrollo/red-de-regresion.md)
scripts/sway-test.sh target/debug        # superficies layer-shell en un Sway sin pantalla
scripts/x11-test.sh target/debug         # ventanas de escritorio en un Xvfb
cargo build -p reiwand-server && cargo build -p reiwand-android --example phone
cp target/debug/examples/phone target/debug/
scripts/package.sh                       # genera dist/reiwand-*.tar.gz
scripts/translations.sh update [idioma]  # actualiza po/ con los textos del QML, del validador, del editor y de la ayuda de reiwand-ctl
target/debug/reiwand-app --check            # carga todo el QML de la app y falla ante cualquier aviso
```

`reiwand-app --check` recorre cada página con datos de prueba, abre los detalles de un elemento, el
editor de imágenes y el de escenas, y falla ante cualquier aviso del QML o si no termina en 240 s.
En el editor de escenas abre y usa el diálogo de la curva, el menú de líneas de tiempo con su panel,
las herramientas de máscaras, partículas y fuerzas; en una carpeta temporal crea un repositorio
estático con claves de prueba desde la pestaña *Repositorio estático* (diálogos *Repositorio
nuevo…*, *Raíz…*, *Revocar* y *Publicar…*), añade un paquete, revoca y retira la revocación de una
versión, publica y comprueba que la página se dibuja.
Además recorre cada página solo con el teclado (Tab, Intro, espacio y Escape) y falla si un control
no se alcanza con Tab, si no se pueden elegir ni aplicar los elementos de la biblioteca, si el menú
lateral no se abre con Intro o no se cierra con Escape, o si la rueda del ratón cambia un
desplegable o un deslizador que no se ha pulsado.
Los archivos QML siguen el formato de `qmlformat` de Qt 6.12.

APK de Android (con el SDK, el NDK 28 o posterior y `cargo install cargo-ndk`):

```sh
rustup target add aarch64-linux-android x86_64-linux-android
ANDROID_NDK_HOME=/ruta/al/ndk android/gradlew -p android assembleDebug
adb install -r -d android/app/build/outputs/apk/debug/app-arm64-v8a-debug.apk
android/gradlew -p android assembleRelease -PtestSuffix=perf  # se instala junto a la app normal como «Reiwand perf», con el manifiesto profileable para medir con perfetto y simpleperf
android/gradlew -p android connectedDebugAndroidTest -Pabis=x86_64  # pruebas en un emulador x86_64
scripts/android-power.sh -o consumo.csv  # consumo, fps y memoria por escenario en el teléfono
ANDROID_NDK_HOME=/ruta/al/ndk scripts/apk-check.sh android/app/build/outputs/apk/release/app-universal-release.apk
WANT="arm64-v8a " scripts/apk-check.sh android/app/build/outputs/apk/release/app-arm64-v8a-release.apk
```

Gradle hace un APK por arquitectura (`app-arm64-v8a-…`, `app-x86_64-…`) y, con más de una, el
universal (`app-universal-…`); el de release pasa por R8, que quita el código de las bibliotecas
que no se usa y deja entero el de la app, que Rust llama por JNI. `scripts/apk-check.sh`, que la
CI pasa a los tres APK publicados, comprueba que cada uno trae justo las arquitecturas que debe con
su biblioteca de Rust, que no trae ONNX Runtime, que todas las bibliotecas están alineadas a
páginas de 16 KB, que el APK lo está para `zipalign` y que la biblioteca de Rust no guarda rutas
del ordenador que la compiló.

`scripts/android-power.sh` recorre por adb los escenarios de consumo (imagen fija, desplazamiento,
fondo tapado, pantalla apagada, ahorro de energía, temperatura alta, paralaje con el teléfono quieto
y vídeo, 30 minutos y 3 veces cada uno) y añade una fila por medida al CSV. Con `trace_processor` de Perfetto en el `PATH` usa los
raíles de energía del teléfono; si no, la estimación de batterystats. Conviene conectar adb por
Wi-Fi para que el cable no cargue el teléfono durante la medida.

La CI firma el APK con la primera clave de un almacén PKCS12 guardado en base64 en
el secreto `ANDROID_KEYSTORE`, con su contraseña en `ANDROID_KEYSTORE_PASSWORD`; sin ellos usa una
clave temporal y lo avisa. Se crea una vez con:

```sh
keytool -genkeypair -keystore reiwand.keystore -storetype PKCS12 -alias reiwand \
  -keyalg RSA -keysize 4096 -validity 36500 -dname CN=reiwand
base64 -w0 reiwand.keystore
```

Una compilación local usa ese almacén con `REIWAND_KEYSTORE=/ruta/al/almacen` y
`REIWAND_KEYSTORE_PASSWORD`; si no, se firma con la clave de depuración del ordenador y antes de
instalarla hay que quitar el APK publicado con `adb uninstall io.github.reiwand`, que borra
también la imagen elegida. Su número de versión es 1 y el de la CI es el de cada ejecución, de
ahí el `-d` de `adb install`.

Flatpak local (compila sin red sobre el SDK de KDE 6.11, con Rust 1.99.1 de static.rust-lang.org y
las fuentes de Cargo descargadas antes por `flatpak-builder`, comprobadas con su SHA-256). Solo
entran los archivos que sigue git, con sus cambios sin confirmar; los que no sigue, como otras copias
de trabajo dentro de la carpeta, se quedan fuera. El SDK y la extensión de LLVM vienen de Flathub:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
packaging/flatpak/sources.sh            # copia a target/flatpak-src y genera cargo-sources.json
flatpak-builder --user --install --install-deps-from=flathub --force-clean \
  --state-dir=target/flatpak-builder target/flatpak \
  packaging/flatpak/io.github.reiwand.Reiwand.yml
```

Con `REIWAND_VERSION`, `sources.sh` pone esa versión en la copia (sin ella, la del `Cargo.toml`) y
`packaging/flatpak/releases.sh` escribe en el metainfo de la copia la lista `<releases>`: esa versión
con la fecha del último commit y las novedades y arreglos desde la etiqueta anterior, y las diez
versiones publicadas antes, así que `appstreamcli` ya no avisa de `releases-info-missing`. La CI construye el Flatpak para
x86_64 y para aarch64, este en un runner ARM de GitHub (gratuito en los repositorios públicos), y
`packaging/flatpak/check.sh` instala cada paquete y comprueba las versiones, `reiwand-app --check` sin
pantalla, los archivos exportados (lanzador, icono, metainfo y servicios D-Bus), las órdenes de
`reiwand-plasma` y que una edición hecha dentro del Flatpak llega a `~/.local/share/reiwand/edits/`.
Instala en el sistema y toca esa carpeta, así que fuera de la CI conviene usarlo en un contenedor
desechable.
El demonio dentro del sandbox lo prueba otro job en el runner sin contenedor, con el paquete ya
construido: `scripts/private-audio.sh packaging/flatpak/serve-check.sh` (la imagen de construcción no
trae servidor de sonido, y el demonio lo necesita siempre).

Todas las acciones de GitHub están fijadas por el SHA completo de su commit, las imágenes de
contenedor por su resumen y las herramientas que descarga la CI (`scripts/tools.sh`: actionlint,
cosign y syft) por versión y SHA-256. La CI pasa actionlint, con shellcheck, por todos los
workflows. Al publicar una versión, `scripts/release.sh` genera con syft el SBOM de los archivos que
sigue git y firma `SHA256SUMS` con cosign sin clave propia (OIDC de GitHub; el trabajo solo tiene
permiso de escritura en el contenido y de pedir el token), y `scripts/server-image.sh` construye
la imagen del servidor con un BuildKit fijado, le adjunta los SBOM y la procedencia, la sube a
GHCR, firma su resumen y le añade la atestación con el SBOM de las fuentes. La primera imagen crea
el paquete `reiwand-server` como privado: hay que hacerlo público una vez en sus ajustes de GitHub.

Los fondos web se prueban donde hay WPE WebKit: el trabajo *Web wallpapers* de la CI ejecuta,
en un contenedor de Arch con `wpewebkit`, la prueba del auxiliar (`apps/reiwand-web/tests/page.rs`:
la interfaz `window.reiwand`, el límite de FPS, la pausa, que la página no llega a la red, a otros
paquetes ni a archivos del equipo) y `scripts/web-test.sh` con el servicio, `reiwand-ctl` y una
página de color. En Ubuntu, `scripts/integration-test.sh` comprueba que el fondo se rechaza con su
motivo. `REIWAND_REQUIRE_WEB=1` hace que ambas fallen si falta el motor. El mismo trabajo ejecuta
`scripts/video-test.sh` (`scripts/integration-video.sh`) en ese contenedor de Arch, con PipeWire y
`gst-plugin-pipewire` 1.6: es la única prueba de la rama del sonido directo por `pipewiresink` 1.2 o
posterior (el informe de que no hay salida de audio, un sumidero nulo, el volumen y el silencio),
porque en Ubuntu 24.04 el plugin es el 1.0; `REIWAND_REQUIRE_PIPEWIRE_SOUND=1` hace que falle si el
plugin es anterior a 1.2. En el teléfono,
`WebPageTest` (pruebas instrumentadas en el emulador) muestra una página de prueba con el WebView y
comprueba sus valores, el color que dibuja, la pausa, el límite de fotogramas, lo que oculta cada
ajuste de privacidad, el borrado de sus datos, los archivos elegidos, los clics solo con permiso y
que no llega a la red, a otros paquetes ni a archivos del teléfono.

La CI ejecuta además `scripts/plasma-test.sh` en un contenedor con Plasma 6: instala el
paquete con `install.sh`, arranca una sesión Wayland sin pantalla (KWin y plasmashell sobre
dispositivos DRM virtuales), activa el plugin, comprueba los píxeles de una captura, carga el icono
de la bandeja, abre la app en su página *Ajustes* desde el botón de la bandeja y con
`--page settings`, comprueba en un bus aparte que la bandeja solo ofrece la app donde está
instalada y, con `qmltestrunner` de `qt6-declarative` (`tests/tray-row.qml`), que la miniatura de
una pantalla, en la bandeja y en la app, sigue difuminada con un paquete para adultos hasta
pulsarla con el ratón o el teclado, y que la bandeja avisa si el fondo destella. También ejecuta
`scripts/sway-test.sh` en un Sway sin pantalla: captura cada salida con `grim`, conecta y quita una
pantalla y comprueba que el vídeo se pausa bajo una ventana a pantalla completa y con la pantalla
apagada (Sway dibuja por software; con `REIWAND_SWAY_RENDERER=gles2` dibuja con la tarjeta gráfica,
lo que necesita un controlador que no presenta en superficies de memoria compartida), y
`scripts/x11-test.sh` en un Xvfb con dos monitores, el fondo de la ventana raíz y la
misma pausa.

| Ruta | Contenido |
|------|-----------|
| `crates/reiwand-core` | Configuración, modos de ajuste, nombres D-Bus |
| `crates/reiwand-render` | Renderizador wgpu, sin ventana o sobre superficies Wayland, X11 y Android, con escenas y conversión de vídeo YUV |
| `crates/reiwand-depth` | Mapas de profundidad: modelo bajo demanda, cálculo con ONNX Runtime y caché |
| `crates/reiwand-scene` | Modelo de las escenas: capas, valores enlazados, líneas de tiempo y efectos, sin GPU, y su esquema JSON |
| `crates/reiwand-editor` | Editor de escenas compartido por la app y `reiwand-ctl scene`: operaciones, historial, puntos de partida, inspector y carpetas de escena |
| `crates/reiwand-pipewire` | Exportación de streams de vídeo PipeWire |
| `crates/reiwand-video` | Reproducción de vídeo en bucle y del sonido de las escenas con GStreamer |
| `crates/reiwand-mix` | Mezclador de las capas de sonido de una escena (listas, pausas, fundidos, paso de un lado a otro y alineación con la escena), compartido por el escritorio y Android |
| `crates/reiwand-wayland` | Superficies layer-shell y ventanas de escritorio X11, escala, ritmo y oclusión |
| `crates/reiwand-audio` | Captura del sonido con PipeWire y su análisis en bandas para las escenas, también a partir del espectro que da Android |
| `crates/reiwand-script` | Scripts de escena en JavaScript, aislados en un proceso por escena con límites de CPU y memoria, su módulo `reiwand` y `reiwand/math` |
| `crates/reiwand-account` | Cliente del servidor propio para las cuentas: inicio de sesión con código, certificados fijados por origen y el archivo de cuentas, compartido por el escritorio y Android |
| `crates/reiwand-carry` | El archivo `.reiwand` para exportar e importar la configuración: contenedor, cifrado, límites contra entradas hostiles y reglas de unión |
| `crates/reiwand-upgrade` | El programa de una sola vez que convierte los datos de la 1.x (servidor, escritorio y teléfono) al formato actual |
| `crates/reiwand-android` | Fondo animado de Android en Rust: JNI, GPU compartida, una superficie por fondo, biblioteca, listas, ajustes, escenas, paralaje, mapas de profundidad, repositorios con cuentas por repositorio, certificados fijados y exportar e importar la configuración |
| `apps/reiwandd` | Daemon |
| `apps/reiwand-ctl` | CLI |
| `apps/reiwand-app` | App de escritorio (Qt Quick y Kirigami sobre Rust con cxx-qt) |
| `apps/reiwand-server` | Servidor propio: repositorio, catálogo, subidas, moderación y cuentas |
| `android` | App de Android: proyecto Gradle, servicio de fondo, reproductor de vídeo e interfaz Compose en Kotlin |
| `integrations/plasma-wallpaper` | Plugin de fondo de Plasma 6 |
| `integrations/plasma-tray` | Icono de gestión en la bandeja de Plasma 6 |
| `integrations/kwin-pointer` | Script de KWin que envía la posición del puntero mientras una escena o un paralaje la usa |
| `po` | Traducciones gettext de los paquetes de Plasma y de la app |
| `packaging` | Servicio systemd, activación D-Bus, lanzador e icono, scripts de instalación, Flatpak |
| `deploy` | Ejemplo de Nginx delante del servidor y variante de `compose.yaml` que construye la imagen desde las fuentes |
| `docs` | Especificaciones de la interfaz D-Bus, los paquetes y sus escenas, el editor de escenas, los repositorios firmados y el servidor |

## Cómo funciona

```
 Plasma (plugin de fondo, QML)                    reiwandd (daemon, Rust)
 ┌──────────────────────────┐   D-Bus             ┌───────────────────────────┐
 │ RequestStream(pantalla,  │ ──────────────────► │ renderiza con wgpu        │
 │   ancho, alto)           │ ◄── nodo, serial ── │ exporta un stream PipeWire│
 │ PipeWireSourceItem       │ ◄══ vídeo PipeWire ═│ por pantalla              │
 └──────────────────────────┘                     └───────────────────────────┘
                                   reiwand-ctl (CLI) ──► D-Bus
```

- `reiwandd` renderiza en la GPU sin ventana propia y publica un stream PipeWire por pantalla.
- El plugin de Plasma pide un stream del tamaño exacto de su pantalla y lo muestra con
  KPipeWire, sin tapar iconos ni widgets del escritorio.
- `reiwand-ctl` y la app de escritorio controlan el daemon: cada acción está en los dos. El icono
  de la bandeja ofrece las más frecuentes y abre la app.
- Los vídeos se decodifican con GStreamer y se pausan cuando una ventana maximizada o a
  pantalla completa tapa la pantalla, con batería o con la pantalla bloqueada; con la pantalla
  bloqueada no suenan nunca, aunque no se pausen.
