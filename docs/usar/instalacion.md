# Instalación (sin root)

Cada versión de la página *Releases* trae siempre los mismos nombres de archivo y un `SHA256SUMS`
para comprobarlos, firmado con [cosign](https://github.com/sigstore/cosign) sin clave propia: la
firma (`SHA256SUMS.sigstore.json`) lleva un certificado de corta duración que Sigstore emite para la
CI de este repositorio en la rama principal y queda en su registro público de transparencia. Las
versiones publicadas antes de pasar a la organización `reiwand` no llevan firma. Con cosign 3
o posterior, junto a los archivos descargados:

```sh
cosign verify-blob SHA256SUMS --bundle SHA256SUMS.sigstore.json \
  --certificate-identity https://github.com/reiwand/reiwand/.github/workflows/ci.yml@refs/heads/master \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
sha256sum -c --ignore-missing SHA256SUMS
```

Descarga `reiwand-x86_64-linux.tar.gz` de la última y ejecuta:

```sh
tar xzf reiwand-x86_64-linux.tar.gz
cd reiwand-*-x86_64-linux
./install.sh
```

Instala todo en tu directorio personal: binarios en `~/.local/bin`, servicio de usuario de
systemd, activación por D-Bus, la app de escritorio (con su lanzador e icono), el plugin de fondo
y el icono de la bandeja de Plasma. Funciona también en distribuciones con
sistema de archivos inmutable. `install.sh` pide al bus de sesión que relea sus servicios, para que
la bandeja abra la app sin volver a iniciar sesión, también con dbus-broker, que no vigila los
servicios nuevos; si aun así el bus no conoce la app, la bandeja le pide que relea sus servicios y lo intenta
otra vez, y si tampoco así se abre, lo dice. Para
desinstalar: `./uninstall.sh` (añade `--purge` para borrar también la configuración).
En Debian y Ubuntu, el selector de archivos del escritorio necesita además el paquete
`qt6-xdgdesktopportal-platformtheme`; sin él la app dibuja su propio selector, e `install.sh` lo avisa.

Después, en Plasma: clic derecho en el escritorio → *Configurar escritorio y fondo de
pantalla* → tipo de fondo **reiwand**. Con escala fraccional y Qt anterior a 6.11, el plugin
pide más píxeles de los que tiene la pantalla; el daemon reduce el stream a los píxeles reales de la
pantalla, los del modo que anuncia el compositor Wayland, para no gastar GPU ni memoria de más (el
tamaño y la escala que le pasa el plugin no bastan, porque llevan la misma escala que la petición).

El icono de Reiwand en la bandeja reúne lo básico con el aspecto de la app: pausar o reanudar,
silenciar, el volumen, una portada o una fila por pantalla con la miniatura y el nombre de su fondo y
los botones *Siguiente* (solo con una lista en marcha), *Elegir…* y *⋯* (favorito, ajuste, filtro y
quitar el fondo), *Cambiar con vista previa…* y *Más* (sonido, destellos y pausa con batería); si el
fondo destella o está quieto, la fila lo dice. También pregunta
qué permitir a una escena que lo pide. Todo lo demás está en la app: *Abrir reiwand* la abre y
*Ajustes…* la abre directamente en su página *Ajustes*, con el modo de ajuste, el color de fondo,
el filtro y la lista de cada pantalla o de todas a la vez, los ajustes de cada escena, los fotogramas
por segundo, las pausas y las transiciones. El filtro (botón *Filtro*, o `reiwand-ctl filter`) colorea
cualquier fondo de una pantalla, imagen, vídeo, escena o página, sin tocar los archivos: uno de 15
estilos propios (cálido, frío, dorado, vivo, intenso, apagado, mate, desvaído, blanco y negro, noir,
sepia, atardecer, menta, rosa y noche) con su intensidad, más exposición, contraste, saturación,
calidez y tono. Va dentro de la pasada que dibuja el fondo en la pantalla, así que no añade otra, y
sin filtro no cuesta nada. Los perfiles guardan y ponen el filtro de cada pantalla junto con su fondo. Lo mismo hace `reiwand-app --page settings` (o la acción
*Ajustes* del lanzador); `--page` acepta también `library`, `scenes`, `playlists`, `profiles`,
`providers`, `repos` y `account`. Si la app ya está abierta, pasa a esa página, salvo que el editor
de imágenes o el de escenas tenga cambios sin guardar: entonces solo se muestra la ventana y el
editor sigue abierto.

Requisitos: glibc 2.39 o posterior para los binarios de las versiones (se compilan en Ubuntu 24.04; no corren en Debian 12, que trae la 2.36, y una compilación local pide la glibc del sistema donde se hizo; `install.sh` avisa si la de este sistema es más antigua), KDE Plasma 6.4 o posterior, PipeWire, GStreamer 1.24 o posterior (con los plugins base
y good y el de PipeWire, que lleva el sonido de los vídeos y de las escenas desde su versión 1.2;
sin él o con uno anterior, el sonido va a la salida por defecto; si aún no hay ninguna salida de sonido, el fondo
suena en cuanto aparece una) y un controlador Vulkan u OpenGL. `install.sh` avisa si no encuentra
GStreamer 1.24 o posterior, y el paquete RPM lo exige. El plugin de fondo usa los módulos QML de KPipeWire,
KWindowSystem y plasma-workspace; en Debian y Ubuntu vienen en `qml6-module-org-kde-pipewire`,
`qml6-module-org-kde-kwindowsystem` y `plasma-workspace`. La app de escritorio usa Qt 6 Quick,
Kirigami y el estilo de escritorio de KDE (`qml6-module-org-kde-kirigami` y
`qml6-module-org-kde-qqc2desktopstyle`).

## Actualizar desde 1.x

La versión 2.0.0 ya no entiende los formatos de la 1.x dentro de los programas: un programa aparte,
`reiwand-upgrade`, los convierte una sola vez y se puede repetir sin riesgo.

- Escritorio: `install.sh`, el servicio de usuario (también con el paquete RPM) y el Flatpak lo
  ejecutan solos antes de que arranque el demonio. A mano: `reiwand-upgrade client`. Si el equipo
  sincronizaba con un servidor, baja una sola vez lo que solo guardaba ese servidor a
  `from-server.reiwand` (en la carpeta de datos), que se suma con `reiwand-ctl import`: actualiza los
  equipos antes que el servidor.
- Teléfono: la app lo ejecuta sola al actualizarse. Quita las huellas de las direcciones guardadas de
  repositorios y no descarga nada.
- Servidor: la imagen lo ejecuta al arrancar; sin la imagen, `reiwand-upgrade server --data DIR` con el
  servidor parado. Antes guarda una copia de la base junto a ella.

- Escenas propias (para quien crea): `reiwand-upgrade scenes CARPETA...` añade `distance` y `keep`
  (los valores que dan el aspecto de antes) a cada efecto `chroma_key` de los `scene.json` bajo esas
  carpetas; después hay que volver a empaquetar y publicar los paquetes. Un paquete ya firmado no se
  toca: su autor debe publicarlo de nuevo.

Si un programa se niega a arrancar diciendo que los datos son de una versión anterior, ejecuta ese
comando. Las direcciones de la 1.x con `?fingerprint=` o `?repo_id=` ya no se aceptan: usa la dirección
simple del repositorio. Un repositorio que un servidor 1.x daba en `/repo/` pasa a estar en `/r/main/`.

## Versiones

Las versiones siguen Semantic Versioning 2.0.0 de forma estricta. GitVersion las calcula en la CI
a partir de los commits de la rama principal, escritos con Conventional Commits
(`tipo(ámbito)!: descripción`, donde el ámbito y `!` son opcionales):

- `fix` y `perf` suben el parche: 1.0.0 → 1.0.1.
- `feat` sube la versión menor: 1.0.0 → 1.1.0.
- `!` tras el tipo o el ámbito, o un pie `BREAKING CHANGE:` o `BREAKING-CHANGE:`, sube la mayor,
  también de 0.x a 1.0.0.
- `build`, `chore`, `ci`, `docs`, `refactor`, `revert`, `style` y `test` no cambian la versión.

La CI comprueba el asunto de todos los commits desde la última versión publicada y el título de
cada pull request, también cada vez que se edita; mientras alguno no siga ese formato, la rama
principal o la pull request fallan y no se publica nada.
Un commit mal escrito que ya está en la rama se corrige añadiendo su hash a `ignore.sha` en
`GitVersion.yml` junto con un commit nuevo del tipo correcto. GitVersion toma como actual el commit
de fecha más reciente, así que cada commit necesita una fecha de commit posterior a la de toda su
historia, algo que la CI también comprueba. Si la fecha se fija a mano, sirve
`GIT_COMMITTER_DATE=@$(( $(git log --format=%ct | sort -n | tail -1) + 1 ))`.

Solo los commits que suben la versión publican una nueva: cuando el último commit de la rama
principal pasa todas las pruebas y su versión es mayor que todas las publicadas, la CI crea
`v<versión>` en la página *Releases* del repositorio, con los archivos, un SBOM SPDX de las
dependencias (`reiwand.spdx.json`), un `SHA256SUMS` firmado y las notas agrupadas en cambios
incompatibles, novedades y correcciones, y publica la imagen firmada del servidor propio. La firma
y la imagen solo se hacen cuando el repositorio está en la organización `reiwand`. Una
versión publicada no cambia nunca y volver a lanzar una ejecución antigua no publica nada. Las
demás ramas y las pull requests llevan una prerelease de la versión siguiente cuyo número crece con
cada commit (`0.3.1-branch-rama.2`, `0.4.0-feature-rama.1`, `0.3.1-PullRequest12.1`) y no se
publican. `--version`, el daemon por D-Bus, el servidor y los paquetes de Plasma y KWin muestran la
versión; una compilación local sin `REIWAND_VERSION` usa la de `Cargo.toml` con el sufijo `-local`.
