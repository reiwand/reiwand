# Paquetes: clasificación, idiomas y créditos

El manifiesto de un paquete (`docs/specs/package.md`) lo describe. En `reiwand.wpak/3`
declara su clasificación (para todos los públicos o para adultos, con sus motivos), si destella o
muestra patrones rápidos que pueden provocar crisis a personas con epilepsia fotosensible, si es
una obra original o rehecha a mano a partir de otras (con el nombre de sus autores), los créditos
y la licencia de cada archivo, las entradas que puede pedir (sonido, puntero, clics, multimedia y
red), sus enlaces y lo que necesita del motor. El título, la descripción, las notas de cambios y
los textos de sus propiedades pueden ir en varios idiomas, con tablas de cadenas y una cadena de
respaldo (tu idioma, su idioma base, el idioma del paquete); los scripts de una escena reciben el
idioma elegido. La descripción y las notas admiten párrafos, listas, negrita, cursiva y enlaces
`https`, que la app abre solo con un clic y tras confirmarlo; el resto de textos se muestran tal
cual y no pueden llevar `<` ni `>`. Los paquetes de la primera versión del manifiesto ya no se
abren. Las seis escenas de ejemplo se
describen en inglés y en español, y `pulse` avisa de que destella con el ritmo.

La app y la bandeja difuminan las miniaturas de los paquetes para adultos hasta que decides
mostrarlas (en los detalles de la app o con un clic o el teclado en la miniatura difuminada de una
pantalla, en la página *Ajustes* de la app o en la bandeja; en *Explorar* basta un clic en la
miniatura, y la decisión se guarda para ese paquete), y *Ocultar el contenido para adultos*
(`library.hide_mature`) los saca de las búsquedas y de los repositorios. *Mostrar el contenido para
adultos sin difuminar* (`library.show_mature`, desactivado por defecto) deja de difuminarlas en la
app, la bandeja y el teléfono. Antes de aplicar un
paquete fotosensible, la app y `reiwand-ctl` avisan y piden confirmación una sola vez por paquete, y
la app también lo permite desde sus detalles con *Permitir este fondo…* o para varios elementos
seleccionados a la vez con *Permitir estos fondos…*; hasta entonces el daemon no lo aplica, un
perfil no lo pone (la pantalla muestra el motivo y el perfil lo pone en cuanto lo permites) y las
listas lo saltan. La
página *Ajustes* de la app y la bandeja avisan en la pantalla que lo muestra. La búsqueda filtra
por clasificación y por facetas que se calculan del propio paquete: las entradas que usa la escena
(`facet:audio`, `facet:pointer`, `facet:clicks`), el tamaño (`facet:4k`), `facet:photosensitive`,
`facet:android` si funciona entero en el teléfono, etc. Las páginas web toman sus facetas de las
entradas que declaran; en los demás paquetes todas, `media` incluida, salen de lo que usan:

```sh
reiwand-ctl lib show 1R1AB2                     # clasificación, origen, créditos, idiomas y descripción
reiwand-ctl lib search 'content:general facet:audio'
reiwand-ctl config playback.allow_flashing true       # permitir los paquetes fotosensibles
reiwand-ctl lib confirm mature 1R1AB2           # dejar de difuminar su miniatura
reiwand-ctl pkg reveal org.example.noche        # lo mismo para un paquete de un repositorio
reiwand-ctl pack check fondo/                   # qué necesita, usa y declara, y si lo reproducen el escritorio y el teléfono
```

Además de imágenes, vídeos y escenas, el formato describe páginas web (`web`: una página de entrada,
un script que se carga antes que los suyos, un almacenamiento local de 5 MiB como mucho que nunca se
sincroniza, y hasta 8 orígenes `https` a los que puede llegar si das permiso de red, con la API
`window.reiwand` para recibir ajustes, sonido, pausa y carpetas), preajustes publicados aparte (`preset`:
dependen de un paquete base por id y rango de versiones y solo llevan valores y sus imágenes o
vídeos), paquetes de recursos que otros autores copian en sus escenas (`assets`, con créditos que
nombran el paquete de origen), dependencias entre paquetes (`requires.packages`), los dispositivos
para los que está hecho (`platforms`), textos en un archivo por idioma (`locale/<idioma>.json`),
paquetes importados para uso personal (nunca se publican) o descargados de un proveedor (con su
crédito y una licencia libre), el color de acento que propone al escritorio, el color alrededor de
la imagen, la extensión sobre todas las pantallas, el volteo, el límite de fotogramas, el inicio en
silencio y la reproducción única. Cada función tiene un nombre en `requires.features`, y el
escritorio y el teléfono dicen si la ejecutan, si reproducen el paquete sin ella o si lo rechazan
con «necesita una versión más nueva de Reiwand» antes de dibujar nada; en esta versión
rechazan los recursos publicados aparte y las imágenes AVIF. `reiwand-ctl pack check` dice qué hace cada plataforma con un paquete
y `docs/specs/package.md` describe el formato entero.

Un paquete de recursos (`assets`) ofrece capas, sistemas de partículas, scripts y efectos sueltos
para que otros autores los copien en sus escenas. En el editor de escenas, el panel *Assets* tiene
*Crear un paquete de recursos* (marcas las partes, les das título y miniatura, y eliges licencia,
crédito e imagen del paquete) e *Importar de un paquete de recursos*, que muestra la miniatura de
cada parte y la licencia y el crédito de cada archivo antes de copiar nada; lo importado queda en
`assets/<id>-<versión>/`, con sus créditos y el origen escritos solos, y la escena funciona sin el
paquete. La biblioteca los lista aparte (filtro *Recursos* o `kind:assets`) y no se aplican como
fondo.

