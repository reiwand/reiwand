# Editor de escenas

La página *Escenas* de la app crea una escena a partir de un punto de partida (vacía, partículas,
paralaje por capas o reloj de texto), abre una carpeta de escena o edita una copia de una escena de
la biblioteca, en una carpeta nueva dentro de la de documentos (`reiwand/<título>`) o en la
que elijas. Una escena en edición es una carpeta con `manifest.json`, `scene.json` y sus recursos
en `assets/`: el editor no tiene un formato propio y cambia los mismos archivos que lee el motor.
Las escenas nuevas usan la segunda versión del manifiesto y nacen válidas: para todos los públicos,
sin destellos, obra original y con las entradas que ya usan declaradas, hasta que cambies esos
datos. `reiwand-app --page scenes` abre esa página y, como con el editor de imágenes, una petición de
otra página no cierra un editor con cambios sin guardar.

- **Capas**: un árbol con todos los tipos de capa (imagen, vídeo, texto, forma, color, espectro,
  partículas, sonido, grupo y ajuste) para añadir, quitar, duplicar, subir y bajar, agrupar (Ctrl y
  clic eligen varias), meter en un grupo o sacar de él, ocultar (o silenciar un sonido), bloquear
  y dejar *solo* una capa en la vista previa. El asa de cada fila la arrastra a otro sitio de la
  lista, también dentro de otro grupo o fuera de él (la misma acción que `reiwand-ctl scene
  move-layer`).
- **Lienzo**: la vista previa la dibuja el propio daemon en una salida virtual que llega por
  PipeWire, como el fondo de Plasma, así que necesita el módulo QML de KPipeWire. Ctrl y la rueda
  amplían, la rueda o un arrastre con el botón central o el derecho desplazan. Una capa sin
  bloquear se arrastra directamente, y la elegida gira, escala, se inclina en profundidad (asa
  redonda del borde izquierdo, `transform.tilt`) y se acerca o aleja (rombo de abajo,
  `transform.z`) con sus asas, y una capa inclinada o alejada muestra en línea discontinua el
  contorno con la misma perspectiva con que la dibuja la escena; las asas escriben con la acción
  `layer` como `reiwand-ctl scene act CARPETA layer`; en
  un valor animado ponen una clave en el cabezal (`--at S`), también en cada parte de una posición
  o escala animada por separado, y una parte que sigue un enlace rechaza el arrastre; con
  *Escribir sobre las claves* de la barra
  dejan un valor fijo encima de las claves o enlaces (`--replace`); un arrastre que
  empieza fuera de las capas, o cualquiera con Mayúsculas, traza un rectángulo que elige las capas
  que toca, como Ctrl en la lista, listas para agruparlas. Las guías marcan lo que se ve en pantallas 16:9, 21:9, 32:9,
  4:3, 16:10 y 9:16. *Pantallas* muestra la escena extendida sobre dos o tres pantallas en fila,
  dos apiladas o las tuyas, con el marco de cada una y oscurecido lo que tapan los marcos entre
  ellas, y *Extender sobre todas las pantallas* lo pide en el manifiesto. *Ver en el escritorio* lleva la escena sin guardar a todas las pantallas
  sin aplicarla; se quita al desactivarlo o al cerrar el editor, y como mucho un minuto después si
  la app se cierra de golpe.
- **Inspector**: se genera a partir del esquema JSON de las escenas (`reiwand-ctl scene schema`), así
  que los campos nuevos del formato aparecen solos: números con deslizador y unidad, colores,
  opciones, vectores, curvas (un clic añade un punto, se arrastra y un doble clic lo quita),
  recursos, efectos con sus parámetros y la vuelta al valor por defecto de cada campo opcional. Los nombres de los campos y de sus valores (modos de fusión,
  formas, curvas de interpolación, efectos, tipos de capa y de recurso, idiomas, *Igual que la
  pantalla* de los fotogramas por segundo) salen en el idioma del escritorio, y una prueba recorre
  el inspector de cada punto de partida con todas sus variantes y falla si algo de lo que muestra
  no tiene traducción al español. Las capas y opciones nuevas que crea la app llevan nombres y
  textos en ese idioma. El rombo de cada valor animable pone o quita un fotograma clave en el
  instante actual o lo enlaza a una propiedad de usuario, al sonido o al puntero, y *Editar el
  origen* ajusta la línea de tiempo, la escala o el suavizado de ese enlace. Encima de los campos,
  una capa ofrece su modo de fusión, el lienzo interruptores y deslizadores para el brillo y el
  paralaje (cambian la escena en vivo al arrastrarlos y cada arrastre es un solo paso de
  deshacer, aunque se detenga a mitad; así con todos los deslizadores del editor, también los de
  rango, y con las asas del lienzo; deshacer a mitad de un arrastre deja el resto como un paso
  propio), y una propiedad deslizante su mínimo, máximo, paso y valor por defecto; *Importar una
  hoja de sprites o un mapa de datos…* del panel de recursos corta una imagen en filas y columnas
  o la lee como datos. En la terminal hacen lo mismo `reiwand-ctl scene act CARPETA
  effects`, `scene act CARPETA layer`, `scene act CARPETA manifest`, `scene act CARPETA property`,
  `scene act CARPETA canvas` y `scene import --linear --atlas FILAS,COLUMNAS`, con las mismas comprobaciones.
- **Línea de tiempo**: reproducir, pausar y mover el cabezal (la vista previa se queda en ese
  instante), una pista por valor animado con sus fotogramas clave, que se arrastran a otro
  instante, la interpolación de cada uno y la línea de tiempo que la mueve; *Curva y base de la
  clave* arrastra en vivo las dos asas Bézier de la curva hasta la clave siguiente, un solo paso de
  deshacer por arrastre aunque se detenga a mitad, también en posiciones, colores y listas de
  números, donde la altura del asa es la parte del camino entre las dos claves (salvo entre dos claves
  escritas como un solo número, como una escala `2`, que se curvan en unidades del valor, igual en la
  vista previa que en el fondo); y la lista de líneas de
  tiempo para crearlas, renombrarlas, quitarlas y editarlas: *Editar la línea de tiempo y sus
  eventos…* abre su panel con la duración en segundos o en cuadros y cuadros por segundo, cómo se
  repite, el fundido del final con el principio, la velocidad (deslizador y campo), el retraso, si
  arranca sola y qué la arranca, y una franja de eventos con nombre: un doble clic añade uno en ese
  instante, se arrastran a otro, se renombran y se quitan (`reiwand-ctl scene act CARPETA timeline`
  con `add_events`, `drop_events` y `clear_events`).
