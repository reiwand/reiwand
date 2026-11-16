# Fondos web

Un paquete `web` es una página (HTML, CSS, JavaScript, WebAssembly, WebGL, imágenes, fuentes,
sonido y vídeo dentro del propio paquete) que se muestra como fondo con las mismas transiciones,
ajuste, límite de FPS y pausas que los demás. La dibuja `reiwand-web`, un proceso aparte que el
servicio arranca para cada página que se ve y para en cuanto ninguna pantalla la muestra. Necesita
WPE WebKit 2.54 o posterior (`wpewebkit` en Arch y Fedora); donde no está (Ubuntu 24.04, que no trae
ningún `wpewebkit`, y el Flatpak, cuyo runtime de KDE tampoco lo trae y para el que no conocemos
ninguno en Flathub) el fondo se rechaza con el motivo en lugar de quedarse en blanco, en `reiwand-ctl` y en la app,
con el texto en español, y la biblioteca avisa antes de aplicarlo: la ficha de la página lo marca
con el motivo, sus detalles lo muestran y `reiwand-ctl lib show` da la línea `not shown here`. Meter WPE en el Flatpak exigiría compilar WPE WebKit 2.54 y sus dependencias
en cada versión (horas y varios GiB de memoria), así que por ahora no está. `docs/specs/web.md` explica el
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
eso se comprueba solo al arrancar la página, porque WPE WebKit no deja fijar un límite por origen.
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
