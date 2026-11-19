# Teléfono: uso diario

Las pantallas que una persona usa todos los días en la app del teléfono (`android/`): la
biblioteca, la página de un fondo y las listas. Cada pantalla dibuja solo su contenido sobre el
marco de la app (`Shell.kt`): el póster entero del fondo en uso detrás, el velo y la barra de tres
palabras (Biblioteca, Listas, Ajustes). Mientras una lista está puesta, el póster es el del fondo
que se ve en ese momento.

## Biblioteca

La fila de arriba lleva la búsqueda y «Añadir» como dos cuadros con icono
(`Now.kt`). Debajo va la tarjeta del fondo en uso: dónde está puesto (solo cuando el servicio de
fondos de la app está elegido en una pantalla), el título, sus datos, «Opciones» y «Favorito»
(`Now.kt`, primer elemento de la cuadrícula de `Library.kt`). La cuadrícula muestra cada portada con
su nombre debajo; la del fondo en uso dice «En uso» y un fondo con destellos lleva una línea ámbar
bajo el nombre en vez de verse borroso (`Cover.kt`). Abrir la búsqueda sube la cuadrícula y enfoca el
campo. La búsqueda encuentra un paquete por su título en cualquiera de los idiomas que trae
(«Caja» y «Turning» hallan «Caja que gira»), con cualquier filtro; si la persona le cambió el
nombre, solo por el nombre nuevo. Una búsqueda sin resultados dice qué se buscó y ofrece borrarla; «Favoritos» vacío ofrece ver
todo; sin red, la línea de conexión desaparece.

Una pulsación larga en una portada abre la selección (`Select.kt`, `Picked.kt`). Arriba: «Cancelar»,
«N elegidos» y «Elegir todos». Abajo: «Lista», «Favorito», «Más» («Quitar de la lista», «Quitar
favorito», «Ajuste», «Quitar») y «Rotar estos». «Rotar estos» crea la lista «Rotación», que cambia
cada hora (si el nombre ya existe, el núcleo le agrega un número y el aviso muestra el nombre real),
y avisa «Lista «Rotación» creada: cambia cada hora» con «Cambiar», que abre el intervalo. Dentro de
una lista la barra no muestra «Lista» ni «Rotar estos», y «Más» ofrece «Quitar de la lista».
«Favorito» en la barra siempre marca; desmarcar está en «Más» → «Quitar favorito».

Piezas compartidas con las otras pantallas: `Seek` (campo de búsqueda), `Cover` y `Thumb` (portadas),
`Choose` (hoja para elegir una lista) y `Removing` (pregunta antes de quitar). Pruebas:
`LibraryLookTest` (normal, búsqueda vacía, favoritos vacíos, sin red, selección y rotación).

## Página de un fondo

`Detail.kt`. Se abre al tocar una portada en la biblioteca o un fondo dentro de una lista.

- **Una sola columna que se desplaza**, con un pie fijo abajo. En vertical, arriba va la vista
  previa en vivo (un poco más de un tercio del alto de la pantalla) con la flecha de volver
  encima; en horizontal, la vista previa a la izquierda y la columna a la derecha, con el título
  arriba.
- **Vista previa.** Muestra la miniatura al instante y la imagen completa la reemplaza cuando se
  terminó de decodificar, fuera del hilo de la interfaz: una foto grande nunca deja el cuadro
  vacío ni congela la página.
- **Encabezado.** El título, la marca «En uso» si es el fondo puesto y una línea de datos: el
  tipo (Imagen, Vídeo, Escena, Página web), el tamaño y la duración cuando la tiene.
- **Por qué se ve distinto.** Si es el fondo en uso y el motor lo frena, una línea lo dice con
  palabras: «Quieta porque es pesada», «Calidad reducida por la batería» o «Calidad reducida por
  el calor», cada una con su explicación. Se consulta cada dos segundos solo mientras la página
  está a la vista y el fondo está puesto.
- **Destellos.** Un fondo que destella con el ajuste de destellos apagado dice «Tiene destellos,
  deshabilitado» y no se puede poner. El contenido para adultos lo filtra el ajuste general: la
  página no lo tapa.
- **Encuadre** (solo imágenes): arrastrar y pellizcar la vista previa, con la ayuda «Arrastra y
  pellizca la vista previa para encuadrar la imagen», el ajuste y «Restablecer el encuadre».
- **Lo propio del fondo:** las estrellas, las etiquetas («Añadir etiquetas», que pide los nombres
  separados por comas) y «Cambiar el nombre», ambos en una hoja desde abajo con su campo; la
  descripción; lo que el teléfono no puede mostrar de ese fondo; los fondos parecidos; el mapa de
  profundidad de una foto; las opciones de una escena y lo que su autor deja ajustar.
- **Pie fijo:** Favorito, «Añadir a una lista», Quitar y «Poner de fondo», que pregunta una sola
  vez dónde: la pantalla de inicio, la de bloqueo o las dos. Quitar pide confirmación.
- Cada control de la página se ve entero y se puede tocar en la pantalla más chica que admite la
  app, con los tres botones del sistema o con gestos.