- **Paquete**: *Detalles del paquete* lleva el manifiesto al inspector, que también se genera a
  partir de su esquema (`reiwand-ctl scene schema --manifest`): título, descripción y notas de esta
  versión en cada idioma (*Traducir* añade uno a partir del texto del idioma por defecto sin tocar
  los que ya hay, y los textos `@clave` se escriben en `strings`), versión, id, autores (hasta
  64), licencia (una expresión SPDX de hasta 256 caracteres), clasificación (para todos o para
  adultos, con sus motivos), si destella, origen y obras de las que
  procede, créditos por archivo, miniatura y vista previa animada (*Elegir…* copia la imagen a
  `assets/` y reduce la miniatura si pasa de 1024 píxeles o de 1 MiB), enlaces, etiquetas, idioma
  por defecto y las entradas que puede pedir. Las entradas que usa la escena se marcan solas y no se
  pueden quitar, y `requires.features` se calcula en cada cambio como en `reiwand-ctl pack build`:
  nunca se escribe a mano; las entradas que añadió el editor se van cuando nada las usa. Como
  `reiwand-ctl pack check`, *Problemas* dice si el escritorio y el teléfono la rechazan o la reproducen
  sin alguna función. Una escena
  nueva toma como idioma por defecto el del escritorio.
- **Propiedades y preajustes**: *Propiedades de usuario* las lista en el orden del formulario, cada
  grupo con sus miembros debajo, y sigue con los preajustes en la misma lista. Se añaden de
  cualquier tipo (deslizador, interruptor, color, selección, texto, archivo, carpeta, botón, grupo,
  etiqueta o separador) sin texto en ningún idioma, así que muestran su id hasta que les pones una
  etiqueta; se suben y bajan, se meten en un grupo o se sacan (los grupos no se anidan), se
  renombran (los enlaces, disparadores, grupos, condiciones y preajustes que las nombran cambian con
  ellas; los scripts no, y se avisa de los que la nombran) y se quitan junto con sus valores en los
  preajustes y en las condiciones de mapa, salvo mientras un enlace, un disparador o una expresión
  las nombra: entonces se dice dónde. El inspector edita cada campo de su tipo: paso, unidad,
  decimales, casilla para el número y escala logarítmica de un deslizador, valores, etiquetas e
  iconos de las opciones, color con transparencia, textos secretos, grupos plegados, archivos que
  acepta y recurso por defecto, carpetas con su orden y su intervalo, icono y orden, y muestra los
  textos `"@clave"` con los de `strings`. `visible_if` y `enabled_if` se escriben como una expresión
  del lenguaje de las condiciones o como un mapa JSON y se comprueban mientras escribes; solo se
  guarda una condición válida, y un problema de una condición o de un preajuste lleva a su campo.
  *Preajustes* lista los del paquete y *Añadir un preajuste* crea otro, con una fila por cada
  propiedad que pueden fijar. Solo las propiedades con valor se pueden enlazar, y el disparador de
  una línea de tiempo se elige entre `start`, `beat`, los botones y esas propiedades. Un valor que
  sigue a una selección o a un interruptor (la imagen de una capa, una máscara, la textura de unas
  partículas, un vídeo o un color) muestra la propiedad que lo guía y una fila por opción para
  elegir su variante. `requires.features` sigue a los tipos y campos nuevos que se usan.
- **Recursos**: importa imágenes, vídeos, sonidos, fuentes y tablas `.cube` dentro de los límites
  del formato y borra los archivos que la escena ya no usa. *Importar un efecto…* (`reiwand-ctl scene
  import-effect`) copia un efecto propio, su descriptor y su shader WGSL (junto a él o elegido aparte,
  como `--wgsl`), a `effects/`, lo comprueba como al leer el paquete y dice qué cambiar si no pasa;
  después se añade a las capas como los integrados, con su nombre en tu idioma, y sus parámetros
  salen en el inspector con su etiqueta y su ayuda en tu idioma, sus límites, las imágenes o capas
  que pueden leer y sus variantes, enlazables a propiedades, fotogramas clave o scripts.
  *Importar un script…* (`reiwand-ctl scene act CARPETA import_script`) copia un módulo `.js` a `scripts/` y
  propone su declaración: los eventos que exporta o atiende con `on` y las partes de `reiwand` que
  importa o llama; conviene revisarla antes de publicar. Al importar otra vez el mismo módulo,
  conserva sus `params` y dice qué cambió en la declaración. *Convertir a KTX2…*, en cada imagen fija,
  la pasa a una textura KTX2 con Basis Universal (UASTC, más nítida, o ETC1S, más pequeña) con un
  control de calidad: muestra el tamaño del archivo antes y después, la memoria que ocupa en cada
  clase de tarjeta y un recorte de cómo la dibuja cada una (BC7 en escritorio, ASTC en teléfonos,
  ETC2 en teléfonos viejos y sin comprimir en las demás). Es un paso que se deshace; `reiwand-ctl scene
  asset DIR ID` muestra lo mismo y `--ktx2 [--codec uastc|etc1s] [--quality N]` convierte, o solo
  informa con `--dry-run [--looks CARPETA]`. Al cargar, la escena pasa cada textura KTX2 al formato
  comprimido que lee la tarjeta y la deja así en memoria, con los niveles del archivo como mipmaps;
  la imagen conserva su tamaño exacto aunque un lado no sea múltiplo de 4 píxeles, porque la textura
  se rellena hasta el bloque siguiente y solo se dibujan sus píxeles reales, también cuando la imagen
  es una máscara, un mapa de luz, un reflejo o una proyección. Una imagen animada
  (GIF o WebP) se convierte en un arreglo de cuadros que conserva el tiempo de cada cuadro; una imagen
  HDR (`.hdr` o `.exr`) en una textura HDR que conserva su luz por encima del blanco; y un panorama, o
  seis imágenes cuadradas elegidas una por cara, en un mapa cúbico (`--cube` o `--faces` en
  `reiwand-ctl scene asset`). Cada clase de tarjeta que guarda una forma sin comprimir lo indica. También se leen KTX2
  HDR (BC6H en escritorio, ASTC HDR donde la tarjeta lo lee y medios flotantes en las demás), que
  ganan rango en lienzos `hdr`; arreglos de cuadros, que se ven como hojas de sprites y se animan
  con `animated`; y mapas cúbicos, que se ven como panoramas equirectangulares. El panel de
  recursos indica en cada KTX2 si es HDR, cuántos cuadros tiene o si es un cubo, y `reiwand-ctl scene
  asset` lo dice igual. Los recortes de cada clase de tarjeta los dibuja la propia tarjeta gráfica
  cuando lee ese formato y el procesador los decodifica cuando no, y cada recorte dice cuál fue. El
  teléfono no convierte a KTX2: no tiene editor de escenas y solo lleva el transcodificador en Rust,
  no el codificador en C++ de Basis Universal; los paquetes hechos en el escritorio se ven igual en
  él.
