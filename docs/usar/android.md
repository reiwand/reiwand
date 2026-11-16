# Android

La misma página *Releases* trae un APK para Android 12 o posterior (64 bits) por arquitectura:
`reiwand-android-arm64-v8a.apk` para los teléfonos (unos 22 MB) y
`reiwand-android-x86_64.apk` para los emuladores y algunas tabletas (unos 23 MB), más
`reiwand-android.apk` con las dos (unos 39 MB) si no sabes cuál te toca. Antes un solo APK de
83 MB llevaba las dos arquitecturas, ONNX Runtime dentro y el código de Kotlin sin reducir.
Siempre se firman con la misma clave y la misma versión, así que cada versión se instala encima de
la anterior, también si cambias de uno a otro, y conserva el fondo elegido. Ábrelo en el teléfono
(Android pedirá permitir la instalación desde esa app) o instálalo desde un ordenador con
`adb install -r reiwand-android-arm64-v8a.apk`.

Para ponerlo de fondo, abre la app **reiwand**. La barra de abajo tiene tres pestañas, *Biblioteca*,
*Listas* y *Ajustes*, y detrás de cada página se ve, quieta y velada, la imagen del fondo en uso. La pestaña *Biblioteca* muestra las fotos y
los vídeos añadidos con *Añadir*, que ofrece tres caminos: *Fotos y vídeos* abre el selector de fotos
de Android, *Elegir archivos* abre el selector de documentos para escoger varias imágenes y vídeos de
cualquier carpeta, de la tarjeta o de una nube instalada, e *Importar una carpeta* añade las imágenes
y los vídeos que hay dentro de una carpeta y de sus subcarpetas (5000 como mucho) mostrando cuántos
lleva de cuántos. Todo se copia al almacenamiento privado de la app, así que no hace falta ningún
permiso y la app no usa la red salvo para descargar el modelo de profundidad cuando lo pides, para
tu servidor propio y los repositorios que añades y, solo mientras escribes la dirección de un
servidor, para buscar en la red local los que se anuncian. Un mismo archivo no se copia dos veces, y los que no se pueden abrir
se cuentan en un solo aviso. Al tocar uno se abre su página, con una vista previa en vivo con la
forma de la pantalla, sus medidas y duración, y los botones *Usar como fondo*, favorito, *Añadir a
una lista* y *Quitar*.

Una pulsación larga en la biblioteca, o en una lista abierta, empieza a seleccionar: cada toque marca
o desmarca un elemento, la barra de arriba muestra cuántos hay con *Seleccionar todo* y *Cancelar*, y
sus acciones sirven para todos a la vez: añadirlos a una lista, marcarlos o desmarcarlos como
favoritos, elegir su ajuste y quitarlos de la biblioteca con una sola confirmación (en una lista,
sacarlos de ella). El gesto de volver sale de la selección, que se conserva al girar el teléfono.

