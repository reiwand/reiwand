# Reproductor multimedia

Una escena puede mostrar lo que suena en un reproductor del equipo: el título, el artista, el
álbum, la carátula, sus colores, si suena o está en pausa y por dónde va. El paquete declara la
entrada `media` (`inputs` del manifiesto; el validador exige que la declare si la usa) y nada le
llega hasta que la permites para ese paquete: el aviso de permisos de la app y de la bandeja, el
interruptor *Mostrar lo que suena* de sus ajustes o `reiwand-ctl input grant <id> media`. Sin permiso
recibe textos vacíos y lo que el autor puso para cuando no suena nada. El servicio escucha a los
reproductores por el bus de sesión (MPRIS) solo mientras una pantalla visible muestra un fondo con
ese permiso que lo usa; sigue sus señales sin preguntar cada poco, elige el que suena (o el último
que sonó) y deja de escuchar en cuanto nada visible lo necesita. Nada de lo que lee se guarda, se
registra ni se envía a ningún sitio. *Ajustes → Reproductor multimedia* y `reiwand-ctl media status`
dicen si se está leyendo, para qué pantallas y si hay un reproductor abierto, sin mostrar la pista.
Si la escucha falla (por ejemplo, se pierde el bus), se marca como no leída y vuelve a empezar a los
10 segundos mientras un fondo visible la necesite.

Algunos reproductores dan la carátula como una dirección de internet en vez de un archivo. Solo se
descarga con *Descargar carátulas de internet* encendido (`reiwand-ctl config media.remote_covers
true`, apagado por defecto y propio de cada equipo), como mucho 4 MiB y en 5 segundos, y esa web
puede saber que esa pista suena aquí; las carátulas solo viven en memoria. Al encenderlo se
descarga enseguida la carátula de lo que suena y al apagarlo se quitan las que vinieron de internet.
Cada dirección se descarga una sola vez: las últimas ocho carátulas de internet se recuerdan en
memoria mientras el interruptor siga encendido, así que volver a una pista no la pide de nuevo.

En el editor, el menú de enlace de cada campo tiene *Seguir al reproductor multimedia* con los
datos que le caben: el título, el artista, el álbum, el artista del álbum, el subtítulo, los
géneros y el tipo para un texto, cada uno con lo que mostraría ahora. *Texto mientras no suena
nada…* abre el texto que muestra mientras no hay pista o falta el permiso (`placeholder`, un texto
o uno por idioma, como mucho 512 caracteres), con sus traducciones; desde la terminal, `reiwand-ctl
scene act CARPETA set` y `reiwand-ctl scene act
CARPETA translate` (el JSON de cada una, con `reiwand-ctl scene actions NOMBRE`). La pestaña *Reproductor*
simula una pista (título, artista, álbum, si suena y carátula) para diseñar sin música sonando; solo la ve la vista
previa, sin pedir permiso ni leer los reproductores. Desde la terminal, `reiwand-ctl scene preview
CARPETA --media pista.json` hace lo mismo con un JSON de esos campos.

El botón *Nivel de la vista previa* de la barra del lienzo dibuja la vista previa como la dibuja el demonio en Q0, Q1, Q2 o Q3,
con batería y con menos movimiento, y para cada efecto dice qué se deja fuera y cuánto cuesta en ese nivel. Ahí mismo se
compara con otro nivel o sin el brillo del lienzo (mantener pulsado el botón enseña el otro lado), se guarda una imagen o un
clip a archivo, con un puntero simulado si hace falta, y se nombran los efectos y emisores que siguen al puntero, para probarlos
con *Probar el puntero y los clics*, que ahora dibuja un aro donde está el puntero y un destello en cada clic. Desde la terminal,
`reiwand-ctl scene preview CARPETA --battery --calm` y `reiwand-ctl scene render CARPETA --out archivo` hacen lo mismo.

La pestaña *Probar* (también en los paquetes de página) muestra las propiedades de usuario del
manifiesto con el mismo control que los ajustes de un paquete mostrado y lista sus presets; elegir un
preset o cambiar un valor solo cambia la vista previa, sin guardar nada ni tocar ninguna pantalla, y
*Volver a los valores por defecto* termina la prueba. Desde la terminal,
`reiwand-ctl scene preview CARPETA --values valores.json [--preset ID]` hace lo mismo.

