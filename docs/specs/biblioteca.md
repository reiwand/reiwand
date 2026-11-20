# Biblioteca de la app de escritorio

La Biblioteca muestra los fondos guardados sobre la imagen del fondo en uso.

## Estante

- Una fila de tapas abajo, sobre la imagen del fondo en uso; la tapa con foco está encendida y bajo ella
  aparece su ficha: nombre, pie (tipo y tamaño) y las acciones.
- Las tapas usan la miniatura que cubre la tapa encendida por la escala de la pantalla (512 px a
  escala 1, 1024 px a escala 2): cada punto de la pantalla tiene un punto propio de la imagen, salvo
  que el fondo o la tapa de su paquete sean más chicos. Las miniaturas guardadas de otro tamaño no se
  borran: el servicio hace las nuevas en segundo plano y las viejas salen por el límite de la caché.
- Una escena empaquetada cuya tapa es más chica que la miniatura pedida, de cualquier tamaño, se dibuja:
  el servicio pinta un cuadro fijo de la escena en su momento de póster, al tamaño pedido y nunca más
  grande que la escena, sin puntero, sonido ni sensores y sin red. Lo hace en segundo plano, de a una
  escena por vez, mientras se ve la tapa; no dibuja con la reproducción en pausa, la sesión bloqueada,
  la pausa por batería ni con la calidad fijada en Q0 o Q1, y entonces usa la tapa. Si la escena no se
  puede dibujar, el registro del servicio lo dice con el id del elemento y el motivo. Una tapa mucho
  más chica que la tarjeta se muestra a su tamaño, centrada sobre el fondo quieto de la tarjeta, sin
  estirarse. Las páginas web y los ajustes empaquetados usan siempre su tapa.
- Con el puntero quieto unos 300 ms sobre la tapa de una escena, una página web o un video (en el estante, en la tapa
  encendida y en «Ver todos»), la tapa muestra el fondo en marcha, dibujada por el servicio en una transmisión
  `preview:` del tamaño de la tapa (`RequestStream`, `Library1.Apply` sobre esa salida y
  `ReleaseStream`, lo mismo que `reiwand-ctl stream`, `reiwand-ctl lib apply --output` y `reiwand-ctl
  release`). Dura mientras el puntero está encima; al salir, al desplazar la fila, al abrir otra
  página o al perder el foco la ventana se suelta la transmisión y vuelve la tapa. Barrer el estante
  con el puntero no arranca nada y nunca hay más de una a la vez. No cambia los fondos de las
  pantallas ni se anota en los ajustes ni en el historial, usa la calidad que el usuario fijó, y no
  arranca para una escena con destellos que el usuario no permitió, una oculta por contenido maduro,
  una dañada o una que pide una versión más nueva: esas quedan en su tapa, sin aviso. Con el teclado
  la tapa encendida sigue mostrando su tapa. Las imágenes no cambian.
  La tapa en marcha siempre va sin sonido: el servicio no abre ninguna salida de audio para la
  transmisión de una tapa (`preview:card-…`) y la página web de una tapa suena a volumen 0, aunque la
  escena tenga capas de sonido o videos con audio o la página reproduzca sonido, y el sonido del fondo de las pantallas sigue igual.
- «Poner de fondo» (Intro) lo pone enseguida, sin diálogo. Con dos o más pantallas dice «Poner de fondo
  en …» y lo pone en la pantalla elegida (la primera, mientras no haya otra elección); un fondo que se
  extiende por todas las pantallas se pone extendido. Un aviso dice dónde quedó.
- «Opciones ›» abre el panel del fondo.
- «Editar una copia…» (solo en una escena guardada como paquete) abre el paso que la copia como un
  proyecto nuevo, también si la escena es tuya: el original no cambia nunca. Para seguir con un
  proyecto propio se abre desde «Crear».