En la página de una imagen, *Ajuste* elige cómo se coloca esa imagen (rellenar, imagen completa,
estirar, centrar o mosaico, o como en *Ajustes*), y arrastrar o pellizcar la vista previa elige la
parte que se ve en el teléfono; *Restablecer el encuadre* lo quita y vuelve a la imagen entera o al
recorte hecho en el ordenador. El ajuste es un campo de la misma edición no destructiva que usa el
escritorio (`reiwand-ctl edit fit`), y el encuadre, un recorte con la forma de la pantalla del teléfono
que la edición guarda como el de una pantalla más, `Android` (`reiwand-ctl edit crop -o Android`): se
sincroniza con el ordenador sin recortar sus pantallas, y se hace dentro del recorte de la imagen
entera si se eligió en el ordenador. Se combina con el paralaje; el paso de páginas del lanzador
recorre la imagen, o el recorte del ordenador si es más ancho que la pantalla, pero no mueve un
encuadre hecho en el teléfono, que ya tiene su forma. La vista previa usa la forma de la pantalla en
su orientación natural, también con el teléfono tumbado, donde se pone al lado de los controles.
*Usar como fondo* pregunta si va en la pantalla de inicio, en la de bloqueo o en las dos (desde
Android 14; antes, las dos muestran lo mismo), y cada pantalla guarda su propio fondo o su propia
lista, igual que una pantalla del escritorio (`[outputs.lock]` en la configuración). Si la app ya
es el fondo de esa pantalla, el cambio entra con un fundido sin pasar por la vista previa del
sistema; si no, o si un mismo fondo de la app cubre las dos pantallas y se elige solo una, abre la
vista previa del sistema para ponerlo allí; lo que se guarda es lo que el sistema puso de verdad (si allí
eliges solo la pantalla de inicio, la de bloqueo conserva su fondo). También se llega desde los fondos animados del sistema. Se dibuja con la GPU (Vulkan, u
OpenGL ES si Vulkan falla) con el mismo renderizador que el escritorio, y se recupera sola si el
sistema le quita la GPU o la superficie. La edición hecha en el escritorio se aplica igual: el giro,
el enderezado, el color y las máscaras, también las de profundidad, que seleccionan sobre el mapa de
profundidad de la foto después del mismo giro y la misma orientación que el escritorio. Admite JPEG, PNG, WebP, GIF y BMP y, con el
decodificador del propio Android (`AImageDecoder`, sin librerías añadidas al APK), las fotos HEIC y
AVIF de las cámaras. El escritorio no lee HEIC, así que no las añade a su biblioteca al recorrer
sus carpetas ni al recibir los archivos privados del teléfono.

La pestaña *Listas* crea listas de fondos que cambian solos cada 15 o 30 minutos, cada hora, cada 6
horas, cada día o nunca, en orden o al azar y, si se quiere, saltando los vídeos con batería. Como en
el escritorio, una lista puede cambiar solo ciertos días de la semana y a ciertas horas (`days` y
`at` de sus reglas), seguir donde se quedó o empezar de nuevo (`resume`), mostrar el mismo fondo en
la pantalla de inicio y en la de bloqueo (`synced`) y tener su propio fundido (`transition_ms`),
cambia de nombre con el lápiz de su página y sus fondos se reordenan
arrastrándolos por el asa de la derecha. Su *Búsqueda inteligente* la convierte en una lista
inteligente con la misma sintaxis que `reiwand-ctl search` (por ejemplo `kind:image tag:mar`): muestra
cuántos fondos encuentra mientras se escribe, se guarda con *Guardar la búsqueda* y *Hacerla manual*
la vuelve a dejar como estaba. La biblioteca también tiene un campo de búsqueda con esa sintaxis,
que se suma al filtro elegido. El cambio se decide cuando el fondo vuelve a verse, sin alarmas ni
despertar el teléfono, también con días y horas. La biblioteca, las listas y sus reglas usan el
mismo modelo y la misma lógica de rotación que el escritorio.

La pestaña *Ajustes* elige cómo se ajustan las imágenes que no tienen ajuste propio y los vídeos
(rellenar, imagen completa, estirar o centrar), el fundido entre fondos, el sonido de los vídeos,
los fotogramas por segundo con batería (60, 30, 15 o 10) y con el cargador (60 o 30), cuándo cambian los
colores del sistema y los gráficos (automático, Vulkan u OpenGL ES). Debajo, el diagnóstico muestra
la GPU, los fotogramas dibujados y por segundo, el límite que se aplica en ese momento, el estado de
carga, ahorro de energía y temperatura y, con una escena, la clase del teléfono (y si se midió en su
GPU), el nivel de calidad y por qué bajó. Los avisos que vienen del motor (los fallos de un
paquete, de una propiedad, de la profundidad o de la sincronización) llevan un identificador de
mensaje y se muestran en el idioma del teléfono con las mismas traducciones que el resto de la app.
*Licencias* muestra los textos de ONNX Runtime, de los componentes que incluye y de los modelos
de profundidad y de relleno (LaMa). El diagnóstico dice también si la clase del teléfono se midió con marcas de
tiempo de la GPU, que cuentan solo el trabajo de la GPU, o esperando a que termine, en los
teléfonos que no las tienen; el escritorio mide igual.