```sh
reiwand-ctl pack new-assets mi-escena/ partes/ --part layer:nieve --part particles:nieve \
  --id org.example.partes --title Partes --license CC-BY-4.0 --attribution "Tu nombre"
reiwand-ctl pack build partes/ && reiwand-ctl pack parts org.example.partes-1.0.0.wpak
reiwand-ctl scene act otra-escena/ import_assets '{"package":"org.example.partes-1.0.0.wpak","parts":["layer:nieve"]}'
```

Un preajuste publicado aparte aparece en la biblioteca como variante de su fondo base: la ficha del
base lista sus preajustes, la del preajuste nombra su base, y *Aplicar* (o `reiwand-ctl lib apply`)
muestra el base con esos valores; los que el base instalado no conoce, de otro tipo o fuera de rango
se descartan con un aviso en el registro. Instalar un paquete de un repositorio instala también, del
mismo repositorio, los paquetes que nombra en `requires.packages` y faltan; si aun así falta el base,
la ficha ofrece instalarlo. El autor puede proponer un color de acento (`display.accent`, un color o
una propiedad de color): el escritorio lo da al fondo de Plasma, que lo usa si eliges el acento desde
el fondo, y el teléfono al sistema; sin él se propone el dominante de la vista previa, y
`reiwand-ctl state` lo muestra por pantalla. `playback` también vale para escenas (límite de
fotogramas, volumen y silencio del autor): los fotogramas por segundo nunca pasan del mínimo entre
lo que pides, lo que pide el autor, la pantalla, la batería y el nivel de calidad. Las escenas con
sonido, los vídeos y las páginas web tienen su propio volumen por pantalla, que empieza en el del
autor (o en cero si pide empezar en silencio) y se multiplica por el general; *Silenciar los fondos
mientras otra aplicación reproduce sonido* (`playback.duck`, apagado de serie) los baja con un
fundido mientras alguna otra aplicación reproduce, contando sus flujos de PipeWire sin leer sus
nombres ni su contenido. En el teléfono el mismo ajuste (*Silenciar el fondo mientras otra app
reproduce sonido*) sigue las reproducciones activas que Android anuncia, sin saber de qué app son
ni qué suenan, y solo escucha mientras un vídeo, una escena o una página con sonido se ve; baja el sonido solo
si hay más reproducciones que las suyas (una por vídeo que suena, una por página con sonido y una por escena que suena). Las propiedades con valores por nivel de calidad (`quality`) los toman solas
cuando el equipo ahorra energía, y scripts y páginas reciben el nivel. Un atajo (`shortcut`) lo
eliges tú en los ajustes del fondo: una dirección `https://` (se abre con el portal del escritorio),
un archivo o una carpeta (con el selector) o una aplicación por su id `.desktop` (fuera del
Flatpak); el fondo nunca ve qué elegiste y solo puede pedir abrirlo dentro de un clic real, como
mucho una vez por segundo, en una pantalla que acepta clics. En el teléfono eliges en la página
del paquete una app instalada, una dirección `https://` o un archivo o carpeta del selector de documentos (se guarda con permiso de lectura que sigue tras reiniciar), y se abre con un toque en la pantalla de
inicio si el paquete tiene permiso para los toques (nunca desde la de bloqueo). Si no se puede abrir
(la app ya no está instalada, ninguna app abre la dirección o Android lo rechaza) *Ajustes* lo dice en
una línea hasta que se cierra o un atajo se abre bien, y esa línea se conserva aunque se reinicie el proceso; Android no avisa cuando bloquea en silencio que
un servicio de fondo de pantalla abra una app, y eso no se puede detectar. Si una escena usa clics en una pantalla
a la que no llegan (como Plasma dentro del Flatpak), sus ajustes lo avisan. Un paquete que pide
sombreadores de cálculo avisa antes de aplicarlo si la gráfica no los tiene, y la ficha de cada
paquete dice en qué equipos funciona entero (`desktop`, `android`): el validador lo calcula con las
funciones que usa y los límites del teléfono (1 GiB descomprimido, imágenes de 8192 px de lado y
32 Mpx).

Las escenas también describen ya todo lo que un autor puede querer rehacer a mano: 15 modos de fusión
más (`linear_burn` … `negation`), tinte y brillo de capa, padre de transformación sin cambiar el orden
de dibujo, inclinación en profundidad con perspectiva, máscaras con intensidad, capas que alimentan
efectos, vídeos como máscara o textura, control de reproducción, hojas de sprites y GIF o WebP
animados, filtrado, repetición y mipmaps de texturas, texto con ajuste de línea, alineación vertical,
caja de fondo, contorno, sombra y desenfoque, colecciones de fuentes (`ttc`, `otc`, `woff2`) y
fuentes de respaldo, sonido en MP3, listas de sonidos en orden o al azar con pausas, sonidos largos
en flujo y disparados por eventos, cámara con zoom, giro y sacudida, ajustes del paralaje (también
con la inclinación del teléfono), resplandor de escena, HDR con mapeo de tonos, fotogramas que se dibujan
sobre el anterior, luces puntuales, focos, tubos y direccionales con volumen y proyección,
iluminación 2D con mapas de normales y luz ambiente, captura de una región, eventos con nombre en
las líneas de tiempo, curvas con manejadores, animación por componente y relativa, la hora y la fecha
como valores, rangos de frecuencias con límites y exponente, y efectos propios del paquete en WGSL
con pasadas, objetivos, variantes, hasta 8 texturas y 24 parámetros, comprobados al leer el paquete
contra el contrato de su preludio. Cada uno tiene su nombre en `requires.features`, y el escritorio
y el teléfono rechazan con el mismo aviso una escena que use alguno que su motor aún no dibuja (el
teléfono ya reproduce también el sonido nuevo). `docs/specs/scene.md`
describe cada campo y sus límites.