## Listas

`Playlists.kt` (la pestaña) y `Entries.kt` (la página de una lista).

### La pestaña Listas

- Arriba el título «Listas». Cada lista es una fila con la miniatura de su primer fondo, el
  nombre y debajo, en una línea, la marca «En uso» si está puesta en alguna pantalla, el
  intervalo y cuántos fondos tiene: «Cada hora · 3 fondos». Una lista con fondos que todavía no
  están en este teléfono lo dice debajo de su fila.
- Debajo de las filas, «Nueva lista» abre una hoja desde abajo con el campo «Nombre» y «Crear».
- **Vacía:** «Aún no hay listas», con la explicación «Elige varios fondos en la biblioteca y toca
  «Rotar estos».», el botón «Ir a la biblioteca», que cambia a esa pestaña, y «Nueva lista».
- **Error:** si no se pueden leer las listas, la pestaña no finge estar vacía: dice «No se
  pudieron leer las listas», ofrece «Reintentar» y guarda el motivo en «Detalles».

### La página de una lista

- Arriba la flecha de volver, el nombre y el lápiz «Cambiar el nombre de la lista» (hoja con el
  campo «Nombre» y «Guardar»).
- **Lista vacía** (una lista manual sin fondos): «Esta lista está vacía», «Añade fondos desde su
  página en la biblioteca.» e «Ir a la biblioteca».
- Las reglas, en filas: «Cambiar de fondo» dice su intervalo y abre una hoja con las siete
  opciones (cada 10, 15 o 30 minutos, cada hora, cada 6 horas, cada día o nunca) y «Otro…», que pide
  minutos enteros de 1 a 43200 y los guarda como `45m`; una lista con otro intervalo lo dice tal cual
  («Cada 45m»); «Orden
  aleatorio»; «Saltar los vídeos con batería»; «El mismo fondo en la pantalla de inicio y en la de
  bloqueo»; los días de la semana y las horas fijas («Añadir una hora», cada hora con su «Quitar
  esta hora»).
- **Búsqueda inteligente:** el campo de la búsqueda, cuántos fondos coinciden, «Guardar la
  búsqueda» y «Hacerla manual». Una lista con búsqueda se llena sola.
- «Poner de fondo» pone la lista (con la misma pregunta de dónde). «Borrar la lista» no borra
  enseguida: pregunta «¿Borrar la lista «…»?», avisa que sus fondos siguen en la biblioteca y el
  botón que borra es el único lleno, en coral.
- **Fondos:** una fila por fondo con su miniatura y su nombre. Tocarla abre su página; mantenerla
  pulsada empieza a elegir varios. En una lista manual cada fila lleva «Quitar de la lista» y un
  asa para arrastrar y cambiar el orden.
- **«Más opciones»**, plegado al final: «Seguir donde se quedó» y «Fundido entre sus fondos»
  («Como en Ajustes», «Ninguno» o una duración).

## Pregunta agrupada

Cuando un fondo pide algo que nadie respondió todavía (sonido, sonido por red, tocar, toques,
medios, medios por red, red), su página muestra un solo bloque bajo la línea de estado: «Este fondo
pide», la nota «Hasta que respondas, funciona sin esto.» y una fila por cada pedido con «Sin eso» y
«Permitir» (`Asks.kt`). No se dibuja nada si el fondo no pide nada (fotos, videos). «Permitir» en el
sonido pide antes el micrófono al sistema; si el sistema lo niega, se avisa y no se guarda la
respuesta. Cada respuesta recarga el fondo con lo permitido. El sonido por red no se pregunta si se
negó el sonido, ni los medios por red si se negaron los medios.

La misma pregunta existe como hoja desde el pie (`AsksSheet`), que se cierra sola cuando no queda
nada por responder; la usan el primer arranque y los ajustes. Las respuestas se guardan por paquete:
`inputs.json` (lo permitido) y `declined.json` (lo respondido «Sin eso»), por id y, como en el
escritorio, por quién hizo el paquete: la clave que firma uno instalado desde un repositorio
(`key:<huella>`, así sus actualizaciones conservan lo respondido) o el SHA-256 del archivo de
cualquier otro (`sha256:<hex>`). Otro paquete con el mismo id nunca hereda un permiso; las
respuestas guardadas antes de este cambio, solo por id, se descartan y el teléfono pregunta otra vez. Operaciones del núcleo del
teléfono (`crates/reiwand-android/src/asks.rs`): `asks` (lo que falta responder), `decide` (permitir y
negar varios a la vez; un pedido en las dos listas es el error `android.decided_twice`), y `scene`
devuelve también `undecided`.

Un fondo que el teléfono juzga pesado queda quieto con la línea «Quieta porque es pesada» y el botón
«Reproducir igual», que fija la calidad en «mínimo 1» para todos los fondos (se deshace en Ajustes →
Calidad). La primera vez por paquete aparece además un aviso arriba de la página con «Ahora no» y
«Reproducir igual».

## Primer inicio