Los vídeos (MP4, MOV, 3GP, WebM o MKV) se repiten en bucle. Con el ajuste *Rellenar* o *Estirar*
los decodifica el hardware del teléfono directamente sobre el fondo, sin pasar por la GPU ni copiar
fotogramas; con *Rellenar* piden a Android que llenen la pantalla recortando los bordes, un recorte
que hace el compositor del sistema, así que un teléfono que no lo respete los mostrará estirados.
Con *Imagen completa* o *Centrar* se dibujan con la GPU, como los vídeos de las escenas (el hardware
los decodifica y cada fotograma se copia a la GPU, como mucho a los fotogramas por segundo
permitidos), con franjas del color de fondo alrededor. Se pausan en cuanto el fondo deja de verse y
sueltan el decodificador si siguen ocultos más de 30 segundos; al volver siguen por donde iban. Si
inicio y bloqueo muestran el mismo vídeo, comparten un solo reproductor. Van sin sonido salvo con
*Reproducir el sonido de los vídeos*. El fundido entre fondos sirve entre todos los tipos: de una
imagen, una escena o un vídeo a otro de cualquier tipo, el que sale y el que entra se mezclan; un
vídeo entra o sale por la GPU durante el fundido y después vuelve a la ruta directa sin cortarse ni
volver a empezar. Al añadir un vídeo, la app comprueba que el teléfono tiene un decodificador para
su pista de vídeo y saca su primer fotograma, como el escritorio comprueba que GStreamer puede
abrirlo; si no puede, no lo añade y lo cuenta en el aviso de los archivos que no se pudieron abrir.
Si el teléfono solo puede decodificar un vídeo con el procesador (no tiene un decodificador por
hardware para su formato, su tamaño o su velocidad) pero sí decodifica por hardware H.264 a ese
tamaño o a 1080p o 720p, la app lo convierte al añadirlo a una copia H.264 con el codificador del
propio Android (Media3 Transformer, sin codificadores añadidos al APK), mostrando el avance con un
botón *Detener* que deja el vídeo como estaba; mientras convierte, una notificación silenciosa y continua, en su propio canal, muestra el avance, cuántos esperan y un botón *Detener*, y desaparece al terminar o detener. La primera conversión pide el permiso de notificaciones con una línea que explica para qué; si lo rechazas, convierte igual sin notificación y no vuelve a preguntar. *Convertir los vídeos que el teléfono no reproduce
con fluidez* en *Ajustes* elige entre *Convertirlos* (la copia toma el lugar del original, que se
borra, y conserva sus listas y su elección), *Convertir y conservar el original* (la copia se
añade como otro vídeo) y *Nunca*. Con el ahorro de energía no convierte nada al añadir y lo avisa;
la página de ese vídeo dice por qué conviene convertirlo y ofrece *Convertir para este teléfono* y
*Convertir y conservar el original*, que funcionan también con el ahorro de energía porque lo
pides tú. La conversión pide a WorkManager que mantenga viva la app mientras dura (sin
batería baja) y anota lo que falta en un archivo: si Android cierra la app antes, la conversión se
retoma desde cero en cuanto arranca otra vez la app o WorkManager la despierta, y se descarta si el
vídeo ya cambió o no existe. Los vídeos HDR de 10 bits (PQ o HLG) salen siempre en H.264 de 8 bits
con el rango estándar (mapeo de tonos) y el audio que el contenedor MP4 no admite se recodifica a
AAC. Los vídeos que el teléfono ya decodifica por hardware se reproducen tal cual.