- **Problemas**: los mensajes del validador cambian con cada edición; al pulsar uno se elige la
  capa o el campo afectado. Esos mensajes y los rechazos del editor llevan un identificador con sus
  argumentos (`scene.color`, `editor.step.add_layer`…) y salen en el idioma del escritorio en la
  app y en `reiwand-ctl`, y en el del teléfono cuando este muestra el mismo error; las capas de los
  puntos de partida (`sky`, `sun`, `flakes`…) y los pasos de deshacer se muestran traducidos sin
  cambiar los ids guardados. Un punto de partida creado en un idioma que sus etiquetas no tienen
  usa ese idioma, el del título, como idioma predeterminado y copia en él las etiquetas en inglés
  para traducirlas. Debajo, la consola de los scripts muestra las últimas 50 líneas que
  escribieron en la vista previa (`reiwand-ctl state` las da en `console` de `previews`, bajo el nombre de la salida `preview:`).

El editor de escenas pinta también las máscaras de los efectos que las piden por canal: junto a
cada parámetro de máscara hay un botón para pintar, el canal (rojo, verde, azul o alfa), un modo
para borrar y el tamaño del pincel, y cada trazo sobre la capa en la vista previa se guarda como una
imagen nueva en `masks/`, de modo que deshacer vuelve a la anterior (`reiwand-ctl scene act CARPETA paint_mask`
hace lo mismo desde la terminal). Si la máscara, el mapa o la imagen de partida es KTX2, la imagen
nueva también lo es, con el mismo códec y la misma calidad, en lugar de un PNG. Mientras la app
sigue abierta, cada trazo parte de una copia exacta del trazo anterior, así que la pérdida de la
compresión no se acumula trazo a trazo; desde la terminal cada llamada parte del archivo, así que ahí
conviene pintar sobre PNG y convertir al final.
Al elegir un efecto de una capa, el inspector ofrece su máscara:
ninguna, una imagen, otra capa o una forma (rectángulo, elipse, anillo, polígono, estrella o línea),
el canal, invertir, el borde suave (con un deslizador de hasta 200 px y un campo de hasta 10 000) y
la intensidad, y el lienzo la dibuja sobre la capa: la imagen encima (también una KTX2, que la app
descomprime una vez a un PNG en su caché), la capa que enmascara
recuadrada y la forma con su contorno, el borde suave por dentro y por fuera en trazos, un asa
para darle tamaño (o mover el extremo de la línea) y un punto junto al borde para suavizarlo; cada
arrastre es un paso de deshacer y sigue la acción `effects mask`, como `reiwand-ctl scene act
CARPETA effects`.

También recorta una malla de una imagen de la escena («Malla desde una imagen» en el menú para añadir
capas), con una cuadrícula de celdas o siguiendo el contorno de la parte opaca, y le pone huesos, igual
que a un `.glb` importado de varias partes. El inspector del campo `mesh` muestra los huesos en árbol:
un clic elige un hueso (también sobre la vista previa), un doble clic lo renombra (y la escena lo
sigue en clips, cadenas, física, reglas, puntero y capas sujetas), y un hueso soltado sobre otro
cuelga de él; al quitar un hueso o colgarlo de otro, en el mismo paso se quitan las claves, cadenas,
cuerpos de física, reglas, huesos de `hit` y capas sujetas que lo nombraban o que dejaron de
encadenarse, de modo que la escena sigue siendo válida. Sobre la vista previa los huesos se arrastran, se pinta la profundidad de los
vértices, que los acerca o aleja del espectador, y se pintan los pesos del hueso elegido con un
pincel que suma, resta, suaviza o normaliza, con fuerza, tamaño, borde suave y huesos bloqueados;
la malla se colorea por el peso de ese hueso o por el hueso que manda en cada parte, y cada vértice
queda con cuatro huesos como mucho y pesos que suman 1. Mientras no se pintan, los pesos siguen a
los huesos más cercanos; una vez pintados se conservan al cambiar los huesos, y *Pesos automáticos*
los vuelve a calcular. Cada cambio guarda una malla nueva en `meshes/`, de modo que deshacer vuelve a
la anterior; al guardar, las mallas anteriores esperan fuera de la carpeta, junto al historial del
editor, y vuelven cuando un paso las nombra, así que el paquete lleva solo las mallas en uso.

*Editar los clips en la línea de tiempo* cambia las líneas de tiempo bajo la vista previa por los
clips de la capa: se elige, crea, renombra o quita un clip, se fijan su duración y su bucle, y el
cabezal recorre, reproduce y repite la vista previa con ese clip, en segundos desde el comienzo del
clip (su `from`), donde se escriben también las claves. Hay filas por canal de cada hueso,
por el orden de las partes y por cada forma de mezcla, con un deslizador que guarda su peso; las
claves se arrastran (con Ctrl se copian), se borran y se les da paso, línea o curva suave. Sobre la
vista previa el hueso elegido tiene asas para moverlo, girarlo y escalarlo en ese instante, y
*Clave* guarda en las claves del clip del JSON de la escena, las mismas que reproduce el motor, solo
los canales que cambiaron las asas, o la pose entera del hueso (con el movimiento del archivo) si no
cambió ninguno. Las formas de mezcla tienen deslizadores que solo cambian la vista previa, y con ellos se
crean, cambian, renombran y quitan expresiones.