De esa lista, el escritorio y el teléfono ya dibujan la cámara 2D (centro, zoom, giro y una
sacudida de ruido que se apaga con «reducir movimiento» y en la calidad más baja), el resplandor de
escena con tinte y su juego HDR (solo en la calidad más alta, o en la siguiente con corriente; difumina
a un cuarto del tamaño, se suma al lienzo con la mezcla sin copiarlo y no cuesta nada con fuerza cero), los
fotogramas que se dibujan sobre el anterior (más caros, porque cada cambio redibuja la imagen
entera), la inclinación en profundidad de capas y grupos con su máscara proyectada igual (una capa
inclinada dentro de un grupo inclinado gira como un solo cuerpo visto por una sola cámara), la captura
de una región por una capa de ajuste para cambiarla en su sitio o dibujarla de nuevo como lupa o
espejo, el espaciado entre letras, el contorno, la sombra y el desenfoque del texto desde un campo de
distancias que la GPU construye una vez por cada cambio del texto en texturas de 8 bits, que todas las
GPU pueden dibujar, también las que no tienen texturas de coma flotante de 16 bits (y que deja
escalarlo sin volver a dibujar las letras), y los vídeos en cualquier sitio que admite una imagen, con un decodificador por
vídeo que se pausa mientras nada lo muestra, la reproducción de `playback` y el control desde
scripts con `layer.video` (reproducir, pausar, parar, saltar, ritmo, volumen de 0 a 1, tiempo,
duración y aviso de fin; los vídeos dentro de una escena van sin sonido en el teléfono, así que allí
el volumen no cambia nada).

El escritorio y el teléfono ya dibujan también los efectos propios del paquete. El inspector de cada
uso de efecto muestra sus reglas para despertar la escena (qué entrada pide cada parámetro, a cuál
espera y a cuál cede el paso) y en los efectos del paquete las cambia con selectores, igual que
`reiwand-ctl scene act CARPETA effect_rule`. Cada uno es un
descriptor JSON y un shader WGSL con hasta 24 pasadas que escriben en la siguiente textura o en
objetivos con nombre a su escala (en 8 o 16 bits por canal), los copian, los intercambian para
alternar entre dos (ping-pong), acumulan con fusión por pasada (reemplazar, normal, sumar o
multiplicar), deforman la capa con un shader de vértice sobre una rejilla de hasta 64 × 64 celdas y
leen hasta 8 texturas con su tamaño. Un efecto con `compose` lee la capa ya dibujada sobre lo que
hay debajo y su resultado sustituye esa zona. Las variantes (`options`, constantes `override` de
WGSL) se compilan todas al cargar la escena, nunca a mitad de un fotograma, y los parámetros y las
variantes salen como controles en el editor. El shader recibe además la hora del día, el tiempo del
fotograma, el puntero y su posición anterior, el tamaño de la pantalla, el espectro de 16, 32 o 64
bandas, mezclado o por canal, el paralaje y el color y la opacidad de la capa, con funciones de
ayuda propias (rotación, gris, HSV y los modos de fusión). Una máscara sin imagen toma su color de
`paint`, y una textura puede usar las genéricas `white`, `black`, `noise` o `normal_flat`. Como el
shader viene del paquete, naga lo comprueba antes de que llegue al driver: sin enlaces propios, con
todos los recursos acotados, con bucles de contador entero que avanzan un paso fijo hasta un
límite fijo sin desbordarse, expresiones de como mucho 256 operaciones de profundidad, como mucho
1 048 576 operaciones por píxel, 32 768 operaciones de largo contando cada llamada entera y 16 KiB
de variables por píxel. El shader se compila en hilos aparte con un plazo de 10 segundos. Si el
driver lo rechaza o no termina a tiempo, la capa se dibuja sin el efecto y la app, la bandeja,
`reiwand-ctl state` y los diagnósticos del teléfono lo anotan, sin tratarlo como un fallo. Antes de
comprobar o compilar un efecto por primera vez cada programa lo anota como «a prueba» y borra la
nota cuando termina o cuando pasan sus primeros fotogramas; una nota que deja un proceso caído o
colgado cuenta como un fallo, igual que una GPU que se detiene, y con dos el efecto queda fuera
(lo recuerda por el hash del efecto, así que una versión nueva vuelve a probarse). *Probar de
nuevo* en la app, en los diagnósticos del teléfono o `reiwand-ctl retry-effects` lo prueban otra vez.
El motor mide en la GPU cada fotograma, cada efecto, también los integrados, y la simulación de las
partículas (`reiwand-ctl scene bench` muestra el tiempo como `gpu` y la parte de las partículas en la
línea de la simulación); un efecto de paquete solo queda fuera por lento cuando los efectos de
paquete de un fotograma pasan del límite por sí solos, nunca por lo que cuestan las capas o los
efectos integrados, de eso se encarga el nivel de calidad. Los destinos de los efectos de paquete se
limpian en su primera pasada y las capas de ajuste leen lo que tienen debajo sin copiarlo: una pila
de cuatro efectos de paquete a 4K cuesta alrededor de un 10 % menos. El paralaje de profundidad
(`depth_parallax`) no dibuja nada mientras no movería la imagen (sin órbita ni `overscan` y con el
puntero fuera, en el centro o sin seguirlo), que en el teléfono es casi siempre. Un efecto de una
pasada con una máscara de imagen en modo `strength` solo se dibuja sobre el recuadro donde su máscara
no es cero, y nada mientras ese recuadro queda fuera de la pantalla; un efecto de paquete puede
declarar con `bounded` qué texturas limitan dónde cambia la imagen.
Cada pasada, cada lectura de textura y cada operación del shader cuentan en el coste de la escena
aunque el descriptor diga menos, y
`reiwand-ctl pack check`, `pack build` y el editor avisan cuando una escena no cabe con calidad 2 en
una GPU integrada antigua o de teléfono. `docs/specs/scene.md` explica cómo pasar un shader de HLSL
o GLSL a WGSL, con una tabla de equivalencias.