En la página de una imagen, *Moverse con el teléfono* activa el paralaje. Sigue el giro del teléfono
con el sensor de rotación que no usa la brújula o, si no lo hay, con el normal: girarlo hacia un
lado mueve la imagen en horizontal y inclinarlo hacia delante o hacia atrás, en vertical, también
con el teléfono de pie. Sin giroscopio sigue la inclinación con el acelerómetro, y si el teléfono no
tiene ningún sensor o no da datos, el paso de páginas del lanzador. El giro se filtra y se recentra
solo en unos segundos, con el mismo muelle y el mismo sombreador que el escritorio: con un mapa de
profundidad y sus capas lo cercano se mueve más que el fondo, y sin ellas la imagen queda quieta.
*Intensidad* y *Enfoque* (la profundidad que se queda quieta) son los mismos valores que en el
escritorio, pero en el teléfono la intensidad se mide sobre la parte que se ve y vale el triple,
porque la pantalla es pequeña y se mira de cerca: con los valores por defecto lo cercano se desplaza
hasta un 4 % del ancho de la pantalla y el fondo hasta un 2 % hacia el otro lado, y la imagen se
amplía lo justo para no enseñar los bordes; las capas se calculan para ese movimiento. La página muestra si la imagen aún no tiene mapa de
profundidad, si se está calculando o si ya está listo, y avisa cuando el movimiento está en pausa
por el ahorro de energía, por poca batería o por calor; con una batería normal, con o sin cargador,
nunca se para. El sensor solo se enciende mientras se ve un fondo con paralaje, a los fotogramas por
segundo permitidos (60 como mucho), y se apaga al taparlo. Con el teléfono quieto no se dibuja
ningún fotograma, y con batería el sombreador da como mucho 8 pasos de profundidad.

En el teléfono, con un mapa, el motor también calcula las capas (en segundo plano, como el mapa, con
el mismo progreso y *Cancelar*) y dibuja con ellas; En los ajustes de paralaje de cada imagen, *Relleno detrás de lo cercano* elige `simple`
(por defecto) o `lama`, y *Capas de profundidad* de 4 a 6, con una nota visible de su coste. LaMa es opcional: gasta más batería y
memoria y tarda minutos por imagen, así que está apagado por defecto, se descarga solo si lo eliges
(208 MB, Apache-2.0, huggingface.co, con el mismo permiso previo y su SHA-256) y nunca corre con el
ahorro de batería, que usa el relleno simple. *Construir las capas de profundidad* las rehace con los
ajustes actuales, y el teléfono sigue sin herramientas de autor.

*Calcular un mapa de profundidad* usa el mismo modelo que el escritorio, Depth Anything V2 Small,
con ONNX Runtime 1.31.0, que ya no va dentro del APK. La primera vez pide permiso para descargar los
dos y muestra su tamaño (154,2 MB en total: 99,1 MB del modelo y 55,1 MB del paquete de ONNX Runtime
para Android), sus licencias (Apache-2.0 y MIT) y los servidores (github.com y repo1.maven.org). Las
descargas llevan el User-Agent genérico del escritorio, se reanudan si se cortan y se comprueban
con su SHA-256 fijado; del paquete de ONNX Runtime solo se guarda la biblioteca de la arquitectura
del teléfono (unos 34 MB en arm64-v8a), comprobada también con su propio SHA-256, de solo lectura
y con el texto de su licencia al lado, y el paquete se borra. El mapa se calcula en segundo plano
en el propio teléfono, con la telemetría de ONNX Runtime apagada, y la foto no sale del teléfono;
*Cancelar* detiene la descarga o el cálculo en cuanto se pulsa, también a mitad del modelo. Con un
mapa, la página muestra los mismos ajustes que el escritorio (`reiwand-ctl edit depth`): nivel
cercano y lejano, curva, suavizado y cuánto crecen los bordes cercanos, que cambian el paralaje y
las máscaras de profundidad al soltarlos, el mapa ajustado en gris o en *Falso color: rojo cerca y
azul lejos*, como en la app, y *Quitar el mapa de profundidad* borra el mapa calculado y el
importado. *Ejecutar el modelo en* elige, como en la app, el dispositivo más rápido o el procesador
(en el teléfono el más rápido es hoy el procesador, porque su ONNX Runtime no trae aceleradores), y
la página avisa, como el escritorio, si la imagen cambió después de editarla. *Usar un archivo de mapa de profundidad*
importa un mapa hecho en el escritorio, el PNG de 16 bits que guarda junto a la imagen (por ejemplo
`foto.jpg.depth.png`), si tiene la misma proporción. En *Ajustes*, *Permitir descargar el modelo de
profundidad* (`models.allow_download`) bloquea la descarga, y ahí mismo se puede borrar el modelo.