El mismo inspector tiene herramientas sobre la vista previa para el resto del personaje. *Crear una
cadena en la vista previa* pide un clic en el hueso donde empieza y otro en el hueso donde termina;
cada cadena elige qué alcanza (el puntero, un punto, un hueso o un punto de la malla, u otra capa),
se dobla hacia el otro lado, tiene peso y límites por hueso. El objetivo de punto es un anillo que se
arrastra y cada límite es un arco alrededor del hueso con dos puntos que fijan su ángulo mínimo y
máximo. Los huesos que se balancean se eligen con un clic sobre la vista previa, con su modo y
deslizadores de rigidez, amortiguación, inercia, gravedad (una flecha cuya punta se arrastra) y
viento, y casillas que acotan su giro (un arco con dos puntos que se arrastran) y cuánto se desliza
su cabeza;
*Dibujar una cuerda* crea, a lo largo del trazo, hasta 9 huesos nuevos encadenados que cuelgan del
hueso elegido y se balancean como cuerda, y *Simular* y *Reiniciar* hacen correr la física en la
vista previa o la devuelven a la pose de reposo. El viento de `canvas.wind` es una flecha que se
arrastra desde el centro del lienzo, con su ángulo, su fuerza y sus ráfagas (`canvas.gusts`). Para
sujetar capas, se arrastra una capa sobre un hueso, o sobre la malla para crear allí un punto nuevo
(un nodo sin malla del archivo, colgado del hueso que manda en ese lugar), eligiendo si sigue la
posición, el giro y la escala, y una línea une cada capa sujeta con su hueso o punto y acaba en un
anillo que se arrastra para moverla a lo largo de él. Los puntos de la malla se listan con su nombre,
que se edita ahí mismo, el hueso del que cuelgan y un botón para quitarlos, y sobre la vista previa
son cuadrados que se arrastran; un punto creado para una sola capa se va con ella cuando la capa
deja de seguirlo. Los puntos nunca cuentan como el final de un hueso, así que no cambian las
cadenas ni la física. Las partes se
ordenan arrastrándolas en una lista de atrás hacia delante, se ocultan y se recortan para verse solo
sobre otra parte, cuyo contorno se marca en la vista previa. Los contornos de las partes, los huesos
y puntos y las asas de cadenas y física se ven en cualquier malla que el editor muestre; una malla
que dibujan varios nodos recibe en la primera edición una malla propia para cada uno (`<malla>_<nodo>`),
así sus huesos y pesos también se editan aquí, y cada primitiva se empareja con su parte por el nodo y
la primitiva de los que sale. En el inspector general, todo campo que
nombra un hueso, un punto, una parte o una forma de mezcla (cadenas, objetivos y límites, física,
reglas, máscaras de huesos de las capas de animación, `hit`, `attach` y `parts`) ofrece los de la
malla en una lista.

Desde la terminal hacen lo mismo `reiwand-ctl scene act CARPETA mesh_from_picture`, `bone` (con `--rename`,
`--root` y `--remove`), `mesh-depth`, `weights` (pincel de pesos con `--mode`, `--by`, `--radius`,
`--falloff` y `--lock`), `auto-weights`, `clip` (con `--add`, `--remove`, `--rename`, `--length` y `--loop`), `key` (claves
de huesos, formas y orden, con `--ease`, `--remove`, `--from` y `--copy`), `expression`, `ik` (`add
RAÍZ PUNTA`, `set N` y `remove N`, con `--target`, `--limit HUESO MÍN MÁX`, `--bend` y `--weight`),
`physics` (`add HUESO...` con `--mode`, `--stiffness`, `--damping`, `--inertia`, `--gravity`,
`--wind`, `--rotation`, `--translation` y `--free`; `rope NOMBRE X,Y;X,Y;...` con `--parent`; `remove
HUESO...`), `wind` (`--angle`, `--strength`, `--noise` y `--off`), `attach` (`--to`, `--bone`,
`--point X,Y` o un nodo, `--offset`, `--nudge`, `--inherit` y `--detach`), `point` (puntos de sujeción
con `--at`, `--parent`, `--root`, `--rename` y `--remove`) y `parts` (`--order`, `--hide`,
`--show`, `--clip`, `--unclip`, `--depth-scale`, `--cull`, `--sort` y `--flat`); `bones`,
`part-outlines`, `vertex-weights` y `keys` imprimen los huesos y puntos, el contorno de cada parte,
los pesos de cada vértice y las claves de un clip.