- Clic derecho o la tecla de menú sobre una tapa (estante o rejilla) abren un menú con lo mismo:
  «Poner de fondo» (o «Mostrar en la carpeta»), «Opciones» y «Editar una copia…». En Explorar el menú
  ofrece la acción principal de la tarjeta, «Detalles» y «Editar una copia…» (escena instalada);
  este último también es un botón de la tarjeta encendida.
- La tapa del fondo en uso lleva solo un ícono chico, sin texto ni marco alrededor; su nombre para
  lectores de pantalla y la pista al pasar el puntero dicen «En uso» o, con dos o más pantallas,
  «En …» con el nombre de cada pantalla donde está. El filete de foco es aparte: marca la tapa con el
  foco del teclado.
- Una nota en la tapa avisa de un archivo que falta o está dañado, de un fondo retirado por su repositorio
  o de uno con destellos (apagado hasta que el usuario lo permita).
- Mientras el escritorio crea las capas de profundidad de una imagen en pantalla, su tapa lo dice en
  todos los listados: «Esperando para generar las capas» en la cola y «Generando capas · N %» con una
  barra fina en el borde de abajo mientras avanza (con LaMa avanza por cada recorte que rellena).
  Si no se pudieron generar, la nota sale en tono de error y el menú de la tapa (también con la tecla
  de menú) ofrece «Volver a intentar» (`RetryLayers`, `reiwand-ctl retry-layers RUTA`); lo mismo si el
  usuario canceló la creación. Al terminar la nota desaparece. La tarjeta de la pantalla en «Ahora»
  muestra el mismo estado y la misma barra, y la línea de tareas ya no lista estas creaciones. Una
  creación cancelada no vuelve a empezar sola: solo al aplicar la imagen otra vez, al cambiar un
  ajuste de profundidad o de paralaje que cambie el juego de capas, o con «Volver a intentar»; el
  estado cancelado vive solo en memoria y se olvida al reiniciar el servicio.
- La barra de arriba filtra por Todos, Escenas, Imágenes, Vídeos, Web, De fuentes y Favoritos; cada
  palabra lleva su número de fondos y solo se ven las clases que tienen alguno (Todos y Favoritos
  siempre, también en 0). «De fuentes» son las fotos que trajo una fuente de fotos; «Imágenes» son
  las demás imágenes, así una fuente con cientos de fotos no tapa lo propio. «Más» abre Paquetes,
  Recursos, las etiquetas y «Renombrar la etiqueta…». El buscador busca dentro del filtro elegido:
  dice «Buscar en Escenas» y, al escribir, «3 de 34» junto al campo. El filtro elegido se mantiene al
  ir a otra página y volver. Las respuestas de una búsqueda nunca le quitan el foco al buscador. El
  buscador acepta `tag:`, `kind:`, `fav:`, `provider:` y `sort:` (también `reiwand-ctl lib search`).
- Marcar un favorito (en la tarjeta, en sus opciones o en el fondo en uso) avisa «Añadido a tus
  favoritos.» con «Ver favoritos», que abre la biblioteca con ese filtro. El filtro muestra todo lo
  marcado, también las fotos de las fuentes; vacío dice cómo marcar uno.
- La rueda del ratón sobre la franja de la biblioteca (línea de avisos, barra, tarjetas y botones
  de la tarjeta encendida) mueve la tarjeta encendida un fondo por muesca, como las flechas, y se
  detiene en los extremos; un panel táctil suma su recorrido hasta una muesca. Vale en todos los
  estantes; la rejilla de «Ver todos» sigue desplazándose. Mientras gira no arranca la tapa viva.
- Cada fondo lleva su título traducido; a falta de título, una foto de una fuente lleva «Foto de
  <fuente>», el resto lleva el nombre del archivo sin extensión y, a falta de todo, «Fondo sin
  título». Nunca se muestra un identificador, tampoco en las tapas de la biblioteca.