Las dos plataformas componen ya las capas con los 32 modos de fusión: a los 17 de siempre se suman
`linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`, `linear_light`,
`pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`, `average` y `negation`,
con fórmulas publicadas por canal que el motor, los efectos integrados y los del paquete comparten.
Una capa, aunque esté oculta, puede alimentar la textura de un efecto con `{ "layer": "<id>" }`: se
dibuja aparte y entra alineada con la pantalla, de modo que cada punto del efecto lee el punto de la
otra capa que tiene encima, y solo se vuelve a dibujar cuando esa capa cambia. Las máscaras de los
efectos aceptan también una capa o una forma, con inversión, difuminado e intensidad, como las de
las capas, y una pasada puede leer `backdrop`, lo que ya hay dibujado bajo la capa, tanto en los
efectos integrados como en los del paquete. Hay tres efectos integrados nuevos: `tint` tiñe la capa
con un color y un modo de fusión o la colorea conservando su luminosidad, `blend_texture` mezcla
otra imagen o una capa con un modo de fusión, desplazada y repetida, y `light_shafts` dibuja haces
de luz propios, en paralelo, desde un punto o desde una esquina, de un color o con un degradado, y
solo pide fotogramas mientras su velocidad no es cero. Cada capa que alimenta un efecto y cada máscara
dibujada cuentan en el coste de la escena. `docs/specs/scene.md` incluye una tabla de equivalencias
con los efectos de otros programas y cómo convertir sus parámetros.

El escritorio y el teléfono ya dibujan de esa lista el tinte y el brillo de capa, el padre de
transformación, las máscaras con intensidad, los ajustes del paralaje (el teléfono también con su
inclinación, con los mismos sensores que el paralaje de las fotos y solo mientras se ve la escena),
el filtrado nítido, la repetición, el espejo y los mipmaps de las texturas (estos se crean una sola
vez, cuando una imagen se dibuja a menos de tres cuartos de su tamaño), las hojas de sprites y los GIF
o WebP animados (el siguiente fotograma solo se pide cuando toca y nunca con la capa oculta), el texto
con ajuste de línea, número de líneas con puntos suspensivos, interlineado, alineación vertical, caja
de fondo y margen, las colecciones de fuentes, `woff2` y las fuentes de respaldo, las curvas con
manejadores, la animación por componente y relativa, y la hora y la fecha como valores, con los
nombres de meses y días en inglés o en español. Los colores de los fotogramas clave se mezclan en
OKLab por el camino de tono más corto. Los scripts leen además la hora del día (`engine.daytime`),
usan en `Date` la zona horaria del equipo aunque cambie, sin leer ningún archivo, y escriben fechas
con `time.format`.

También dibujan los eventos con nombre de las líneas de tiempo (cada uno salta una vez por pasada,
aunque un fotograma se lo salte, y arranca otras líneas de tiempo o sonidos sin despertar la escena
antes de tiempo) y el cierre de los bucles, que mezcla la última clave con la primera. Los scripts
cambian la velocidad de una línea de tiempo, la paran, leen su tiempo, su duración y si sigue en marcha,
y reciben sus eventos y su final. El escritorio toca las capas de sonido completas: MP3, varios archivos
en orden o al azar con pausas, sonidos largos de hasta diez minutos leídos en flujo sin cargarlos
enteros y sonidos disparados por eventos, con el volumen y el paso de un lado a otro suavizados. El
sonido sigue el tiempo de la escena: los bucles se vuelven a alinear, al reanudar suena donde dice la
línea de tiempo y la latencia baja de unos 160 ms a unos 50 ms. El teléfono toca las mismas capas,
con unos 200 ms de latencia. Una escena sin imagen de vista previa recibe una miniatura dibujada a partir de la
propia escena en su momento de póster.

El formato describe también las escenas más avanzadas: capas `mesh` que deforman una imagen con un
esqueleto (glTF binario con huesos y pesos), con clips y capas de animación, cinemática inversa
hacia un punto, el cursor, un hueso u otra capa, física de muelles y cuerdas con viento, reglas que
cambian el padre de un hueso, formas de mezcla y expresiones, partes con orden y recorte, y huesos
que reciben el puntero; capas `model` con modelos glTF, materiales metálico-rugosos (también pelo,
follaje, croma o shaders de material del paquete), grupos 3D con profundidad compartida, cámara 3D
con recorridos y sombras de las luces; capas sujetas a huesos o puntos de un modelo; sistemas de
partículas en espacio de mundo con puntos de control, ajustes por capa, sistemas hijos, perspectiva,
emisión desde una capa, alrededor o entre puntos, inicializadores con exponente, HSV, paleta,
velocidad vectorial, turbulencia, ruido fractal y reasignación, operadores que oscilan, limitan,
frenan, mantienen distancias o forman bandadas con ventanas de vida, colisiones con cajas, segmentos
y capas, y renderizadores de cuerda, estelas largas, recorte, orientación, luz, refracción y shaders
propios; la declaración de lo que usa cada script (eventos, partes de la interfaz, parámetros,
plantillas de capas y reservas de geometría); valores, carátulas y disparadores del reproductor
multimedia; escenas extendidas sobre todas las pantallas; y
salida a la iluminación RGB con su permiso (ver [«Iluminación RGB»](luces-rgb.md)). Los modelos se leen al abrir el paquete y cada hueso,
animación, parte o material que nombra la escena se comprueba contra ellos. Las entradas de un
paquete de la versión 2 salen solo de lo que declara: también las de sus scripts, que ya no se
adivinan por su texto; un script de la versión 1 que no declara nada usa el sonido o el puntero solo
si importa `audio` o `input` de `reiwand`, nunca porque esas palabras salgan en un comentario. Todo tiene su nombre en `requires.features` y, mientras el motor no lo dibuja,
el escritorio y el teléfono rechazan la escena con el aviso de versión más nueva (el teléfono, que no
reproduce sonidos de escena ni tiene iluminación RGB ni varias pantallas, solo omite esas partes).
Las capas `solid` ya reciben el puntero: `click:`, `enter:` y `leave:` arrancan líneas de tiempo y
sonidos sin scripts, y los scripts reciben los eventos de cada capa; los scripts también reproducen,
pausan y paran las capas de sonido.

