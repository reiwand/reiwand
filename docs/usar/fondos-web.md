# Fondos web

Un paquete `web` es una página (HTML, CSS, JavaScript, WebAssembly, WebGL, imágenes, fuentes,
sonido y vídeo dentro del propio paquete) que se muestra como fondo con las mismas transiciones,
ajuste, límite de FPS y pausas que los demás. La dibuja `reiwand-web`, un proceso aparte que el
servicio arranca para cada página que se ve y para en cuanto ninguna pantalla la muestra. Necesita
Qt WebEngine (Qt 6.8 o posterior) y una sesión gráfica, que el servicio ya tiene: `qt6-qtwebengine` en Fedora
(45 trae la 6.11), `qt6-webengine` en Arch y, en el Flatpak, la base `io.qt.qtwebengine.BaseApp`, que ya
va incluida. Donde falta el motor (Ubuntu 24.04, que trae Qt 6.4, demasiado antigua) el fondo se rechaza con
el motivo en lugar de quedarse en blanco, en `reiwand-ctl` y en la app,
con el texto en español, y la biblioteca avisa antes de aplicarlo: la ficha de la página lo marca
con el motivo, sus detalles lo muestran y `reiwand-ctl lib show` da la línea `not shown here`.

Qué instalar en cada sistema:

- **Fedora Workstation, KDE Spin:** el paquete RPM lo recomienda y `dnf` lo instala solo; a mano:
  `sudo dnf install qt6-qtwebengine`.
- **Fedora atómico (Silverblue, Kinoite y derivadas):** `rpm-ostree install qt6-qtwebengine` y reiniciar
  para estrenar el nuevo despliegue (`systemctl reboot`). Con la instalación sin root (`install.sh`) hace
  falta igualmente, porque el motor no viene con el sistema base. Con el Flatpak no hace falta nada.
- **Arch y derivadas:** `sudo pacman -S qt6-webengine`.
- **Flatpak:** nada; el motor llega con la base de Qt WebEngine (unos 600 MB más al instalar) y la
  página corre además dentro del aislamiento de Chromium de Flatpak.
- **Ubuntu 24.04 y Debian 12:** no se pueden mostrar; el fondo se rechaza con el motivo.

El motor pesa más que el WebView del teléfono: una página a 1080p gasta del orden de 400 MiB y de 1,5 a
2 veces el procesador de lo que gastaba el motor anterior, y mientras una página no cambia no dibuja
nada. El Qt WebEngine de Fedora usa el `libavcodec-free` del sistema, una compilación de ffmpeg "limitada en
el número de códecs" (así la describe su paquete): se espera que no reproduzca H.264 ni AAC, aunque
no lo hemos probado con un vídeo real en Fedora 45; VP8, VP9, Opus y Vorbis se esperan bien (AV1 sin probar). En Arch el motor usa el
`ffmpeg` completo del sistema. `docs/specs/web.md` explica el
motor, el aislamiento y la interfaz de la página, y `docs/crear/fondos-web.md` es la guía para quien
los hace. El fundido hacia una página empieza cuando la página dibuja su primer fotograma (o falla),
así que el fondo anterior no se funde en una pantalla vacía, y al salir de una página se funde desde
su último fotograma. El límite de FPS frena todos los fotogramas de la página, no solo
`requestAnimationFrame`: el motor web no dibuja el siguiente hasta que pasa el periodo, también para
animaciones CSS, vídeos y lienzos; `reiwand-ctl state` cuenta los que dibujó cada pantalla
(`page_frames`). En el teléfono la página se dibuja directamente en la superficie del fondo, sin
fundido, y el límite solo frena `requestAnimationFrame`.