Los filtros *Escenas* y *Páginas web* de la biblioteca tienen *Fondos de ejemplo*, que
vuelve a añadir el repositorio local «Example wallpapers» que la app añade al primer arranque (con las
seis escenas y la página web *Luces* que trae el APK, los mismos ejemplos que el escritorio) y abre
*Explorar*, donde se instalan como los de cualquier repositorio, y *Abrir un paquete*, que importa un `.wpak` con el selector de documentos de Android.
Se admiten los cuatro tipos de paquete que son un fondo, como en el escritorio: los de imagen se
muestran como una foto más (con su ajuste, su encuadre, su edición y su paralaje), los de vídeo como
un vídeo más, los de escena con el motor de escenas y los de página web con el WebView del sistema
(ver [«Fondos web»](fondos-web.md)); un preajuste publicado aparte se muestra con su paquete base y sus valores (la
vista previa los usa sin guardarlos), y al usarlo como fondo escribe sus valores en los ajustes de
la base; su página nombra la base con *Mostrar* o, si falta, *Instalarlo* desde el repositorio que la
ofrece, y la página de la base lista sus preajustes, como en el escritorio. Los
de recursos se rechazan con un aviso, igual que en el escritorio. La imagen o el vídeo del paquete se extrae al importarlo, por partes y sin cargarlo
entero en memoria, a los archivos de la app y se borra con el paquete. Las escenas se dibujan con el mismo motor que el
escritorio: capas, efectos, partículas (en la GPU si admite sombreadores de cálculo y, si no, en la
CPU con menos partículas), textos, líneas de tiempo y propiedades. La página de una escena muestra
sus propiedades y sus preajustes, que se guardan en la misma configuración que el escritorio; los
preajustes propios se guardan, se renombran y se borran ahí, y se exportan e importan como archivo,
los botones de la escena funcionan y los textos secretos quedan sellados por el almacén de claves de
Android. Las propiedades de archivo y de carpeta se eligen con el selector de documentos de
Android: el archivo elegido (una imagen o, si la propiedad los admite, un vídeo) se copia a los
archivos de la app, y de una carpeta se copian las imágenes y los vídeos que admite, con sus
subcarpetas hasta la profundidad que pide y como mucho los archivos que pide. La app guarda el
permiso de lectura de la carpeta y copia otra vez lo que cambió en ella cada vez que se abre y,
para las carpetas del propio teléfono, en segundo plano cuando Android avisa de un cambio en la
carpeta o en sus fotos y vídeos: WorkManager espera a que los cambios se calmen 10 segundos (2 minutos como mucho), sin
batería baja y sin consultar nada a intervalos, así que la copia suele ponerse al día en menos de un
minuto. Los avisos de una nube o de la red no despiertan la copia, porque muchas avisan después de
cada lectura: su carpeta se pone al día al abrir la app y, en segundo plano, después de cada copia
de una carpeta del teléfono y una vez al día, solo por Wi-Fi u otra red sin límite de datos (salvo
*Usar también datos móviles en segundo plano*, en los ajustes; sin esa red espera a una, sin consultar nada
mientras tanto), y la carpeta de una nube que no avisa se pone al día sin abrir la app;
*Quitar* borra la copia y devuelve el permiso. Como en el escritorio, la escena muestra el archivo
de la carpeta que toca, en orden o al azar y cada el tiempo que pide, y solo cambia mientras se ve.
La página de un paquete de vídeo tiene sus ajustes (velocidad, inicio, final, volumen, zoom y
posición) y sus preajustes, como en el escritorio, con un recorte de dos asas sobre la duración real del vídeo (se mide al abrir la página si la biblioteca no la tiene, sin tope de una hora) y un encuadre que se arrastra sobre el fotograma de portada; con zoom o posición el vídeo pasa por la
gráfica, y con límite de fotogramas sigue directo en la pantalla, que se pide a esa frecuencia y a la
que además se le saltan los fotogramas que sobran antes de mostrarlos, así que una pantalla que
no baja a esa frecuencia tampoco recibe más (el decodificador sigue decodificándolos todos). El límite de fotogramas del autor (en los vídeos, con
ese matiz), el inicio en silencio y la
reproducción única valen para vídeos y páginas como en el escritorio (al terminar, el último
fotograma queda en pantalla sin volver a dibujarse, también si después cambian el encaje, el
filtro, el volteo, el zoom o la posición), y una página sin volumen
propio tiene el ajuste *Volumen*, que se multiplica por el sonido de los vídeos. Un vídeo en
silencio o con volumen 0 no descodifica su sonido ni ocupa la salida de audio. Cuando dos pantallas
o una vista previa muestran el mismo vídeo con ajustes distintos (un preajuste y su base, por
ejemplo), cada una lo reproduce con los suyos al verse.
La página de un paquete avisa de todo lo que el teléfono omite: el color alrededor de la imagen y la
extensión sobre varias pantallas, los sonidos nuevos, lo que no tiene sentido en el teléfono
(los sonidos de los scripts) y la iluminación RGB. Los scripts de escena guardan datos, crean formas,
mueven la cámara, cambian los padres de las capas y listan la carpeta elegida igual que en el
escritorio. *Ajustes > Filtro* elige el filtro de color de la pantalla de inicio y el de la de
bloqueo, con los mismos aspectos, cada uno con una muestra de colores ya filtrados, y deslizadores (fuerza, exposición, contraste, saturación, calidez
y tono, con su valor a la vista) que la app de escritorio y *Mostrar tal cual*; la de bloqueo usa el
de inicio mientras *El mismo filtro que la pantalla de inicio* está activo, y el fondo cambia al
mover cada control. El filtro y el volteo de imágenes y
vídeos se aplican también aquí; un vídeo con filtro o volteado pasa por la gráfica en vez de
reproducirse directo en la pantalla. Los tonos de un vídeo HDR (PQ o HLG) los adapta el sistema
cuando se reproduce directo y la propia app cuando pasa por la gráfica, con la misma curva que el
escritorio (blanco de 203 nits y luz máxima de 1000 nits); si `playback.hdr` enciende el HDR de la
pantalla, la app conserva las luces por encima del blanco hasta lo que la pantalla admite, también
con filtro, y si el decodificador ya entrega los fotogramas en colores estándar los muestra tal cual
sin encender el HDR. Cuando una foto o nada ocupa el lugar del vídeo, la pantalla sale del HDR. Si un paquete pide sombreadores de cálculo y la gráfica del teléfono no los tiene, su página lo
avisa. El color de acento del fondo (o, si no propone uno, el dominante de su vista previa) es el
color que Android da al sistema. Un paquete de más de 1 GiB descomprimido se rechaza. Al instalar un
paquete de un repositorio también se instalan del mismo repositorio los paquetes que nombra en
`requires.packages` y que faltan (la versión más nueva que cumpla el rango), y los que piden estos,
hasta cuatro niveles; un preajuste o un paquete de recursos ya instalado cuenta como presente. Si una
dependencia no se pudo instalar, el teléfono lo dice con un aviso que nombra lo que faltó (también al
actualizar o al instalar la base de un preajuste) y el escritorio igual: el trabajo de instalación termina
con ese texto y el resultado trae `missing`. Un paquete hecho solo para
el escritorio se rechaza con ese aviso, traducido. El nivel de calidad sale de la
clase del teléfono (básica con poca memoria o sin sombreadores de cálculo, alta si el fabricante
declara una clase de rendimiento de Android 13 o posterior, media en el resto, ajustada con la misma
medida corta que en el escritorio y guardada en los archivos de la app) y de la batería: con
batería baja un nivel, y con calor baja más; si los fotogramas tardan demasiado en la GPU baja otro,
y solo vuelve a subir con el cargador puesto y el teléfono frío. En las GPU de teselas de los teléfonos cada
pasada de render cuesta cerca de un milisegundo por sí sola, más que sus píxeles, así que allí la
escena dibuja menos pasadas: el primer efecto de una imagen, un vídeo o una malla lee la imagen donde
está en lugar de una copia de la capa, una malla oculta que da su máscara a otra del mismo lugar y
tamaño se lee sin dibujarla aparte, y los efectos que se funden en una pasada siguen fundidos cuando
el nivel quita uno de en medio. En el teléfono de pruebas (60 Hz, pantalla apagada) una escena con
una imagen y dos olas bajó de 15,5 a 9,9 ms por fotograma en Q0 y otra con una malla animada de 16,9
a 15,1 ms en Q2. Cada fotograma llega a la GPU en un solo envío, con las mallas posadas, los moldes,
los modelos 3D y las partículas dentro, así que el tiempo que miden los niveles los incluye; las
mallas de una misma pose se posan juntas en una pasada y las capas en modo exclusión se mezclan sin
copiar el lienzo. Con eso la escena de la malla animada pasó de 50 a 57 fotogramas por segundo
mostrados en Q0 y la de las dos olas de 57 a 59. Las capas en modo aclarar u oscurecer siguen
copiando lo que tienen debajo y cortando la pasada del lienzo. El motor de la escena sigue vivo
cuando el sistema cambia la superficie del fondo, así que no se vuelve a cargar ni compilar. Las capas con
paralaje de una escena siguen la inclinación del teléfono con los mismos sensores que el paralaje de
las fotos, aunque la escena no lo pida con `gyro`, y sin sensores, el paso de páginas del lanzador;
con el ahorro de energía se quedan quietas. Con la calidad automática, una escena demasiado pesada
para el teléfono se queda quieta en su fotograma de muestra; con un nivel fijo o un mínimo elegido
se reproduce igual y los ajustes avisan de lo que cuesta. Cada escena dibuja a sus propios fotogramas por segundo, como
mucho a los de los ajustes, solo mientras algo cambia y nunca mientras no se ve. Los vídeos dentro
de una escena los decodifica el hardware y se copian a la GPU fotograma a fotograma, sin sonido, y
se pausan si la escena se queda quieta. Los scripts de las escenas se ejecutan en el teléfono como en
el escritorio, en un hilo propio con los mismos límites (QuickJS sin archivos ni red, 32 MiB de
memoria y 4 ms por fotograma); si uno falla tres veces seguidas se detiene y la escena sigue sin él,
y el diagnóstico de *Ajustes* lo dice. Sus sonidos suenan en el teléfono (ver arriba) y su volumen
aparece entre las propiedades. El diagnóstico también dice si los efectos de los paquetes se
cronometran con marcas de tiempo de la GPU o esperando a que termine, y al cerrarse el fondo el
teléfono termina las compilaciones de sombreadores y limpia sus marcas de prueba, para que un cierre
normal no cuente como un efecto que colgó la GPU.