- En Explorar, la tarjeta de un fondo instalado usa la misma tapa, el mismo tamaño y la misma tapa
  viva que la biblioteca; la de uno sin instalar muestra la vista previa del repositorio.
- «Añadir fondos…» abre el selector de archivos del escritorio y después el panel «Añadir N fondos», que pregunta si
  copiarlos a la biblioteca o dejarlos donde están. «Escribir las rutas…» cambia la lista por un campo
  con una ruta por línea. Al terminar, el aviso «Se añadieron N fondos.» con «Ver» vuelve al estante en
  Todos, donde los nuevos quedan primero. Soltar archivos sobre la biblioteca los añade sin copiarlos,
  con el mismo aviso.
- El velo de abajo sube hasta detrás de la barra, para que las palabras se lean también sobre un fondo
  claro; su opacidad sigue los tonos que la app saca de la imagen.
- Teclas: ← → para moverse, ↑ lleva a los filtros de la barra (← → entre palabras, Intro elige, ↓
  vuelve a las tapas), ↓ o «Ver todos» muestra la rejilla, Mayús con una flecha (o Mayús+clic)
  elige los fondos entre el anterior y el nuevo, Ctrl+A elige todos, Esc deja de elegir.
- Al llegar a la biblioteca (al abrir la app, desde el menú, otra página, «Opciones» o un diálogo) las
  tapas ya tienen el foco, así que las teclas de abajo a la derecha andan sin un clic. Tab pasa de la
  tapa encendida a sus botones («Poner de fondo», «Opciones») y Mayús+Tab vuelve a las tapas; mientras el
  foco está en esos botones la barra muestra solo Tab y Esc. Intro pone el fondo y Espacio empieza a elegir.

## Ver todos

La misma colección en una rejilla con un velo parejo sobre la imagen; la ficha del fondo con foco queda
fija abajo. Primero va lo que está puesto ahora: el fondo o los fondos en uso en cada pantalla
(con su ícono), después, si hay una lista en marcha, sus fondos en el orden de la lista (cada uno con
el nombre de la lista) y luego el resto en el orden de siempre, sin líneas ni marcos entre los grupos. Un filtro o
una búsqueda mantienen esa regla dentro de sus resultados. La rejilla llega con el foco en el fondo
en uso. El estante conserva su orden. En la terminal, `reiwand-ctl lib search 'first:<uid>,<uid>'` da
el mismo orden. Esc o «Ver el estante» vuelve al estante.

## Varios elegidos

Ctrl+clic, el círculo que aparece al pasar sobre una tapa o Espacio eligen fondos. «Elegir fondos en la
biblioteca» (en «Listas») abre «Ver todos» ya eligiendo, con un círculo en cada tapa; desde «Fondos» de
una lista, «Añadir fondos…» hace lo mismo y suma el botón «Añadir a «lista»», que vuelve a la lista con
los elegidos añadidos. En lugar de la ficha aparece la línea de la selección:
«N elegidos» («1 elegido» con uno), «Rotar estos cada…», «Lista», «Etiquetas», «Más» (favorito,
valoración, permitir los destellos, enviar los archivos a los otros equipos, quitar) y «Dejar de elegir»
(Esc). Cada acción abre un panel chico junto a su botón: «Rotar N fondos» con 10 min, 30 min, 1 h, Cada
día, «Otro…» (segundos, minutos u horas) y «Usar una carpeta en su lugar…»; «Lista» con las listas
hechas a mano y «Una lista nueva…»; «Etiquetas» con las que se añaden y las que se quitan; y las
preguntas de valorar, permitir los destellos y quitar (con «Eliminar también las copias importadas»).
Con varias pantallas la rotación arranca en la pantalla elegida.

## Archivo dañado o que falta

La ficha dice en Coral que no se pudo abrir el archivo, con la palabra de su clase (imagen, vídeo,
escena o fondo), y ofrece «Mostrar en la carpeta» (Intro), «Opciones ›», que abre el panel de la
escena dañada, y «Quitar de la biblioteca…», que pregunta antes de quitar; los detalles muestran el
error.