Las capas `mesh` ya se dibujan en el escritorio y en el teléfono: la GPU deforma los vértices con
hasta cuatro huesos cada uno y las formas de mezcla más pesadas, y la imagen resultante se compone
como cualquier otra capa, con sus efectos, máscaras y modos de fusión. Los clips toman las
animaciones del archivo o claves propias en el JSON de la escena (giro, posición y escala de cada
hueso, peso de cada forma y orden de cada parte), en bucle, ida y vuelta o una vez; las capas de
animación se mezclan con peso, velocidad, fundidos de entrada y salida, capas aditivas y capas
limitadas a unos huesos. Las reglas pasan un hueso a otro padre sin que salte, las expresiones
agrupan formas, y las partes se ordenan y se recortan unas sobre otras. Un esqueleto quieto o que ya
terminó no pide fotogramas, la pose se calcula solo para capas visibles y sin reservar memoria en
cada fotograma, y en la calidad más baja la malla se dibuja a la mitad de resolución y se mueve a 30
fotogramas por segundo como mucho; la textura de la malla se lee con mipmaps y filtrado lineal
cuando se dibuja más pequeña, así que los bordes del personaje siguen suaves, sin escalones. Los scripts controlan los clips con
`scene.layer("zorro").anim("saludo")` (`play`, `pause`, `stop`, `seek`, `rate`, `weight` y el aviso
`ended`) si declaran `animations`.

También se mueven solas: las cadenas de cinemática inversa giran sus huesos hasta alcanzar un punto,
un hueso, otra capa o el puntero (este solo con el permiso del puntero, que la app avisa como
cualquier otro uso del puntero), exactas para dos huesos (cuyos límites pueden doblarlas al otro lado) y,
en cadenas más largas, por rondas desde la pose de sus clips (que conservan su curva) que nunca doblan
una articulación al otro lado, o desde un arco parejo si la pose está recta, se dobla al otro lado o no
alcanza el objetivo; cada fotograma parte de su propia pose, igual en vivo, tras saltar o al volver a
empezar, con límites por hueso, peso y el
sentido en que se doblan todas las articulaciones (que el editor invierte con un toque sobre la cadena). La física de huesos simula muelles, péndulos y cuerdas con
un paso por fotograma dibujado tan largo como el intervalo entre fotogramas (1/120 s a 120 fps, 1/60 s
a 60, 1/30 s a 30, también con batería, así que se mueve igual a esas frecuencias; un intervalo de más
de 1/30 s se reparte en pasos iguales de 1/30 s como mucho, y un fotograma recupera como mucho un
cuarto de segundo), con inercia,
amortiguación, gravedad, límites de giro y de estiramiento y el viento de `canvas.wind` con ráfagas
(`canvas.gusts`, de 0 a 1) tomadas en el instante de cada paso, iguales a cualquier tasa de fotogramas;
un cuerpo quieto medio segundo deja de pedir fotogramas y en la calidad más baja los huesos se
quedan en la pose de la animación. Con `depth` la profundidad de los vértices se proyecta con la
perspectiva de `canvas.perspective` (también bajo la inclinación de la capa y la de los grupos y padres
que la contienen, siempre a lo largo de la cara de la capa tal como la giran) y las caras vistas por
detrás se ocultan si se pide; primero se dibujan los píxeles opacos de todas las piezas, que marcan
la profundidad, y después los suaves, que solo la consultan, así los bordes suaves de una pieza
cercana dejan ver lo de atrás sin halo en cualquier orden, aunque las piezas se crucen, y entre
piezas a la misma profundidad la que va después tapa los bordes suaves de la anterior; además las
piezas se ordenan solas de atrás hacia adelante según la profundidad media de sus vértices en la
pose, formas mezcladas incluidas (se puede apagar por capa con `depth.sort: false`), para que lo
suave sobre lo suave se mezcle en orden. El panel de la malla en el
editor enciende la profundidad y ajusta su perspectiva, el ocultado de caras y este orden, y
`reiwand-ctl scene act CARPETA parts` hace lo mismo con `--depth-scale`, `--cull`, `--sort` y `--flat`. Cualquier capa se sujeta a un hueso o a un nodo de una malla con
`attach`, siguiendo su posición, giro y escala del mismo fotograma; lo que no hereda del hueso
lo sigue tomando de la cámara, que la acerca y la gira igual que al resto de la escena. Los scripts leen y mueven huesos
con `scene.layer("zorro").bone("brazo")` (giro, posición y escala locales o del mundo, para el
fotograma siguiente y antes de la física, de modo que el hueso vuelve con su muelle al soltarlo),
fijan formas de mezcla con `shape`, empujan huesos con `impulse`, buscan el hueso más cercano con
`hit` y leen puntos de sujeción con `attachment`; los huesos de `hit` reciben el puntero y los
eventos dicen qué hueso se tocó.