Al elegir un sistema de partículas o una capa de partículas, la vista previa dibuja encima sus
herramientas: cada forma de emisor con asas (el punto, las esquinas de una caja, el centro y el radio
de un círculo, también alrededor del punto de control al que estén atados, los extremos de una línea; para `around`, el centro, el radio y el final del arco, con
un punto en cada lugar de nacimiento; para `between`, sus extremos, un asa que curva la línea y otra
que desliza el punto más ancho del afinado; para `layer`, el contorno de la capa y los píxeles que
pasan el umbral), los puntos de control `p0` a `p7` como marcadores que se arrastran y un anillo con
un asa para la distancia focal de la perspectiva. La velocidad turbulenta dibuja el abanico de
direcciones que puede dar el ruido, las exactas de las primeras partículas que nacen ahora con la
semilla con que corre la vista previa (`clock.seed` o, si es `null`, una que el editor elige para la
sesión y le pasa a la vista previa; *Conservar esta semilla* la escribe en `clock.seed`, y si el
arranque sigue una banda de sonido un control fija cuánto suena para los rayos, que giran con ella
como en el motor; `reiwand-ctl scene act CARPETA particles` dibuja lo mismo)
y una flecha con un asa
que la gira y otra que abre o cierra el abanico; el desfase fractal dibuja flechas desde una rejilla
de lugares de nacimiento hasta donde los lleva el ruido, calculadas como en el motor, y un anillo con
un asa para la distancia. Atractores, vórtices, órbitas y frenadas cerca de un punto dibujan un
anillo a trazos por cada alcance (radio, interior, exterior y el anillo del vórtice) con un asa que
lo cambia, y un asa en su centro; la sección de distancias y bandadas los añade y muestra el punto
que siguen, su fuerza y un deslizador por alcance. El inspector del sistema ofrece el tipo de cada
emisor, a qué punto de control o al puntero siguen `around` y `between`, una lista de las capas desde
las que puede emitir (nunca una que contenga el sistema) con el umbral, su color, su movimiento y
cuándo se vuelve a leer, los puntos de control con su dial de ángulo, las reasignaciones de valores
iniciales con sus entradas y salidas, rangos de dos asas, límites, operación, onda y la curva de la
salida frente a la entrada (un componente de curvas que también pueden usar otras herramientas), el
recorte de alfa con su rango, su opacidad y la rampa que da, los campos de la velocidad turbulenta
(dirección, giro, velocidad del ruido, dispersión, fase, velocidad y banda de sonido) y del desfase
fractal (distancia, tamaño, detalle, dispersión, deriva y lados), la orientación con un dial, la
perspectiva con su distancia focal y los rangos de profundidad, lo que cada hijo toma de su
partícula padre y los shaders de partículas del paquete con el aviso de que el motor los comprueba y
puede dejarlos fuera. Desde la terminal hace lo mismo `reiwand-ctl scene act CARPETA particles` con
`shape`, `control-point`, `remap add|set|remove`, `cutout`, `orient`, `perspective`,
`emit-from-layer`, `inherit` y `shader`, y `handles` imprime lo que la vista previa dibuja.

El inspector de un sistema empieza con la lista de los sistemas de la escena, donde cada uno se
renombra (las capas y los sistemas que lo usan lo siguen; los scripts que lo nombran entre comillas no se reescriben y la app y `reiwand-ctl` avisan de cada uno), se abre o se quita, y se añade uno sin
capa. Los rangos iniciales (vida, velocidad, dirección, tamaño, giro, rotación y alfa) tienen un
deslizador de dos puntas, una curva que reparte los valores hacia un extremo con su dibujo y un
selector por extremo que lo enlaza a un ajuste del paquete. Los colores iniciales salen de dos
colores, de tonos con su saturación y su brillo (con pasos), de una paleta con su ruido o de una
barra de degradado donde un clic añade una parada y arrastrarla la mueve. Cada fuerza puede actuar
solo en parte de la vida de la partícula, con una forma de cuatro asas que dice dónde aparece, actúa
y se desvanece. Cada emisor tiene su ritmo (un deslizador en vivo, con *Ritmo fijo* si sigue claves o
un enlace) y una franja de tiempo que muestra cuándo emite: el retraso, el final de la duración, los
tramos de emisión y de pausa (los más cortos en una fila arriba y los más largos en otra abajo) y las
ráfagas, con asas que arrastran el retraso, la duración, el largo de los tramos y la separación entre
ráfagas (la escala de la franja queda quieta mientras se arrastra), y debajo el tamaño de la ráfaga y si sale con el tiempo,
el ritmo o un clic, seguir el puntero, emitir solo con sonido y la velocidad que toma del emisor. La
sección *Orden de dibujo y lo que pueden cambiar las capas* elige qué partículas quedan encima y cuántas veces se repiten los fotogramas
en una vida, y qué valores pueden cambiar las capas de partículas (lo demás queda bloqueado). Al
elegir una capa de partículas, el inspector ofrece sus propios valores sobre los del sistema
(opacidad, tamaño, parte de las partículas, velocidad, vida, ritmo del tiempo, brillo y tinte, cada
uno con su vuelta al del sistema) y los puntos de control que se arrastran en el lienzo pasan a ser
solo de esa capa. Desde la terminal, `reiwand-ctl scene act CARPETA particles` suma `emitter N` con
`--burst`, `--every`, `--on`, `--periodic`, `--duration`, `--delay`, `--follow`, `--audio-gate` e
`--inherit`, `window N A,B,C,D|none`, `rename NUEVO`, `start --hsv|--palette|--gradient`,
`render --cycles --order`, `settings --lock`, `force turbulence --time-scale --mask` y las fuerzas
`noise`, `cap`, `torque`, `spin-drag`, `vortex`, `orbit` y `slow-near`. Los rangos de `start` aceptan
`MÍN,MÁX,CURVA`, con `prop:ID` en un extremo para enlazarlo a un ajuste y `_` para dejarlo como está;
sin curva se conserva la que había y `1` la quita (por ejemplo `start --life 1,prop:vida,2`).