Las miniaturas de los paquetes para adultos salen difuminadas y su página no los muestra hasta
pulsar *Mostrar igualmente*, salvo que *Ajustes* muestre el contenido para adultos sin difuminar.
*Ocultar el contenido para adultos* (`library.hide_mature`, desactivado por defecto), justo encima,
los saca de la biblioteca y de sus búsquedas, igual que en la app de escritorio y en `reiwand-ctl`, salvo
que la búsqueda pida `content:` ella misma. Un paquete fotosensible lleva el aviso «destella y está deshabilitado» y no se previsualiza ni se
usa como fondo mientras el ajuste general de destellos siga apagado (viene apagado); las listas lo
saltan. El contenido para adultos se recuerda por paquete. La biblioteca y la página de un paquete muestran su título y su descripción en el
idioma del teléfono, y la página avisa de cada parte que no funciona en él, de sus sonidos a sus
ajustes de vídeo y las funciones nuevas que omite.

*Mostrar lo que suena* aparece en las escenas y páginas que declaran `media` y también empieza
apagado. Android solo deja ver los reproductores de otras apps con el acceso a las notificaciones:
al activarlo, la app explica para qué lo usa y abre ese ajuste. Con los dos permisos, el teléfono
sigue la sesión multimedia activa por sus avisos, sin consultarla periódicamente, solo mientras se ve
un fondo que la usa, y le da lo mismo que el ordenador: textos, carátula reducida a 256 píxeles,
colores, posición, disparadores `media:` y eventos de scripts; nunca lee las notificaciones ni guarda,
registra o envía lo que suena. Debajo del interruptor dice si ahora se lee, cuántos reproductores
encontró y si hay pista, si suena y si tiene carátula, nunca cuál. Una página que además puede salir
a internet pide un segundo permiso, con su propio diálogo al encenderlo.