De las partículas nuevas, el escritorio y el teléfono ya dibujan, igual en la GPU que en la CPU, el
espacio de mundo (las partículas nacen con la posición, el giro y la escala de su capa y después no
la siguen), hasta ocho puntos de control movidos por valores, el cursor, la capa o los scripts, las
cajas huecas y las elipses o medios círculos con un punto de anclaje, la duración, el retardo, la
emisión por ventanas aleatorias, el tope de una partícula por paso y la espera a que suene algo, los
inicializadores con exponente, alfa propio, HSV, paleta, velocidad vectorial, velocidad hacia fuera
y velocidad heredada del cursor o de un punto, los operadores con ventanas de vida (oscilar posición,
alfa o tamaño, limitar la velocidad, par y freno de giro, frenar cerca de un punto, alcance y caída
de atractores, vórtices con anillo y turbulencia con desplazamiento y máscara), los sistemas hijos
estáticos, al nacer, al morir o que siguen a cada partícula (dos generaciones como mucho), los ajustes
por capa (alfa, tamaño, cantidad, velocidad, vida, ritmo, brillo, color y puntos, con bloqueo en el
sistema), el brillo, los ciclos y el estiramiento acotado del dibujo, y el control desde scripts con
`particles(capa)`: `play`, `pause`, `stop`, `playing`, `emit(n)`, `point(n, [x, y])`, `instance` y
`from(n, capa)` y `wall(n, capa)`, que hacen que el emisor o el colisionador `n` de tipo capa tome
otra capa, también una copia de una plantilla (`null` vuelve a la del sistema).
Una escena simula como mucho 64 sistemas contando hijos y capas con partículas propias, dentro de
las 200 000 partículas, y el precalentamiento llega a 30 s en pasos de 1/15 s. Lo demás de la lista
anterior sigue pidiendo una versión más nueva.

También dibujan las estelas con historial (cada partícula guarda sus últimas posiciones, hasta 16
pasos de 1/60 s, y deja una cinta que se estrecha y se apaga con el color y el tamaño que tenía en
cada punto; 4, 8 o 16 pasos según la clase de equipo, menos o ninguno en los niveles de calidad
bajos y ninguno con render por software), el orden de dibujo (`render.order`: las más nuevas, las
más viejas, las más grandes, las más pequeñas o las más bajas encima, en 256 escalones), los choques
contra la parte opaca de una imagen (`collide` con `mask`, convertida al cargar en un campo de
distancias de hasta 16 384 celdas), el color inicial en degradado (`init.gradient`, hasta 8
paradas), las hojas de sprites con rectángulos de distinto tamaño, y los extremos de los rangos
iniciales y el radio de los círculos enlazados a valores, que se leen en cada fotograma. Los enlaces
de sonido toman rangos de bandas en 16, 32 o 64 (`audio: [0, 3]`, `of`, `reduce: mean | max`) de un
canal, límites (`bounds`), exponente y suavizado distinto al subir y al bajar (`smooth: { up, down }`),
también para el cursor, las propiedades y los scripts, y una escena puede fijar con `audio` su ataque,
su caída y si sigue el volumen del sistema (`gain: fixed`) o se ajusta sola (`auto`). Cuando las
partículas pasan al procesador, el estado de la pantalla lo dice (`"particles": "cpu"`) y la
aplicación, la bandeja, `reiwand-ctl` al mostrar la escena y los ajustes del teléfono avisan de que se
muestran menos.

Además dibujan, igual en la GPU que en la CPU, la perspectiva de las partículas en escenas 2D
(`perspective` con la profundidad y su velocidad al nacer, de lejos a cerca con fusión normal), el
recorte del alfa de la textura (`render.cutout`), la orientación hacia la pantalla, el movimiento o un
ángulo fijo (`render.orient`), la emisión desde los píxeles visibles de otra capa con su color y su
movimiento (forma `layer`, leída a 256 o 512 píxeles y de nuevo solo cuando la capa cambia), la
velocidad inicial turbulenta, el desfase fractal del nacimiento, el nacimiento por turnos alrededor
de un punto o entre dos con arco y extremos afinados, la reasignación de valores al nacer
(`init.remap`, hasta ocho, con la hora, el tiempo, la vida, la velocidad, el azar, la distancia a un
punto, el sonido o el puntero), la herencia del color, el tamaño, el alfa y la velocidad de la
partícula padre (los hijos de un sistema con perspectiva nacen a su profundidad o donde se lo ve
dibujado, y al heredar la velocidad también toman la que lleva en profundidad), y los shaders de partículas de un efecto del paquete (`render.shader`, con las mismas
comprobaciones, el mismo tiempo de construcción y el mismo límite de tiempo en la GPU que los demás
efectos del paquete, y dejado fuera, sin dejar de dibujar las partículas, si no se puede construir o
es demasiado lento).

También mueven y dibujan, igual en la GPU que en la CPU, las partículas que mantienen una distancia
con un punto o se acercan al segmento entre dos (`keep_distance`, `keep_between`), la reasignación
aplicada en cada paso (fuerza `remap`, también con la edad), las bandadas (`boids`: separación,
alineación, cohesión y velocidad máxima, con una rejilla hash rehecha en cada paso; hasta 8 192
partículas, 2 000 en la CPU, y 32, 16 u 8 vecinas según la calidad y el equipo; las que nacen en el
mismo punto se separan), los choques contra
cajas por dentro o por fuera, segmentos, el interior de un círculo y la parte opaca de otra capa
(campo de distancias de 128 × 128 rehecho solo cuando la capa cambia), la respuesta que deja pegada
la partícula (`stick`) y la que la mata, que cuenta como muerte para los sistemas hijos. Las cuerdas
(`render.rope`) unen las partículas en el orden en que nacieron con una tira Catmull-Rom construida
en un sombreador de cálculo, de hasta 4 096 tramos (1 024 y 2 048 en los equipos más modestos y
menos subdivisiones en calidad baja), y las estelas largas guardan hasta 64 posiciones repartidas a lo
largo de `length` pasos (`segments`), con subdivisión, ancho, textura y su escala y desplazamiento,
16 o 32 posiciones en los equipos intermedios y ninguna en el más modesto.