Con la biblioteca vacía la página no muestra encabezado ni filtros: dice «Aún no hay nada» y ofrece
primero «Explorar repositorios» y después «Añadir fotos», con la tira de fondos de ejemplo y su
botón «Explorar», que abre el repositorio de ejemplos en Explorar. La hoja «Añadir» agrupa fotos, archivos (también paquetes), carpetas y fondos
animados. La página «Repositorios» (antes «Fuentes») lleva a «Agregar un repositorio», una página propia con un único campo, la dirección, y el botón fijo sobre el teclado; no hay campo de huella ni lector de códigos, y la app no pide el permiso de la cámara. Un enlace `reiwand://add?url=…` abre la misma página con la dirección escrita y no agrega nada hasta que se pulsa el botón. La dirección vale si empieza por https://; http:// solo vale para una dirección de este mismo teléfono (localhost o 127.0.0.1), y el aviso lo dice con esas palabras.

La respuesta decide lo que sigue: un repositorio público se agrega y se abre en Explorar; uno privado pide una cuenta («Usar la cuenta X», «Usar otra cuenta» o iniciar sesión con un código que se aprueba en el navegador); un servidor con certificado propio muestra su huella para confiar en ella, una sola vez, para esa dirección; la dirección de un servidor (no de un repositorio) lista los repositorios públicos que ofrece, para elegir uno; y una firma o un certificado que cambian se avisan sin aplicar nada. Un rechazo de la dirección aparece bajo el campo y lo marca; cualquier otro (el servidor no responde, la dirección no es un repositorio) aparece debajo sin marcar nada. El formulario decide el lugar por el código del rechazo, no por su texto.

Lo primero que se añade a una biblioteca vacía pasa a ser el fondo: un solo aviso dice «X ya es tu
fondo», con el nombre en el idioma de la app, y pregunta dónde (inicio, bloqueo o ambos). Si el
fondo del sistema ya es el nuestro se pone solo; si no, la vista previa del sistema lo confirma.
Una carpeta sin imágenes lo dice y ofrece «Elegir otra». Los tonos de la app salen del póster del
fondo en uso, una vez por cambio.

## Ajustes

Tres bloques, Fondos, Pantalla y Teléfono, con la cuenta como primera fila y un pliegue «Más
opciones» por bloque. El buscador filtra por el nombre de la fila sin mirar mayúsculas ni
acentos; si nada coincide dice «Ningún ajuste se llama así». La calidad se elige en tres palabras
(Automática, Ahorrar batería, Máxima) y los siete niveles quedan tras «Más opciones»; un nivel
fijo manda.

La fila «Fundido entre fondos» (`transition.kind`) explica qué hace la elección en este teléfono:
de golpe si está apagada o si el movimiento está reducido (`transition.less_motion` o «quitar
animaciones» del sistema), con un fundido para `fade`, con la transición propia del fondo para
`own`, y con una calidad fija baja solo las transiciones livianas. La fila no dice nada del ahorro
de batería: las páginas no cuentan por qué un fondo está quieto.

## Modo privado

Ajustes empieza con el bloque Privacidad y su única fila, «Modo privado». Está apagado por omisión y queda guardado en `settings.toml` (`private_mode`), así que sobrevive a reinicios.

Encendido corta toda la red del teléfono:

- No se actualizan repositorios (ni a mano ni el trabajo de fondo), no se agrega un repositorio por dirección y no se instala nada de uno remoto.
- No se descargan carátulas ni vistas previas, ni modelos de profundidad, y las fuentes de fondos (imagen del día, WebDAV, Immich…) no consultan nada.
- Los fondos web solo cargan sus propios archivos: la vista bloquea la red y la biblioteca nativa contesta vacío a cualquier origen permitido.
- No se habla con tu servidor ni con tu computadora: no se inicia sesión, no se vota, no se escucha la red local ni se buscan computadoras cercanas.

Sigue funcionando todo lo local: la biblioteca, los fondos instalados, las listas, la rotación, los ajustes y los repositorios incluidos en la app.

Las pantallas que usan la red (Explorar y repositorios, el formulario de repositorio, la cuenta y las actualizaciones) muestran arriba una línea tranquila, «Modo privado: la red está cortada.», con el botón «Desactivar», y sus botones de red quedan apagados. No es el aviso de «Sin conexión»: no hay error, solo el corte elegido.

Cada lenguaje tiene una sola puerta (`net.rs` en la biblioteca nativa, `Net.kt` en la app) que se niega antes de resolver ningún nombre, y una prueba recorre el código para que ninguna conexión se abra fuera de ella. `PrivateModeTest` y `PrivatePageTest` en el emulador, y la prueba nativa del oyente local, comprueban que con el modo encendido un oyente en 127.0.0.1 no recibe ninguna conexión y que con el modo apagado sí la recibe.

En Explorar, cada paquete instalado desde un repositorio lleva «Desinstalar», que lo quita de la
biblioteca y borra su registro de instalación: el repositorio vuelve a ofrecerlo con «Instalar».
Quitar un fondo instalado desde la biblioteca hace lo mismo con su registro.