## Sin resultados y fallos

Una búsqueda sin resultados dice «Nada coincide con «…»» y ofrece «Quitar los filtros».

Un fondo importado cuyo archivo no está en este equipo muestra un aviso que dice cuál es, de
dónde viene (una lista, un perfil o un repositorio importados) y qué hacer; si viene de un
repositorio, ofrece «Añadir el repositorio». Cerrarlo lo oculta para ese fondo.

## Primer arranque

Con la biblioteca vacía la página no muestra la barra de filtros: dice «Aún no tienes fondos» y ofrece
tres caminos, «Explorar repositorios» (principal), «Añadir una fuente» y «Añadir una carpeta», con los
ejemplos incluidos (los del repositorio `bundled`) en una fila debajo; pulsar uno lo instala y lo pone de
fondo. Soltar archivos o una carpeta sobre la página muestra «Suelta para añadirlos a tu biblioteca».

El primer fondo se pone solo: cuando la biblioteca está vacía, ninguna pantalla muestra un fondo y el
repositorio de ejemplos ya está, la app instala el primero que no pide nada (ni entradas ni permisos) y
no parpadea, lo aplica a todas las pantallas con `Apply` y lo dice en un aviso con «Deshacer». Deshacer
llama a `ClearWallpaper` y deja el escritorio como estaba. Sucede una sola vez: el archivo
`reiwand/first-run` de la carpeta de configuración lo recuerda, aunque la biblioteca se vacíe después.
Si el repositorio de ejemplos no está (instalación sin ejemplos) solo se ven los tres caminos.

Si el servicio no responde, la página muestra solo el fallo, «El servicio de fondos no responde», con
«Reiniciar el servicio» y los detalles; reintenta cada tres segundos y vuelve sola cuando el servicio
contesta. El botón pide a systemd de la sesión que reinicie `reiwandd.service` (como el applet de la bandeja);
sin esa unidad pide al bus que active `io.github.reiwand.Daemon`. Si ninguna de las dos funciona, el
texto lo dice y el motivo queda en «Detalles». En la app es la llamada `Service.Restart` de su tabla.

## Opciones de un fondo

Usa el esqueleto de opciones: el título es el nombre del fondo, con «Favorito» y «Poner de fondo»
arriba, y la ruta «Biblioteca › nombre» (Esc vuelve). El primer nivel muestra la vista previa, por
qué la escena está quieta si lo está, «Opciones del autor» (las de la escena en cada pantalla donde
está puesta), «Mis preajustes», lo que el paquete usa, «De su autora» (variantes y fondo base),
«Tuyo» (etiquetas, listas) y «Más». Una escena puesta sin opciones propias dice «Esta escena no
tiene opciones propias» y deja «Mis preajustes» y «Más».

En una imagen, «Tuyo» empieza por «Profundidad · Moverse con el puntero»: mientras el modelo de
profundidad no está instalado, la fila dice cuánto se descarga. Encenderla guarda en la edición de
la imagen el paralaje que sigue al puntero (lo mismo que `reiwand-ctl edit parallax --source pointer`);
si falta el modelo pide permiso con el mismo cuadro del editor (cancelarlo la vuelve a apagar) y,
con el modelo listo, calcula el mapa de profundidad en este equipo. Apagarla quita el paralaje y
conserva sus valores. Los ajustes finos siguen en Editar › Paralaje.
Mientras una imagen así está en una pantalla, Ajustes › Privacidad la lista entre lo que pueden usar
las escenas, con su nombre y «Puntero», y «Dejar de usar Puntero» apaga ese paralaje (lo mismo que
`reiwand-ctl inputs revoke RUTA pointer`; `reiwand-ctl inputs status` la lista igual). Volver a encender el
interruptor no pregunta nada.