La vista previa dibuja también los colisionadores del sistema con el color de su respuesta (rojo
donde mueren las partículas, verde donde se pegan) y asas para cada tipo: el borde del lienzo, el
plano con su punto y el lado en el que se quedan, el círculo con su centro y su radio, la caja con
dos esquinas, el segmento con sus extremos y la capa con su contorno y su campo de distancias; un
anillo con la distancia que se mantiene alrededor de su objetivo, o la línea entre dos puntos, con
asas que mueven el objetivo, los extremos y la distancia. El inspector ofrece la respuesta de cada
colisionador (rebotar, deslizar, parar, pegar o eliminar) con el rebote y el rozamiento, si un
círculo o una caja retienen las partículas dentro y la capa con la que chocan; los objetivos y
deslizadores de las distancias; las bandadas con el radio de vecinas, la separación, la alineación,
la cohesión y la velocidad máxima; los operadores de reasignación con el mismo editor de curvas y
la edad como entrada; la cuerda y la estela con sus campos y un selector de textura, y un aviso que
deja en una partícula por paso los emisores que soltarían varias, para que la cuerda quede pareja. El presupuesto
de partículas simula el sistema en el procesador hasta el instante de la línea de tiempo y dice
cuántas viven de su capacidad, las plazas en la GPU y en el procesador de cada clase de equipo (los
teléfonos y los equipos antiguos caen en las dos más bajas) y lo que guarda cada nivel de calidad
de nacimientos, vecinas, puntos de estela y de cuerda; puede dibujar encima las partículas
simuladas, la cuerda y algunas estelas, y al pulsar una partícula marca el radio de sus vecinas. Un
selector de calidad de Q0 a Q3 limita la de la vista previa y *Reiniciar los sistemas* vuelve a
empezar solo sus partículas, sin rehacer la escena. En la terminal, `reiwand-ctl scene act CARPETA particles` suma `collider add|set|remove|
response`, `keep distance|between|remove`, `boids`, `operator remap add|set|remove`, `rope`,
`trail`, `texture`, `turbulent`, `offset`, `pace` y `budget` (`rope` avisa también de los emisores
que amontonan la cuerda), y `reiwand-ctl scene preview --quality Q --restart` hace lo mismo con la
vista previa. El resto del sistema también tiene subcomandos: `create` (un sistema sin capa, copia
de otro con `--from` o hijo de otro con `--parent` y `--on`, como *Nuevo sistema hijo* del panel de
hijos), `delete`, `settings` (capacidad, semilla, precalentado, espacio y ritmo de los emisores),
`start` (rangos iniciales de vida, velocidad, ángulo, tamaño, giro, color, alfa, velocidad en X e Y
y velocidad radial), `force gravity|drag|turbulence|oscillate|attractor|remove`, `child
add|set|remove`, `size-over-life`, `color-over-life` y `render` (estiramiento, cuadros del atlas al
azar, por tiempo o por vida, ritmo del atlas y brillo por encima del blanco).

El menú *Añadir capa* ofrece *Capa desde un archivo* (imagen, vídeo, sonido o malla `.glb`: elige el archivo, lo importa como recurso y añade la capa en un solo paso, como `reiwand-ctl scene act CARPETA add`), *Luz* con sus cuatro tipos, cada uno con valores de partida
razonables. Las luces se ven sobre la vista previa como marcadores que se arrastran y la elegida
muestra sus asas: el anillo del radio, la altura, el giro, los bordes del cono de un foco, los dos
extremos de un tubo y la flecha de una luz direccional, y a petición la caja que redibuja al
moverse; si sus valores tienen claves, el arrastre crea la clave en el instante actual. El inspector
de la luz elige el tipo y el color, ajusta intensidad, radio, altura, largo y cono, dibuja la caída
de la luz con la distancia y la deja sola o la silencia en la vista previa. Al elegir el lienzo se
listan todas las luces con esos dos interruptores, cuántas hay frente a las que guarda cada clase de
equipo, los colores de la luz ambiente y del cielo con una esfera iluminada por ellos y el selector
de calidad de Q0 a Q3, que aquí dice si la luz se calcula una vez o cuánto cuesta en cada fotograma.
Las capas de imagen, vídeo y texto tienen su parte de iluminación: los mapas de normales, de
reflejo, de rugosidad y de luz propia, sus deslizadores, *Generar normales* a partir de la imagen
(con fuerza, desenfoque y una vista del relieve antes de guardarlo como imagen del paquete) y un
pincel que pinta en gris la rugosidad o la luz propia sobre la vista previa con su vista de calor.
Las luces puntuales y los focos tienen *Iluminar el aire* con deslizadores de densidad y muestras,
la curva del resplandor y lo que hace cada nivel de calidad; los focos eligen qué proyectan (imágenes,
vídeos y las capas que se pueden proyectar) y la vista previa dibuja la imagen dentro del cono, donde
cae sobre el lienzo. Las herramientas de partículas tienen su parte de luz: recibir las luces, el
mapa de normales con una vista de la partícula iluminada por delante y por detrás, y la refracción con
su mapa, su cantidad y el desplazamiento dibujado sobre la vista previa; el presupuesto dice qué
cuesta cada nivel y qué recortan los teléfonos.
En la terminal hacen lo mismo `reiwand-ctl scene act CARPETA light`,
`scene act CARPETA particles`,
`scene ask CARPETA lights` (las asas, la caja y las cuentas, o el coste con `--cost`), `scene act CARPETA lighting`,
`scene act CARPETA normals`, `scene act CARPETA paint_map` y
`scene act CARPETA ambient`.

Los modelos 3D se añaden con *Modelo 3D desde un archivo*, que lee el `.glb` antes de importarlo y
dice sus triángulos, materiales, texturas, huesos y clips y qué recibe cada clase de equipo, o con
*Modelo 3D* y *Grupo 3D* del menú de capas. La vista previa de una escena con modelos tiene una vista
de edición con su propia cámara, que se guarda con el proyecto pero solo la usa la vista previa (igual
que la vista de depuración y la división del mapa de tonos): el botón derecho la gira, el central
(o Mayús) la desplaza y la rueda la acerca, mientras la cámara de la escena se dibuja como una
pirámide con asas para el ojo y el punto al que mira. El modelo elegido muestra flechas en x, y y z
para moverlo, una asa para girarlo y otra para escalarlo; un clic en su caja lo elige. El inspector de
un modelo enseña sus cuentas, el reflejo (`env`) con miniatura y un panel de materiales con una esfera
de muestra, el shader, la luz, la doble cara, las sombras, los colores, los números y las miniaturas
de sus mapas; el de un grupo lo vuelve 3D y el de una capa plana dentro de él activa *Ocultarse tras
los modelos más cercanos*. Al elegir el lienzo aparecen la cámara 3D (campo de visión y giro en vivo,
*Poner la cámara desde la vista*, *Mirar por la cámara*) y los colores brillantes (HDR, mapeo de tonos
y un deslizador de antes y después sobre la vista previa); el selector de calidad dice cuántos
triángulos, grupos 3D y memoria de texturas cuesta cada nivel y si cabe en cada clase de equipo.
En la terminal hacen lo mismo `reiwand-ctl scene act CARPETA scene3d` y `scene ask CARPETA cost3d`.