El escritorio y el teléfono dibujan también las capas de luz (`type: light`): puntuales, focos con
su cono interior y exterior, tubos y direccionales, con color, intensidad, radio y altura sobre el
lienzo, que se emparentan, se sujetan a huesos y se animan como las demás capas. No se dibujan: las
capas de imagen, vídeo y texto con `lighting` las reciben con su mapa de normales, rugosidad (con su
mapa), reflejo de una imagen o de lo que tienen debajo, luz propia (con su mapa) y una intensidad
que mezcla lo iluminado con lo de siempre, más la luz ambiente de `canvas.ambient` y la del cielo de
`canvas.sky` según hacia dónde mire la normal; los scripts leen y cambian las dos como el fondo. La
luz se calcula en la misma pasada que dibuja la capa, con 12 luces como mucho (8 en los teléfonos y
los equipos modestos) y solo hasta donde llega cada una: con las luces quietas las capas iluminadas
se dibujan una vez y se guardan, una luz que se mueve solo redibuja la caja de su radio y, en la
calidad más baja, la luz se calcula una sola vez al empezar o al cambiar de nivel o de tamaño, y las
luces que se mueven se quedan donde empezaron, sin pedir fotogramas. Una escena sin luces se ve exactamente como antes.

Las luces puntuales y los focos pueden iluminar el aire (`volumetric`, con densidad, muestras y
haces): un resplandor que sale de la luz hasta su radio, en el lugar de la luz entre las capas (las
de encima lo tapan), donde las partes oscuras de las capas de debajo cortan la luz en haces si se
pide; se calcula a la mitad del tamaño de la pantalla, a un cuarto en Q1 y en los teléfonos, y
apagado en Q0, y se guarda mientras la luz y lo que tiene debajo están quietos. Un foco proyecta una
imagen, un vídeo o una capa con sus efectos (`texture`, hasta cuatro focos por escena) sobre las
capas iluminadas y el aire (las partículas toman solo su color); una capa que recibe luz no se
puede proyectar, para no formar un bucle. Las partículas reciben las luces (`render.lit`) en sus dos
caras, hasta 8, con un mapa de normales que gira con cada partícula (`render.normal`), igual en la
simulación por GPU que en la del procesador, y pueden refractar lo que tienen detrás con un mapa de
normales (`render.refract`, como gotas en un cristal): a tamaño completo en Q3, a la mitad en Q2 y
nada en Q1 y Q0.

El escritorio y el teléfono dibujan también los modelos 3D (`type: model`): glTF binario con mallas,
nodos, materiales metálico-rugosos (color, normales, metal y rugosidad, oclusión, luz propia, modos
de transparencia y doble cara) y texturas PNG, JPEG o WebP de hasta 4096 píxeles (2048 en los
teléfonos y los equipos modestos). Un modelo suelto se dibuja en su propia caja; un grupo con
`space: "3d"` reúne sus modelos en una pasada con profundidad invertida de 32 bits y se compone como
una capa más, con efectos, máscaras y mezcla. Las capas planas del grupo quedan delante salvo que
pidan `depth_test`: entonces se colocan en el plano que mira la cámara y los modelos más cercanos las
tapan. Los materiales de la escena cambian por nombre el shader (`pbr` o `unlit`), el tinte, la luz
propia, la doble cara y los números; las capas de luz iluminan los modelos (12 como mucho, 8 en los
teléfonos), con la luz ambiente y la del cielo, y una imagen panorámica (`env`) da la luz de alrededor
y los reflejos, sin reflejos en la calidad más baja. La cámara 3D (`camera3d`: campo de visión, planos
cercano y lejano, ojo, centro y arriba) sirve para los modelos, los grupos 3D y las capas inclinadas,
y los scripts la cambian para el fotograma siguiente. Un modelo quieto se dibuja una vez y se guarda,
un grupo 3D se redibuja solo cuando algo suyo cambia, y cada clase de equipo admite 50 000, 150 000,
500 000 o 2 000 000 de triángulos (con las capas de pelo de la calidad más baja): si los modelos
dibujan más, con la calidad automática ese equipo muestra el póster de la escena (con un nivel
fijo o un mínimo elegido la reproduce igual y avisa de lo que cuesta). Con `canvas.hdr` los colores más brillantes que el blanco se
conservan en imágenes de 16 bits y `canvas.tonemap` (`aces` o `agx`) los ajusta a la pantalla en una
última pasada; sin `tonemap`, la escena se ve como siempre.

Los modelos también se animan con el mismo sistema que las mallas: clips de las animaciones del
archivo o con claves propias (giros en x, y y z y desplazamientos en 3D por hueso, pesos de formas de
mezcla), capas de animación con peso, velocidad, modo aditivo, fundidos y máscara de huesos, física
de huesos (que se congela en la calidad más baja) y movimiento de raíz (`root_motion`), con el que un
clip que avanza mueve la capa en vez del esqueleto. El esqueleto se calcula en la CPU solo para los
modelos visibles y la GPU deforma los vértices (4 huesos por vértice) y mezcla las formas del archivo.
Los materiales también pueden ser `fur` (capas de pelo empujadas por la normal, de 1 a 32 y menos en
calidades bajas, con largo, densidad y dirección de peinado), `foliage` (balanceo con el viento del
lienzo, medido por el rojo de los colores de vértice), `chroma` (un color clave que se vuelve
transparente) o un shader de material del paquete con `fn material(s: Surface) -> Surface`, que se
valida y se limita como cualquier efecto de paquete. Las luces puntuales, de foco y direccionales
pueden dar sombra (`shadows`) con mapas de 2048, 1024 y 6×512 píxeles, filtrados con cuatro muestras;
el escritorio dibuja dos luces con sombra, la clase T2 una y las clases más modestas ninguna, y un
mapa solo se vuelve a dibujar cuando cambia su luz o lo que proyecta. La cámara 3D sigue recorridos
(`camera3d.paths`) por claves con una curva Catmull-Rom centrípeta, en orden o al azar, con fundido a
negro o cruzado entre ellos y bucles cerrados sin saltos.