La página solo lee los archivos de su paquete, no llega a la red salvo que la declare y la permitas
(`reiwand-ctl input grant <id> network`, o el interruptor de sus detalles en la app), no ve rutas,
dispositivos ni la batería, y el motor web la ejecuta en su propio entorno aislado. Por defecto ve
tu zona horaria, tu idioma, lo que puede hacer tu tarjeta gráfica (WebGL) y guarda sus datos en
`~/.local/share/reiwand/web/<id>/` si los declara. Al aplicarla y en sus ajustes, la app, la
bandeja y `reiwand-ctl` avisan de qué usa, y puedes ocultar cada cosa por fondo: en los detalles del
paquete en la app (*Privacidad*) o con
`reiwand-ctl lib confirm hide_zone|hide_language|hide_gpu|hide_storage ELEMENTO` (`lib unconfirm` lo
deshace). *Borrar datos* en la app o `reiwand-ctl lib clear-data ELEMENTO` borra lo que guardó, y se
borra solo al quitar el paquete. Si lo que guarda pasa del doble de lo que declara más 1 MiB, la
página arranca sin guardar nada y su error lo dice hasta que se borren sus datos; en el escritorio
eso se comprueba solo al arrancar la página, porque el motor web no deja fijar un límite por origen.
Los permisos de sonido, puntero y clics son los mismos que los de
las escenas: los clics solo llegan desde clics reales en las pantallas donde los activas. Una página
oculta deja de dibujar al momento y, tras 5 minutos oculta, se detiene su proceso y queda su último
fotograma. El ejemplo *Luces* (`scenes/lights`) es una página web.

En el teléfono los mismos paquetes se muestran con el WebView del sistema, el motor que Android
mantiene al día para todas las apps, dibujado directamente sobre el fondo sin copias. La página ve
la misma interfaz y tiene las mismas restricciones: solo lee sus archivos y los que eliges (la app
sirve cada petición), no llega a la red salvo que la declare y la permitas, no tiene cookies,
ventanas, diálogos, descargas ni permisos, y no puede abrir conexiones entre pares (WebRTC) ni en
sus marcos: un script que corre antes que los suyos se las quita a la página, a sus marcos (también
`about:blank`, `srcdoc` y los de árboles shadow), ventanas y documentos reabiertos, y la política de
seguridad del contenido lleva `webrtc 'block'` para los motores que la respetan (el WebView 113 no). Su página en la app avisa, encima de *Usar como fondo*, de lo que
usa (zona horaria, idioma, modelo del chip gráfico y datos que guarda en el teléfono), y su sección
*Privacidad* tiene un interruptor para cada cosa, otro para la red si la declara y *Borrar datos*;
el mismo aviso sale al importarla. El límite de lo que guarda es el del escritorio: el teléfono lo
mide cada vez que la página arranca (lo que cuenta el WebView para su origen más su
`localStorage`) y, mientras corre, rechaza con `QuotaExceededError` lo que `localStorage` quiera
guardar de más (con `setItem` o como propiedad, en la página y en sus marcos); tras escribir en
IndexedDB o Cache Storage, y al dejar de verse, mide el origen de nuevo, y si pasa del límite la
página arranca otra vez sin guardar nada. Si al arrancar ya pasa del límite, la página corre sin guardar nada y su
sección *Privacidad* lo avisa hasta *Borrar datos*. Ocultar la zona horaria y el idioma lo hace un script que corre
antes que los de la página (en ella, sus marcos y sus workers); no cubre lo que dibuja el propio
motor (selectores de fecha y hora, tipografías según el idioma) ni el `Accept-Language` de las
peticiones, que siguen al teléfono. *Reaccionar al tacto* le da
el puntero y *Reaccionar a los toques* los clics, solo desde toques reales y solo en la pantalla de
inicio; su sonido sigue el ajuste de sonido de los vídeos. Se pausa al dejar de verse, con el ahorro
de energía, con poca batería o con el teléfono muy caliente, se cierra tras 5 minutos oculta y
`requestAnimationFrame` va como mucho a los fotogramas por segundo del teléfono; el cambio a una
página o desde ella no tiene fundido. En un emulador, *Luces* dibuja 30 fotogramas por segundo con
el límite de 30, una página fija no gasta nada después de su primer fotograma y una página oculta
no dibuja ni usa el procesador.