Para animar un modelo, su inspector tiene un panel de animación: los clips de la capa con un botón
para abrirlos en la línea de tiempo de clips (con filas de giros y desplazamientos 3D por hueso, pesos
de las formas de mezcla y las marcas de los eventos), *Clip nuevo del archivo* con las animaciones del
`.glb`, *Movimiento de raíz*, huesos que se balancean con ajustes predefinidos (pelo, cola, tela,
antena, cadena, gelatina), un deslizador por forma de mezcla y las capas de animación (clip, peso,
velocidad, fundidos, modo aditivo y máscara de huesos), que ahora también aparecen bajo los huesos de
las mallas. En la vista de edición el modelo elegido dibuja sus huesos, que se eligen con un clic, y el
camino que recorre con el movimiento de raíz; el hueso elegido muestra un anillo que lo gira (por el
borde) o lo inclina (por dentro) y un cuadrado que lo mueve, y cada arrastre pone una clave en el clip
en el cursor de reproducción. Los huesos que se balancean dibujan el abanico de su límite de giro con
dos asas para cambiarlo, y su panel tiene el viento de cada hueso, los límites y la flecha del viento
sobre la vista previa. Las asas de mover, girar y escalar parten del valor del modelo en ese momento:
si tiene claves ponen una clave y si sigue un enlace no lo tocan. Cada material elige pelo (capas por calidad, largo,
densidad y un panel para peinarlo), vegetación (balanceo, velocidad, la flecha del viento y la máscara
de los colores de vértice sobre la vista previa) o croma (color clave, tolerancia, suavizado y el mate
sobre la vista previa); *Shader de material nuevo* escribe una plantilla y cada shader del paquete dice
si compila o en qué línea y columna falla, con un aviso de que corre en la tarjeta gráfica. El panel de
luces enciende las sombras por luz, con el tamaño del mapa y lo que dibuja cada clase de equipo, el
desplazamiento y una vista que pinta de rojo lo que queda en sombra y dibuja los volúmenes de los
mapas. El panel de la cámara 3D edita recorridos: claves tomadas de la vista actual, que se arrastran
sobre la curva dibujada en la vista de edición (el ojo y el punto al que miran, cada uno a su
profundidad) y en su propia fila de la línea de tiempo de la escena, bucle, suavizado, orden, fundido y un botón para
recorrerlos desde la cámara. El panel de coste suma por nivel los mapas de sombra, las capas de pelo y
los vértices con piel. En la terminal: `reiwand-ctl scene act CARPETA animate`,
`scene act CARPETA clip`, `scene act CARPETA key`, `scene act CARPETA
scene3d` y `scene act CARPETA physics`.

### Pasar modelos FBX y OBJ a glTF con Blender

Reiwand lee solo glTF binario (`.glb`). Para un modelo en FBX u OBJ, con Blender 3.6 o más
nuevo:

1. Abre Blender, borra el cubo de la escena inicial y usa *Archivo → Importar → FBX* (u *OBJ*). Si
   el modelo trae texturas aparte, deja las imágenes junto al archivo para que Blender las encuentre.
2. Comprueba la escala y la orientación: glTF usa metros, +Y arriba y −Z adelante; el exportador
   convierte los ejes de Blender, así que basta con que el modelo se vea derecho en Blender. Aplica
   la escala y la rotación con *Objeto → Aplicar → Todas las transformaciones*.
3. Revisa los materiales: el exportador traduce el *Principled BSDF* (color, metal, rugosidad,
   normales, emisión y alfa); otros nodos se pierden. Para que una textura viaje dentro del archivo,
   conéctala directamente al *Principled BSDF*.
4. Usa *Archivo → Exportar → glTF 2.0*, elige el formato *glTF Binary (.glb)* y, en *Incluir*,
   *Objetos seleccionados* si solo quieres una parte. Deja activadas *Normales*, *UV* y *Tangentes*,
   y en *Animación* las acciones y las formas de mezcla si las tiene. Cada acción de Blender llega
   como una animación del archivo con su nombre; las armaduras llegan como pieles de hasta 4 huesos
   por vértice (en *Datos de malla → Pesos*, *Limitar total* a 4 antes de exportar si hace falta).
   Para la vegetación, pinta en *Pintura de vértices* el color de los vértices: el rojo dice cuánto
   se balancea cada vértice con el viento.
5. Si el archivo pesa mucho, reduce texturas a 2048 píxeles o menos (lo que reciben los teléfonos) y
   usa el modificador *Diezmar* para bajar triángulos; `reiwand-ctl scene ask CARPETA model_info` dice si
   cabe en cada clase de equipo.
6. Impórtalo en el editor con *Modelo 3D desde un archivo* o con
   `reiwand-ctl scene import CARPETA modelo.glb --id modelo` y `reiwand-ctl scene act CARPETA scene3d '{"model":{"add":{"asset":"modelo"}}}'`.

Los archivos que cargan imágenes o búferes desde fuera, o que exigen extensiones no admitidas (como
la compresión Draco), se rechazan: expórtalos sin compresión de mallas.

Cada cambio es una operación sobre un JSON Pointer. Deshacer y rehacer (Ctrl+Z y Ctrl+Mayús+Z)
guardan hasta 500 pasos o 64 MiB, y un arrastre cuenta como un solo paso. Lo que no se ha guardado
se copia cada 30 segundos y al cerrar en `~/.local/state/reiwand/editor/`, que solo puede leer
tu usuario, y la app ofrece recuperarlo al volver a abrir la carpeta; la copia se borra al cerrar
con todo guardado y las que llevan 30 días sin usarse se borran solas. Importar, compilar y aplicar
trabajan en segundo plano, sin bloquear la ventana. *Publicar…* abre una página de tres pasos
(revisar, firmar y elegir dónde) que crea el `.wpak` con el mismo código que `reiwand-ctl pack build`,
lo firma con tu clave de autor (como `reiwand-ctl pack sign`) y lo guarda, lo sube a tu servidor o lo
añade a un repositorio; *Poner de fondo* lo aplica a una pantalla o a todas.

Cada acción del editor se hace también con `reiwand-ctl scene act CARPETA ACCIÓN 'JSON'`, que trabaja
sobre la carpeta sin daemon, guarda al momento y comparte con la app el historial de deshacer, donde
cada acción es un paso; `reiwand-ctl scene actions` las lista y `scene actions NOMBRE` da su forma y un
ejemplo (todas en [`docs/crear/acciones.md`](acciones.md)). `preview` y `apply` usan el servicio:

```sh
reiwand-ctl scene new mar --from parallax --title Sea --locale en   # empty, particles, parallax, clock o web
reiwand-ctl scene open copia --from escena.wpak            # una copia sin firma, derivada, con el id <id>.copy
reiwand-ctl scene import mar ola.ogg                       # copiar un recurso a assets/
reiwand-ctl scene import mar espuma.png --atlas 4,4         # hoja de sprites; --linear para mapas de datos
reiwand-ctl scene act mar add_layer '{"kind":"sound","source":"ola"}'   # y remove_layer, duplicate_layer, move_layer
reiwand-ctl scene act mar add_layer '{"kind":"text","name":"Título"}'   # id «titulo», con ese texto
reiwand-ctl scene act mar add_layer '{"kind":"mesh","file":"zorro.glb"}'   # importa el .glb y lo muestra en una capa mesh
reiwand-ctl scene act mar set '{"pointer":"/scene/layers/1/opacity","value":0.5}'   # cualquier campo por su JSON Pointer
reiwand-ctl scene act mar keyframe '{"pointer":"/scene/layers/1/opacity","time":2,"value":1,"ease":"in_out_sine"}'
reiwand-ctl scene act mar keyframe '{"pointer":"/scene/layers/1/opacity","time":3,"from":2}'   # moverlo a otro instante
reiwand-ctl scene act mar timeline '{"id":"olas","add":true,"duration":6,"loop":"repeat","add_events":[{"t":3,"name":"ola"}]}'   # y to, remove, frames con fps, drop_events
reiwand-ctl scene act mar keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"value":-0.5,"timeline":"olas","base":0.5}'   # relativo a 0,5
reiwand-ctl scene act mar keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"out_handle":[1,0]}'   # asa Bézier hasta la clave vecina (aquí la de 3 s); también in_handle
reiwand-ctl scene act mar keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"straighten":true}'   # recta hasta la clave siguiente, sin tocar las curvas vecinas
reiwand-ctl scene act titere clip '{"layer":"zorro","id":"saludo","add":true,"play":true}'   # un clip nuevo que ya suena en una capa de animación
reiwand-ctl scene act mar translate '{"pointer":"/manifest/title","language":"es","text":"Mar"}'   # otro idioma; "add":true no pisa nada, sin "text" lo quita
reiwand-ctl scene act mar set '{"pointer":"/manifest/content","value":{"rating":"mature","descriptors":["violence"]}}'
reiwand-ctl scene thumbnail mar captura.png                # miniatura en assets/, reducida si hace falta
reiwand-ctl scene act mar add_property '{"kind":"combo","id":"tiempo"}'   # y move_property, rename_property, remove_property
reiwand-ctl scene act mar set '{"pointer":"/manifest/properties/tiempo/options","value":["calma","tormenta"]}'
reiwand-ctl scene act mar set '{"pointer":"/manifest/properties/tiempo/default","value":"calma"}'
reiwand-ctl scene act mar add_property '{"kind":"slider","id":"velocidad","label":"Speed"}'
reiwand-ctl scene act mar translate '{"pointer":"/manifest/properties/velocidad/label","language":"es","text":"Velocidad"}'
reiwand-ctl scene act mar condition '{"id":"velocidad","text":"tiempo == \"tormenta\"","enabled":true}'   # comprobada como en validate
reiwand-ctl scene act mar add_preset '{"label":"Calm"}' && reiwand-ctl scene act mar set '{"pointer":"/manifest/presets/0/values/tiempo","value":"calma"}'
reiwand-ctl scene act mar effects '{"add":{"layer":"far","effect":"vignette","params":{"intensity":0.5}}}'   # y set, remove, move, mask
reiwand-ctl scene act mar layer '{"transform":{"layer":"far","position":[960,400],"scale":[1.2,1.2],"tilt":[15,0],"z":40}}'   # y blend, instance
reiwand-ctl scene act mar property '{"id":"velocidad","min":0,"max":10,"step":0.5,"default":2}'
reiwand-ctl scene act mar manifest '{"tags":{"tags":["mar","noche"]}}' && reiwand-ctl scene act mar manifest '{"license":{"license":"CC-BY-4.0"}}'   # y origin, authors, display
reiwand-ctl scene act mar canvas '{"bloom":{"strength":1.5}}' && reiwand-ctl scene act mar canvas '{"parallax":{"amount":2}}'
reiwand-ctl scene act mar group '{"ids":["near","far"],"id":"colinas"}' && reiwand-ctl scene act mar hide '{"id":"sky","hidden":true}'
reiwand-ctl scene act mar undo && reiwand-ctl scene history mar
reiwand-ctl pack check mar                             # problemas, cada uno con su JSON Pointer
reiwand-ctl scene preview mar -o DP-1 --time 2             # verla en el escritorio 60 s sin aplicarla
reiwand-ctl scene preview mar --layout dual --out dual.png # extendida sobre dos pantallas, sin escritorio
reiwand-ctl scene preview mar --layout dual --out dual.png --time 4   # el segundo 4 en vez del póster
reiwand-ctl scene preview mar --quality q1 --battery --calm   # como la dibuja el demonio en el nivel Q1, con batería y menos movimiento
reiwand-ctl scene render mar --out mar.png --quality q1 --battery      # un fotograma a archivo, sin escritorio
reiwand-ctl scene render mar --out mar.gif --frames 90 --pointer 0.2,0.5:0.8,0.5 --click   # un clip con un puntero simulado
reiwand-ctl scene render mar --out brillo.png --against q0 --without-bloom   # comparar niveles o el brillo, partido por la mitad
reiwand-ctl scene render mar --out hora.png --time-of-day 15:42               # corre los scripts con esa hora de pared
reiwand-ctl pack sign mar --key autor.key && reiwand-ctl pack build mar   # firmar y crear el .wpak
reiwand-ctl scene apply mar -o DP-1                        # añadirla a la biblioteca y mostrarla
reiwand-ctl scene apply mar --span                         # mostrarla extendida sobre todas las pantallas
```
