# Iluminación RGB

Un fondo puede pintar el teclado, el ratón y otras luces RGB a través de un servidor de OpenRGB
(con su protocolo SDK; el cliente es propio). Una escena lo hace con una capa `adjustment` con
`rgb: true`, que manda lo que capta (todo el lienzo o la región de su `size`) reducido a 64 × 16
como mucho; una página web, con `window.reiwand.rgb.setImage(bytes, ancho, alto)`, que devuelve `true`
solo cuando los colores llegan a las luces (o a la vista previa) y `false` sin el permiso, con la
iluminación apagada o en el teléfono. En ambos casos el
paquete declara `permissions.rgb`, se envía como mucho 30 veces por segundo y solo cuando cambia el
fotograma, y nada sale hasta que haces dos cosas: encender la iluminación en *Ajustes → Iluminación
RGB* (`reiwand-ctl config rgb.enabled true`) y permitirla para ese paquete (el aviso de permisos de la
app o `reiwand-ctl input grant <id> rgb`). El servidor es este equipo (`127.0.0.1:6742`) salvo que
pongas otra dirección (`rgb.server`), y entonces la app avisa de que los colores viajan por la red.
La misma sección tiene *Conectar*, la lista de dispositivos con sus luces y *Probar* en cada uno (un
arcoíris de 3 segundos); `reiwand-ctl rgb status`, `rgb devices`, `rgb test [N]` y `rgb strip -o
SALIDA` hacen lo mismo desde la terminal. Mientras una ventana tapa el fondo que manda los colores y
el fondo se pausa por eso (`pause.when_covered`, lo predeterminado), eliges qué hacen las luces
(*Mientras una ventana tapa el fondo y este se pausa* en la misma sección, `reiwand-ctl rgb covered follow|pause` o
`rgb.when_covered`): *Pausar y dejar las luces como están* (lo predeterminado) no manda nada, no
dibuja ni capta nada para las luces y deja los dispositivos con sus últimos colores hasta que el
fondo vuelve a verse, sin devolverlos a su propio modo para no hacerlos parpadear; *Seguir el fondo*
mantiene las luces cambiando con la escena tapada: la escena sigue a 30 fotogramas por segundo como
mucho, dibujada a 320 píxeles por lado como mucho solo para la tira y nunca en la pantalla, sin
que esos fotogramas cambien su nivel de calidad, y una página sigue igual sin mostrarse, las dos solo mientras el servidor está conectado y tiene
dispositivos (si no, el fondo tapado sigue en pausa y el servicio vuelve a probar el servidor cada
10 segundos, solo mientras ese fondo tiene el permiso `rgb`, manda colores y nada más lo pausa:
nunca por una imagen, un video, un paquete sin colores, la sesión bloqueada o la batería; el
servicio cierra la conexión, la haya abierto un fondo tapado o uno a la vista, en cuanto nada tapado
pide las luces y ninguna pantalla a la vista y en marcha manda colores, al bloquear, pausar, pasar a
batería, cambiar a un fondo sin colores o quitar la pantalla, y la mantiene mientras una pantalla a la
vista siga mandando colores aunque la última en pintar haya sido la tapada). Si el fondo tapado no se pausa, sigue dibujándose y las luces lo
siguen como siempre, elijas lo que elijas. `reiwand-ctl rgb covered` y `reiwand-ctl rgb status` muestran
la elección y si las luces están en pausa ahora; con *Seguir el fondo*, `reiwand-ctl rgb status` y la
sección RGB de la app dicen además, pantalla por pantalla, por qué las luces esperan: el fondo está
en pausa, la sesión bloqueada o el equipo con batería (y el fondo se pausa entonces), el paquete
no tiene el permiso `rgb` o la pantalla tapada muestra algo sin colores. Cada dispositivo pasa a su modo directo solo cuando
se le pintan colores y recupera su propio modo (y sus colores, si ese modo fija cada luz) en cuanto
nada manda colores: al detenerse o cambiar el paquete, al quitar la pantalla, al apagar la
iluminación, al cambiar de servidor, al terminar una prueba sin nada que pintar y al detenerse el
servicio. Ese modo propio se guarda en este equipo mientras el dispositivo está tomado, así que si el
servicio se cierra de golpe o el servidor no responde al devolverlo, vuelve en cuanto el servicio
alcanza de nuevo el servidor. La lista de dispositivos de la app y `reiwand-ctl rgb devices` muestran ese modo propio y si
volverá cuando paren las luces. Cada dispositivo toma toda la imagen hasta que lo ubicas: en la
misma sección, el tablero de luces muestra la imagen que reciben (la tira de 64 × 16 del fondo que
las manda) y eliges un dispositivo o una de sus zonas, arrastras su marco sobre la imagen y tiras de
su esquina para darle tamaño; también eliges hacia dónde corre una fila de luces (de izquierda a
derecha, al revés, de arriba abajo o de abajo arriba), *Toda la imagen*, *Dejar el dispositivo en
paz* (no se le toca el modo) o *Dejar la zona apagada*. En la terminal es `reiwand-ctl rgb place
"NOMBRE" "x y ancho alto [right|left|down|up]"` (fracciones de la imagen; `--zone N` para una zona,
`off` o `none`; si dos dispositivos se llaman igual, el nombre lleva entre paréntesis su número de
serie, o su orden si no lo tienen, como lo muestra `reiwand-ctl rgb devices`), y `reiwand-ctl rgb place` lista lo ubicado; se guarda en este equipo y no se
sincroniza. Para probar una página web con sus luces, su ficha de la biblioteca tiene *Probar la
página y sus luces…*: la muestra en una vista previa con la tira que mandaría a las luces, aunque la
iluminación esté apagada o el paquete no tenga permiso (ahí `setImage` devuelve `true` y los colores
solo se ven); al cerrarla se olvida. Desde la terminal, `reiwand-ctl stream preview:NOMBRE`, `reiwand-ctl
lib apply ID -o preview:NOMBRE`, `reiwand-ctl rgb strip -o preview:NOMBRE` y `reiwand-ctl release
preview:NOMBRE` hacen lo mismo. Cada zona con matriz (las teclas) toma la celda de
la tira que le cae encima y las demás, la columna de cada luz. En el editor, el inspector de una capa
`adjustment` tiene *Enviar a la iluminación RGB* (activa también el permiso del paquete, `reiwand-ctl
scene act DIR rgb`), *Todo el lienzo* o *Una franja*, la región con asas en la vista
previa y la tira de 64 × 16 que reciben las luces, en vivo. El teléfono no tiene iluminación RGB y
reproduce esos paquetes sin ella.