La carátula es un recurso de imagen con `media: current` (la de lo que suena) o `media: previous`
(la anterior), que vale donde vale una imagen; su archivo es lo que se ve mientras no suena nada o
sin permiso. El selector de imagen del editor ofrece *Carátula de lo que suena* y *Carátula
anterior*, hechas a partir de la imagen elegida, y deja elegir esa imagen de reemplazo; desde la
terminal, `reiwand-ctl scene asset CARPETA cover --cover current --placeholder IMAGEN`, y con
`--field /scene/layers/0/image` la pone en ese campo en un solo paso, usando una carátula que ya
exista o creando otra sin convertir nunca una imagen normal. Las carátulas
se leen una vez por pista, se reducen a 256 píxeles como mucho y solo viven en memoria.

De cada carátula salen cinco colores, calculados una vez: el principal, el secundario y el
terciario (los más frecuentes), uno de texto que se lee bien sobre el principal y negro o blanco de
alto contraste: `{"media": "primary"}`, `secondary`, `tertiary`, `text` y `contrast`, con `smooth`
para pasar de uno a otro sin saltos. La pestaña *Reproductor* los muestra como muestras de color.
`{"media": "playing"}` vale 1 (o verdadero) mientras suena y 0 en pausa, parado o sin pista, y
sirve para mostrar u ocultar capas o para cualquier número. `position` y `duration` (segundos) y
`progress` (de 0 a 1) sirven para barras de avance y, en un texto, se escriben como `3:25` o `42%`.
La posición avanza sola desde la última lectura del reproductor, sin volver a preguntarle: una
barra pide como mucho 30 fotogramas por segundo y un texto uno por segundo, solo mientras suena.

Una línea de tiempo o una capa de sonido también puede arrancar sola, sin scripts, con
`media:play` (empieza a sonar), `media:pause` (se pausa o se para), `media:track` (llega una pista
nueva) o `media:cover` (llega una carátula nueva). El selector de disparador del editor los ofrece
con su nombre, junto a los eventos con nombre de las líneas de tiempo y los clips (`event:`) y el
clic, la entrada y la salida del puntero de cada capa sólida o con huesos tocables (`click:`,
`enter:`, `leave:`), el botón *Pista siguiente* de la pestaña *Reproductor* los prueba en la vista previa y
desde la terminal `reiwand-ctl scene act CARPETA timeline` los pone (`--list` muestra los que
hay).

Los scripts que declaran `media` leen el objeto `media` del módulo `reiwand` (estado, textos de la
pista, si hay carátula y sus colores, posición, duración y progreso) y reciben `mediaStatus`,
`mediaPlayback`, `mediaProperties`, `mediaThumbnail` y `mediaTimeline` cuando algo cambia; sin
permiso solo ven `enabled: false`, y nunca reciben la imagen de la carátula. *Nuevo script…* en el
panel de recursos (o `reiwand-ctl scene act CARPETA import_script`) crea un
módulo con todos esos eventos y valores para enlazar; hay otras plantillas (`touch`, `timer`, `audio`,
`spawn`, `wave`, `daytime`, `setting` y `launches`). La sección *Scripts* del panel de recursos
lista los módulos de la escena y deja quitarlos, ordenarlos, atar un `.js` de la carpeta, abrirlos en
el editor del sistema (la vista previa se recarga sola al guardar el archivo) o editarlos en la
propia app, con una referencia de la API. Un informe compara el código con su declaración (lo que
usa y no declara, lo que declara y no usa) y *Declarar lo que usa* lo corrige. Las plantillas de
`scene.spawn` salen como un árbol de capas con su inspector, y *Hacer plantilla* toma las capas
elegidas. Para probarlos sin música, `reiwand-ctl media
simulate PANTALLA pista.json` muestra una pista inventada en cualquier pantalla (`--stop` la quita).

Un fondo web que declara `media` recibe lo mismo con `window.reiwand.onMedia(cb)`: estado, título,
artista, álbum, la carátula como una dirección `blob:` hecha en la propia página, sus colores,
posición y duración. Si además puede salir a internet (declara orígenes de red), pide una segunda
confirmación aparte (`reiwand-ctl input grant ID media_network`, el segundo aviso de la app y la
bandeja, que también lo dejan quitar en los controles de la escena, o en el teléfono un diálogo
propio al encender el segundo interruptor), porque el título y la carátula podrían salir por la red.

Lo mismo vale para el sonido: un fondo web que declara `audio` y orígenes de red no oye nada hasta
que se permiten `audio` y también `audio_network` (`reiwand-ctl input grant ID audio_network`, el
segundo aviso de la app y la bandeja, o en el teléfono un diálogo propio al encender el segundo
interruptor), porque lo que oye, el sonido que suena o el micrófono, podría salir por la red.