El nivel «Más» (ruta «Biblioteca › nombre › Más») empieza por «Permisos de esta escena»: una fila por
cada cosa que la escena pide (puntero, sonido, clics, lo que suena y sus variantes con red), con
«Permitido» o «Sin permiso»; cambiarla responde con `Inputs1.Decide`. Siguen «Esta escena» (sonido,
«Calidad de esta escena · La de tu pantalla», «Editar una copia…»), el nombre, la valoración, los
datos del archivo, «Seguir moviéndose» con movimiento reducido, la privacidad de una página web y
sus luces, «Editar…» y «Quitar de la biblioteca…».

«Calidad de esta escena» no fija una calidad propia: la calidad se fija por pantalla. La fila
aparece mientras la escena está en una pantalla y dice, con las palabras de Ajustes, la calidad de
esa pantalla (`playback.quality.<pantalla>` o, si no tiene, `playback.quality`) y dónde se cambia;
Intro abre Ajustes.

## Mis preajustes

El nivel «Mis preajustes» (ruta «Biblioteca › nombre › Mis preajustes») lista los preajustes del
fondo y los globales (`Profiles1.ListPresets`), con su colocación y su relleno. Intro usa el
preajuste en ese fondo (`Library1.Apply` con el preajuste); «Eliminar…» pide confirmación en un
panel junto a la fila. «Nuevo preajuste…» abre un panel con «Nombre», «Colocación» y «Color de
relleno»; Tab pasa al campo siguiente, Esc cancela y «Guardar» crea un preajuste global
(`Profiles1.SavePreset` con elemento vacío).

## Pregunta de permisos

Cuando una pantalla muestra una escena que pide cosas que la persona todavía no respondió (el campo
`undecided` de `GetInputs().shown`), la app hace una sola pregunta, «“nombre” pide permiso para usar»,
con una línea por cosa (puntero, sonido del equipo, clics y toques, lo que suena, y lo que suena o el
sonido con red), su explicación y «Permitir» o «Sin esto» en cada una. «Permitir» al pie aplica lo
elegido y «Sin esto» deja todo afuera, con `Inputs1.Decide`; la escena ya funciona mientras tanto sin
esos permisos, y si algo quedó afuera un aviso dice dónde cambiarlo. La red y las luces no se
preguntan acá. Una vista previa (la tapa en marcha, la ventana de vista previa o el editor, salidas
`preview:`) no pregunta: la escena se ve sin esos permisos hasta que se pone de fondo en una pantalla. Se pregunta una vez por escena en cada sesión de la app; lo respondido se recuerda en
el servicio. La pieza es `PreguntaPermisos` y otras pantallas la usan con `ask(escena, clave,
nombre, cosas)`.

## Quieta en su póster

El panel dice que la escena puesta está quieta solo en dos casos: es
demasiado pesada para el equipo con la calidad automática (`quality_auto` = `still`). En este último
caso ofrece «Reproducir igual», que fija `playback.quality.<pantalla>` en `min-q1`, y además la app
muestra una vez un aviso con el mismo botón. Un fondo con destellos mientras están apagados en
Ajustes (`flash_disabled`) también lo dice; el ajuste general solo se cambia en Ajustes.
Una pausa por una ventana que tapa la pantalla, por batería, por pantalla bloqueada o a mano no se
muestra en la app: su motivo lo da solo la terminal (`reiwand-ctl state`).

## Escena dañada

Si el archivo falta o está dañado, el panel muestra «No se pudo abrir esta escena» con el motivo,
«Volver a instalarla» cuando vino de un repositorio, «Mostrar en la carpeta» y «Quitar de la
biblioteca…» con confirmación. Volver a instalar la misma versión comprueba el archivo contra el
tamaño y el hash que se anotaron al instalarlo: si falta o no coincide, lo descarga otra vez del
repositorio con las mismas comprobaciones de firma y de listado que la primera instalación y
conserva su registro y sus ajustes; si está entero, no descarga nada.