El escritorio y el teléfono dibujan también la mezcla por degradado, el revelado con el puntero,
el desplazamiento por mapa o por un ruido propio, las cáusticas, los destellos, el fuego, el giro,
la transformación, el pulso (también al ritmo de una banda de sonido), el contraste local, la estela
de movimiento, el VHS, los fluidos (con tantos emisores de punto y de línea como haga falta, hasta 64,
que el editor coloca con asas sobre el lienzo y gira y refuerza desde la punta de su flecha, que
se animan con fotogramas clave en la línea de tiempo, siguen una propiedad (el lienzo muestra los
del valor actual o los que el inspector elige para cada valor, y cada emisor acepta sus extremos
escritos) o cambian desde un script; quietos mientras su fuerza es 0), el destello de lente y el chisporroteo eléctrico, y los parámetros
nuevos de los efectos que ya existían: rayos de luz direccionales o en estrella, aberración cromática
en cuatro modos, grano en blanco y negro o con otro modo de fusión, desenfoque mezclado con la imagen,
fondo visible en el iris y la lente, reflejo inclinado, desplazamiento sin repetición, sacudida con
ruido, fricción y límites, brillo con fusión, degradado y cuatro estilos (línea, línea doble, anillo
o destello, elegidos en el editor entre dibujos de cada uno), balanceo doble, remolino elíptico,
ondulaciones con perspectiva, reflejo, tinte, paredes y velocidad, ondas dobles con perspectiva y un
desfase por píxel (`phase`) que evita que las zonas vecinas se muevan a la vez. Con los valores por
defecto todo se ve exactamente como antes. Los efectos que dependen del tiempo dejan de pedir
fotogramas cuando su velocidad es cero; la estela y los fluidos se detienen en cuanto se asientan;
los fluidos se congelan en la calidad más baja, y los efectos medios se omiten en ella, como al
funcionar con batería. Los modos de fusión `tint` y `diffuse_light` son aproximaciones cuyas fórmulas
pueden cambiar. Se nombran `effect.<id>` y `effect.<id>.<parámetro>`, de modo que los motores
anteriores rechazan con el aviso de versión más nueva los paquetes que los usan. Las líneas de
tiempo pueden medirse en fotogramas (`frames` y `fps`, que ya funciona en las dos plataformas) y
enlazar el final de un bucle con su principio (`wrap`, rechazado mientras el motor no lo dibuje), y
`"seed": null` da a una escena una semilla nueva cada vez que empieza. Varias capas que muestran el mismo
vídeo comparten su reloj, guiado por la primera; con `playback.own_clock` una capa `video` lo
reproduce con un reloj propio (su velocidad, tramo, pausa y saltos de los scripts, a costa de un
decodificador más; un vídeo elegido por una propiedad con `map` lleva un reloj propio para cada
vídeo de la lista y los saltos de los scripts los mueven a todos, y un vídeo que una propiedad de
archivo elige al vuelo lleva el suyo para el archivo que el usuario elija; en una capa de imagen, en
un vídeo atado sin `map` a una propiedad que no es de archivo o en una capa que muestra geometría de
scripts se rechaza), que el inspector del editor ofrece como *Reloj propio*; la línea de tiempo del editor
muestra cada reloj propio con una tira de fotogramas del vídeo, el tramo como una barra cuyos
extremos se arrastran, su velocidad, bucle y reproducción y dónde está en el instante elegido
(`reiwand-ctl scene ask CARPETA clocks` lo imprime, con N fotogramas), y `reiwand-ctl scene act DIR set
'{"pointer":"/scene/layers/N/playback/own_clock","value":true}'` también. Un paquete con un vídeo HDR
(PQ o HLG) recibe la faceta `hdr` y `reiwand-ctl pack check` lo dice: en el ordenador se decodifica con
10 bits y la gráfica adapta sus tonos a colores normales (BT.2020 a sRGB, el blanco de la pantalla
a 203 nits y la luz más fuerte a 1000), también dentro de las escenas, y los vídeos normales no
cambian. Los clientes de repositorios se saltan las versiones de un tipo de paquete que no conocen
en vez de dejar de leer el índice. La tabla de perfiles (qué hacen el escritorio y el teléfono con
cada función) está en `docs/specs/package.md`, y `docs/specs/scene.md` termina con una guía para
rehacer a mano una escena hecha con otro programa: dónde va cada parte en el formato y cómo pasar
las funciones de HLSL y GLSL a WGSL.

`reiwand-ctl pack build` calcula las funciones del motor que usa el paquete (`requires.features`) y las
escribe en el manifiesto; si un paquete pide una función o una versión que este Reiwand no
conoce, dice que hace falta una versión más nueva (en el escritorio, con su miniatura). Las
versiones publicadas antes de la versión 2 del manifiesto no abren sus paquetes, pero siguen leyendo
los repositorios que los tienen: solo ven en ellos los de la primera versión. Las páginas web, los
preajustes y los paquetes de recursos van solo en los índices nuevos del repositorio, que esas
versiones no leen, así que no los ven. El validador comprueba
que las entradas declaradas incluyan las que la escena puede usar con cualquier valor de sus
propiedades y dice dónde las usa. Después de actualizar Reiwand, la biblioteca vuelve a leer
una vez sus paquetes, así que los que pedían una versión más nueva empiezan a funcionar y los
anteriores reciben sus facetas. En el teléfono, la página de un paquete avisa de lo que no funciona
allí.