*Reaccionar al sonido*, *Reaccionar al tacto* y *Reaccionar a los toques* solo aparecen en las
escenas y las páginas web que los usan y se activan paquete a paquete; empiezan apagados. El sonido se lee con el visualizador de Android, que da un espectro de
poca resolución de lo que suena en el teléfono; Android pide el permiso del micrófono la primera vez
que se activa y muestra su indicador mientras escucha. Solo escucha mientras se ve una escena con
ese permiso, y solo calcula bandas en memoria. Tras dos segundos de silencio deja de leerlo hasta que
algo empieza a sonar; si un reproductor sigue activo en silencio, lo mira cada dos segundos. El tacto
es la posición del dedo sobre el fondo en la pantalla de inicio, que la escena sigue como el puntero,
y los toques son los clics del teléfono: un toque sobre una capa `solid` arranca sus líneas de tiempo
`click:` y llega a sus scripts. Solo cuentan los toques reales sobre la pantalla de inicio (la de
bloqueo no da ninguno) y sobre la vista previa de la página de la escena, y solo en las escenas
con ese permiso; sin él, tocar la vista previa no le llega a la escena.

El sonido de las escenas suena en el teléfono con las mismas reglas que en el escritorio: las
escenas empiezan en silencio (*Reproducir el sonido de los vídeos y las escenas* en *Ajustes*, que
es `playback.muted`), `playback.scene_sound` en `false` las deja mudas, el volumen es
`playback.volume` por el ajuste *Volumen* de la escena (que aparece en las escenas con sonido) y
*Silenciar el fondo mientras otra app reproduce sonido* lo baja con el mismo fundido. Solo suena en la
pantalla de inicio: ni en la de bloqueo ni en la vista previa, ni cuando el fondo no se ve, y se
calla con el ahorro de energía, con poca batería sin cargador o con mucho calor, igual que el
movimiento de la escena. Todas las capas de una escena se mezclan en una sola salida de AAudio, que
se abre cuando algo suena y se cierra cuando todo calló y pasaron cinco segundos; los sonidos se
descodifican con los códecs de Android la primera vez que se oyen (los largos con `stream`, a
medida que suenan), se realinean con la escena una vez por segundo y llegan al altavoz unos
200 ms después de la escena, así que los sonidos que arranca un toque van algo más tarde que en el
escritorio. Un sonido que arranca por un evento o por un script respeta igual el silencio y el volumen elegidos en Ajustes.
No hay red ni permisos nuevos: la salida de sonido no pide el micrófono (eso solo es *Reaccionar al
sonido*) y lo que suena en un fondo no sale del teléfono.

Solo se dibuja cuando cambia algo y nunca mientras el fondo no se ve: con una imagen fija no hay
ningún fotograma más. El fundido y el desplazamiento al pasar de página en el lanzador van como
mucho a los fotogramas por segundo elegidos (60 con batería y con el cargador si no se cambian, y
30 en la vista previa), se detienen con el ahorro de energía o con poca batería y bajan cuando el
teléfono se calienta. Los vídeos se pausan con el ahorro de energía, con poca batería o con el
teléfono muy caliente. Los colores del sistema (Material You) salen de la imagen o del primer
fotograma del vídeo y se calculan una sola vez al añadirlos. *Colores del sistema* decide si Android
cambia de tema al elegir un fondo (lo normal), también en cada cambio de una lista o nunca, y Android
lo aplica la próxima vez que se apaga y enciende la pantalla.
