# Uso desde la terminal

Si lo que lee la salida de `reiwand-ctl` deja de leer antes de tiempo (por ejemplo `| head` o
`| grep -q`), `reiwand-ctl` termina sin error ni aviso.

```sh
reiwand-ctl set ~/Imágenes/fondo.jpg               # todas las pantallas
reiwand-ctl set ~/Vídeos/mar.mp4 -o DP-1 -f fit    # una pantalla, modo de ajuste
reiwand-ctl fit center                             # solo el modo de ajuste, mismo fondo
reiwand-ctl fit tile -o DP-1                       # el modo de ajuste de una pantalla
reiwand-ctl background '#203040'                   # color alrededor del fondo
reiwand-ctl clear                                  # quita el fondo y devuelve la pantalla al escritorio (en Plasma: color de fondo)
reiwand-ctl plasma-back                            # en Plasma: vuelve al tipo de fondo propio de Plasma
reiwand-ctl clear -o DP-1 && reiwand-ctl restore -o DP-1   # quitar el fondo de una pantalla y recuperarlo
reiwand-ctl filter warm --strength 0.6 -o DP-1      # filtro de color de una pantalla (none lo quita)
reiwand-ctl filter --list                          # los filtros que hay
reiwand-ctl pause | resume | toggle                # pausar los fondos animados
reiwand-ctl volume 40 | mute | unmute              # sonido de los vídeos y las escenas
reiwand-ctl sound off -o DP-1                       # el fondo que se ve ahí, sin sonido donde sea que se muestre (on, follow); --volume 30 lo baja
reiwand-ctl lib apply UID -o DP-1 --sound off      # aplicar un vídeo ya sin sonido (lo recuerda ese fondo; es un ajuste local, no va en el paquete)
reiwand-ctl config pause.on_battery below:20      # pausar con batería por debajo del 20 % (never = nunca, always = siempre)
reiwand-ctl config playback.lowest_below_percent 30  # calidad mínima de las escenas con batería por debajo del 30 %
reiwand-ctl playlist add Mar --folder ~/Fotos/Mar --follow   # carpeta entera; sigue a la carpeta
reiwand-ctl playlist rotate --folder ~/Fotos/Mar --every 10m   # crea la lista, pone el tiempo y la empieza
reiwand-ctl config                                 # ver los ajustes
reiwand-ctl config transition.duration_ms 1000     # cambiar un ajuste
reiwand-ctl state                                  # estado en JSON
reiwand-ctl gpu                                    # tarjetas gráficas y la que está en uso
reiwand-ctl gpu dedicated                          # dibujar con la dedicada desde el próximo arranque
reiwand-ctl quality q3                             # calidad fija de las escenas (auto, q0-q3, min-q1, min-q2)
reiwand-ctl reload                                 # releer la configuración
```

Modos de ajuste: `fill`, `fit`, `stretch`, `center`, `tile` y `span` (una imagen, un vídeo o una
escena sobre todas las pantallas, con su tamaño físico y los marcos entre ellas). Una escena
extendida se dibuja en cada pantalla solo en su parte, con su propio nivel de calidad y un mismo
reloj para todas, así que sus animaciones siguen a la par de una pantalla a otra. Si el autor la
hizo para varias pantallas (`display.span: "all-outputs"`), la app propone `span` al aplicarla y
muestra dónde están las pantallas; cada fila de pantalla lo deja cambiar y abre el mismo esquema.
`reiwand-ctl lib apply ID --span` hace lo mismo desde la terminal. Las salidas que no dicen dónde
están (transmisiones, vistas previas o pantallas que el escritorio no ubica) muestran la imagen o la
escena entera, salvo que el esquema de pantallas de la app las coloque a mano sobre el tablero o
les haga mostrar la misma parte que otra salida (`layout.monitors.<salida>` y
`layout.follow.<salida>` en la terminal). Las pantallas que muestran la misma
imagen con la misma edición, como las de `span` o las que tienen su propio recorte, comparten una
sola copia en la GPU, y los JPEG grandes se decodifican directamente a un tamaño reducido. Con una
escena pasa lo mismo: cada imagen del paquete se sube una vez y la usan todas las pantallas que la
muestran, la copia decodificada sale de la memoria en cuanto está en la GPU, los sombreadores se
compilan al cargar la escena y un fotograma estable no crea recursos ni reserva memoria propia,
tampoco al evaluar la escena, que reutiliza las listas del fotograma anterior.
Ajustes: transición (`fade` o `none` y su duración), fotogramas por segundo y cuándo pausar
(ventana encima, batería,
bloqueo). Un ajuste desconocido termina con error y muestra la lista de ajustes válidos.

`playback.max_fps` vale `display` por defecto (*Igual que la pantalla* en la app):
con corriente cada pantalla se dibuja como mucho a su frecuencia de refresco (la que da el
compositor al presentar cada superficie o la del modo de `wl_output`, la de RandR en X11 o, en
Plasma, la que da KWin) y con batería a esa frecuencia hasta 60 fotogramas por
segundo, porque la batería baja la calidad (resolución y efectos) y no el ritmo; un número fija el tope (`reiwand-ctl config playback.max_fps 60`). Salvo en los vídeos, la
GPU pone su propio límite: el render por software se queda en 24, una GPU
integrada en 60 salvo escenas ligeras, y una dedicada sigue a la pantalla hasta 240 (60 con
batería). Los vídeos se quedan en 30 con `display` y render por
software y nunca pasan de sus propios fotogramas, nada se dibuja si nada cambia y `reiwand-ctl state`
muestra la frecuencia (`refresh`) y el tope (`fps`) de cada pantalla.

Las escenas calculan antes de dibujar cuánto cuesta cada fotograma: cuentan sus efectos, las capas
que se dibujan aparte y las mezclas que copian el fondo, cada una por la parte de la pantalla que
ocupa, y eligen el nivel de calidad más alto que cabe en la GPU. Los efectos de una imagen, un vídeo
o una malla corren a la densidad de su imagen: si la pantalla la amplía (un lienzo de 2560×1440 en un
teléfono de 1440×3040 muestra cada texel en 2,1 píxeles), trabajan sobre sus propios texeles y el
resultado se amplía al componer, como la imagen misma, así que en Q3 se ven igual y cuestan una
cuarta parte; Q2, Q1 y Q0 bajan esa densidad a 0,75, 0,6 y 0,5, y el coste las cuenta así. Si la
pantalla amplía el lienzo, Q2 lo dibuja a un píxel por unidad del lienzo (entre tres cuartos y el
tamaño completo) y Q1 a 0,85 (entre 0,6 y tres cuartos), píxeles que no tendrían más detalle. Las GPU integradas antiguas (Intel
HD y UHD, las APU de AMD con Vega y similares) y los equipos con menos de 4 GiB de memoria tienen un
nivel intermedio propio. La primera vez que se usa una tarjeta gráfica con un driver, un render
corto de unos 300 ms mide cuánto tarda y ajusta su clase, que se guarda en
`~/.local/state/reiwand/gpu-tier.json`, una por tarjeta, y se vuelve a medir solo si cambia el
driver de esa tarjeta. La medida mueve la clase un paso como mucho, y si la baja (un arranque con el equipo ocupado
puede hacerlo) se repite en los arranques siguientes, hasta tres veces, y cuenta la mejor. Mientras
la escena se mueve, el tiempo que cada fotograma añade en la GPU, partículas incluidas, guía la
calidad (lo que va desde que la GPU termina el fotograma anterior hasta que termina este, así que no
cuentan ni la espera detrás de un fotograma que sigue en la GPU ni las pasadas que corren a la vez en
las GPU de teselas; las medidas por pasada solo reparten ese tiempo entre los efectos). Sin marcas de tiempo en la GPU
mide uno de cada quince fotogramas esperando a la GPU. En el ordenador y en el teléfono el nivel se guía solo por el
tiempo medido, con el mismo piloto: el modelo de costes solo da el nivel de los primeros fotogramas, y desde el
duodécimo cuenta la media de los últimos 32 (sin los que pasan del doble o no llegan a la mitad de
su mediana, para que un pico suelto no cuente) frente al intervalo del ritmo con que se dibuja (el
tope que elijas o el de la pantalla). Si durante 2 s pasa del 85 % del intervalo, baja un nivel; si
durante 6 s no llega a la mitad (o, cuando ya midió el nivel de arriba, si la media por lo que ese
nivel encarecía cabe en tres cuartas partes), sube uno, hasta Q3 (menos con ahorro de energía,
calor o el tope del paquete); en el teléfono también con batería, en el ordenador no sube con batería y
conserva el motivo («con batería») mientras se queda en el nivel de partida. Entre dos cambios pasan al menos 5 s, y una subida que vuelve a bajar
en menos de 30 s duplica la espera de la siguiente (hasta cuatro veces), así que no salta entre dos
niveles. Con `auto`, una escena que en el nivel más bajo tarda más del doble del intervalo durante
2 s queda quieta en su póster hasta que cambies la elección o se vuelva a cargar. *Ajustes* muestra
el tiempo medido, el objetivo y la última decisión, y la frecuencia no depende del nivel: 60
fotogramas por segundo con batería salvo que elijas otra, salvo por lo que sigue. El piloto decide un par, el nivel y un
divisor del ritmo, y juzga contra el intervalo del ritmo en uso. Con `playback.auto_pace` en `detail` (el valor por defecto)
y un ritmo de 60 o más, un nivel que no cabe en el intervalo entero pero cuya media prevista cabe en tres
cuartas partes del doble se alcanza a mitad de ritmo (30 fotogramas por segundo con 60 Hz, nunca menos de 30); si el
nivel a mitad de ritmo cabría también entero (menos de la mitad del intervalo entero durante 6 s) vuelve al ritmo
entero antes de subir otro nivel, y una sobrecarga baja primero a mitad de ritmo y solo después un nivel. Con `smooth`
el ritmo no cambia nunca, un nivel fijo tampoco, y el tope que elijas lo acota. *Ajustes* lo muestra en la decisión
(«q2 a 30 fps»), `reiwand-ctl config playback.auto_pace detail|smooth` y la app lo cambian; en el ordenador la mitad de
ritmo dibuja esa pantalla a la mitad de fotogramas, y `reiwand-ctl scene bench --quality auto` imprime el ritmo. Presenta con tres imágenes para que la GPU dibuje un fotograma
mientras se prepara el siguiente. `reiwand-ctl state` (`tier`,
`tier_measured`, y `quality` y `quality_reason` en cada pantalla), la app y la bandeja muestran la
clase medida, el nivel y por qué bajó, y también el tiempo de GPU medido por fotograma frente al
intervalo, el ritmo y la última decisión del piloto (`quality_frame_ms`, `quality_period_ms`,
`quality_pace` y `quality_step`; columna `MEASURED` de `reiwand-ctl quality`).

El nivel también se puede elegir a mano con `playback.quality`: `auto` (lo de siempre), un nivel
fijo de `q0` a `q3`, o `min-q1` y `min-q2`, que dejan la elección automática pero nunca por debajo de
ese nivel (puede subir más). Lo que elijas se respeta siempre, también con batería, con el ahorro
de energía, con calor y aunque la escena pese más de lo que aguanta la GPU: entonces la escena se
ve con todos sus detalles y con menos fotogramas por segundo. Solo la elección automática detiene
una escena: la que pesa más que el presupuesto del equipo aun en el nivel más bajo (solo en el ordenador)
o la que se midió demasiado lenta en ese nivel o dibuja más triángulos de modelos de los que admite
su clase se queda quieta en su póster con `auto`, se
reproduce en el nivel fijo elegido y, con `min-q1` o `min-q2`, en ese mínimo (o más arriba si
cabe). La app (junto a la elección de calidad en *Ajustes* y en cada pantalla con escena), la
bandeja, el teléfono (bajo su ajuste de calidad, del fondo de inicio o de bloqueo que dibujó
último, nunca de una vista previa, y sin aviso en cuanto ese fondo pasa a ser una imagen, un video o
una página), `reiwand-ctl quality` y `reiwand-ctl state` avisan de
lo que cuesta frente al presupuesto del equipo (en cuanto se miden sus fotogramas, de cuánto tardan
en la GPU frente al intervalo y a cuántos fotogramas por segundo, y no avisan si caben), que puede
perder fotogramas y gastar más batería, y
si la elección automática la bajaría o la dejaría quieta (`quality_auto` en el estado: `still` si
está quieta en su póster, `stop` si la reproduce un nivel fijo o un mínimo y `auto` la dejaría
quieta, `lower` si solo la bajaría); si solo
la pesan los triángulos de modelos y su costo cabe, avisan de que dibuja más triángulos de los que
aguanta el equipo y de que la elección automática la dejaría quieta. Solo se mantienen con
cualquier elección los límites que protegen la memoria y el controlador: los datos y triángulos de
modelos que puede cargar una escena, el tamaño de las texturas y los efectos que quedan fuera por
el presupuesto fijo o porque detuvieron la GPU, que el aviso cuenta. Las bajadas
automáticas solo valen en `auto` o por encima del mínimo elegido, y el motivo es `user`. La app lo ofrece en *Ajustes* → *Calidad de las escenas* y en cada pantalla con
escena o página web, con su propio valor (*Como todas las pantallas* vuelve al general; una página
toma el nivel de la pantalla que la muestra, y las pantallas con niveles distintos la ejecutan por
separado); el teléfono, en sus ajustes. `reiwand-ctl quality q3`, `reiwand-ctl quality min-q1 -o DP-1`, `reiwand-ctl quality none -o DP-1`
o `reiwand-ctl config playback.quality q2` y `reiwand-ctl config playback.quality.DP-1 q1` lo cambian, y
`reiwand-ctl quality` muestra la elección, el nivel y el motivo de cada pantalla;
`reiwand-ctl config playback.quality.DP-1` lee el valor propio de una pantalla aunque no esté
conectada.

Las pantallas HDR muestran los colores más brillantes que el blanco de las escenas con
`canvas.hdr`: si el escritorio y el controlador gráfico ofrecen una superficie HDR (en Wayland, con el
protocolo de gestión de color), la escena guarda 16 bits hasta el final y dobla lo que pasa de 0,8
del blanco hacia el brillo máximo de esa pantalla, que el escritorio comunica junto a su blanco de
referencia (si no lo dice, se supone el doble del blanco, una decisión que se mantiene). Los colores
van al escritorio en sRGB lineal extendido si el controlador lo ofrece; si solo ofrece PQ (BT.2100),
el daemon los codifica en PQ; y si no ofrece ninguno de los dos pero el escritorio acepta
descripciones de imagen propias, el daemon codifica en PQ y describe la superficie él mismo con
sus metadatos HDR (MaxCLL, la luz más alta que puede dibujar, y MaxFALL, su blanco). La app,
`reiwand-ctl quality` y `reiwand-ctl state` dicen cuál de las tres se usa. `playback.hdr` vale `auto` (solo si
el escritorio o el teléfono dicen que la pantalla muestra HDR, con corriente y con una escena con
`canvas.hdr` y mapeo de tonos en `q3`, con un vídeo HDR o con una imagen KTX2 HDR, porque gasta más), `on` (siempre que la superficie lo admita, aunque la pantalla no lo diga) u `off`, y cada pantalla puede tener el
suyo (`playback.hdr.DP-1`); si la pantalla no puede, se ve en SDR como siempre. Con el HDR encendido, un vídeo HDR (PQ o HLG)
conserva sus luces por encima del blanco, ajustadas al pico de la pantalla (el más bajo de las
pantallas que lo muestran); una imagen KTX2 HDR (UASTC HDR, sin retoques de foto) conserva también su
luz por encima del blanco, doblada con suavidad hacia el pico de la pantalla, en el escritorio y en el
teléfono; y las demás imágenes, los vídeos normales y las páginas, hechos para el rango
estándar, ponen su blanco en el blanco de la pantalla. La app lo ofrece en
*Ajustes* → *HDR* y en cada pantalla, con lo que la pantalla admite y por qué está activado o no; el
panel de tonos del editor avisa de cómo se ven esas escenas en una pantalla HDR. En el teléfono
(Android 14 o posterior) está en sus ajustes, la app usa `COLOR_MODE_HDR` en su ventana y el
diagnóstico dice si la pantalla muestra HDR y su pico. `reiwand-ctl quality --hdr on`,
`reiwand-ctl quality --hdr off -o DP-1` y `reiwand-ctl config playback.hdr.DP-1 auto` lo cambian, y
`reiwand-ctl quality` y `reiwand-ctl state` muestran el estado de cada pantalla. El ajuste no viaja en el archivo exportado ni entra en los perfiles, porque depende de cada equipo: al
usar un perfil, cada pantalla conserva su propio valor; las páginas web siguen el general. Con batería o con el perfil de ahorro de energía de power-profiles-daemon
las escenas bajan de nivel y de fotogramas; si un sensor del sistema dice que la GPU o la CPU están
cerca de su temperatura crítica, bajan más hasta que se enfrían. Cuando el escritorio pide reducir
el movimiento (`reduced-motion` del portal de ajustes), las escenas se quedan quietas en su
fotograma de muestra y solo se redibujan una vez por minuto para que los relojes sigan en hora,
salvo las escenas a las que dejes seguir moviéndose (*Seguir en movimiento cuando el escritorio pide
reducir el movimiento* en los detalles del paquete en la app, el mismo interruptor en su página del
teléfono, `reiwand-ctl lib confirm motion ELEMENTO` y `reiwand-ctl lib unconfirm motion ELEMENTO` para
volver atrás), pensado para escenas tranquilas: no se ofrece para los paquetes fotosensibles. Los
efectos lentos, como las nubes o las ondas, declaran a cuántos fotogramas por segundo se ven bien y
una escena que solo se mueve por ellos no pasa de ahí. Cada fotograma redibuja solo la parte de la
pantalla que cambió y, si nada cambió, no dibuja nada; las partículas quietas y las escenas
detenidas en un fotograma no cuentan como cambio. Con render por software cada escena se dibuja a
como mucho un megapíxel por núcleo y se escala a la pantalla, la espera a la GPU llega hasta 60 s
con pocos núcleos (tras un fotograma con efectos de paquetes, también al componerlo, la espera sigue
siendo de 10 s para detectar cuelgues) y, si un
fotograma tarda más de un octavo de esa espera, la resolución baja por pasos y vuelve a subir cuando
sobra tiempo, así que un fotograma pesado nunca se toma por una GPU colgada. Los desenfoques y las deformaciones ya no
oscurecen los bordes derecho e inferior, y el desenfoque de movimiento puede seguir la velocidad de
su capa (`follow`).

En un equipo con una GPU integrada y otra dedicada, el daemon dibuja por defecto con la misma que el
compositor, para que los fotogramas le lleguen sin copiarse (al arrancar le pide la «retroalimentación»
de `linux-dmabuf-v1`, de donde saca el bus PCI de su tarjeta; en Plasma, KPipeWire solo ofrece los
modificadores de esa tarjeta, y con otra PipeWire cae a memoria y la CPU copia cada fotograma). Si el
compositor no lo dice (X11, sin esa versión del protocolo o una tarjeta que no se puede abrir) elige la
que gasta menos energía, que suele ser la integrada. `render.gpu` elige otra: `auto` (lo de siempre),
`integrated`, `dedicated`, `software` (render por software, para pruebas) o parte del nombre de una
tarjeta sin distinguir mayúsculas, como `Graphics A`, que sirve para elegir entre dos del mismo tipo.
La app lo ofrece en *Ajustes* → *Tarjeta gráfica* con las tarjetas que encontró el daemon (nombre,
tipo y API) y marca la que está en uso y la que dibuja el escritorio; si el compositor dibuja con otra
(porque se fuerza una tarjeta o por el recurso), la app, `reiwand-ctl gpu` (columna `DESKTOP`) y
`reiwand-ctl state` (`gpu_note`) avisan de que los fotogramas se copian por el procesador; `reiwand-ctl gpu` las lista y `reiwand-ctl gpu dedicated` o
`reiwand-ctl config render.gpu dedicated` elige una. El cambio vale desde el siguiente arranque del
daemon, porque el render se crea una sola vez y lo comparten todas las pantallas, vídeos y escenas;
hasta entonces la app, `reiwand-ctl gpu` y `reiwand-ctl config render.gpu` lo avisan (`gpu_restart` en
`reiwand-ctl state`), y la app ofrece *Reiniciar el daemon ahora* junto al aviso, igual que
`reiwand-ctl restart` (con systemd, se lo pide a systemd; si no, el daemon se vuelve a ejecutar a sí
mismo).; cambiar solo mayúsculas y minúsculas del
nombre no pide reiniciar. Si ninguna tarjeta coincide, o la elegida no se puede abrir, el daemon
arranca igual con la elección automática y lo avisa en la app, en `reiwand-ctl gpu` y en
`gpu_warning`. Una tarjeta que solo se alcanza con OpenGL ES, cuyo controlador no dice de qué tipo
es, toma el tipo que el sistema le da en `/sys/class/drm` (Intel integrada en el bus PCI 0 o, sin
dirección PCI, según su modelo; NVIDIA dedicada; AMD integrada con 2 GiB de memoria propia o menos;
con varias tarjetas de una marca y de tipos distintos, el de la que arranca la pantalla, o el de la
otra si `DRI_PRIME` pide otra), así que `integrated` también la elige. Si hay dos
tarjetas idénticas, la app las distingue por su bus PCI y `reiwand-ctl gpu pci:0000:03:00.0` elige la
de ese bus (la columna `BUS` de `reiwand-ctl gpu`); la lista sigue los nodos de `/dev/dri`, así que una
tarjeta que conectas o quitas aparece o se va al segundo, sin reiniciar ni consultar nada mientras
tanto. Con
`WGPU_BACKEND` puesto, el daemon solo carga los controladores de esa API, también
al listar las tarjetas. La clase de cada
tarjeta se mide y se guarda aparte, así que volver a una no la mide otra vez. La dedicada dibuja
las escenas pesadas con más calidad y gasta más energía. El ajuste no viaja en el archivo exportado ni entra en los
perfiles, y el teléfono, que tiene una sola GPU, no lo muestra y lo ignora.

Los fondos que destellan están deshabilitados por defecto: `playback.allow_flashing` (*Permitir los
fondos que destellan* en la página *Ajustes* de la app, que el botón *Ajustes…* de la bandeja abre, un
interruptor propio en la bandeja, `reiwand-ctl config playback.allow_flashing true` y un interruptor en los
ajustes del teléfono) viene apagado, y mientras esté apagado cada paquete fotosensible lleva el aviso
«destella y está deshabilitado» en las listas, no se reproduce y la rotación lo salta. Además, mientras
esté apagado el programa mide el brillo de cada fotograma que cambió de las escenas y, si una destella
más de tres veces por segundo, la reproduce más despacio o la detiene; `reiwand-ctl state` lo indica en
`flashes`. Esa medida cuesta una pequeña lectura de la GPU por fotograma que cambió; con el ajuste
encendido no se mide ni se avisa de nada.

Los streams llevan cada fotograma en memoria de la GPU (DMA-BUF) cuando el controlador Vulkan
puede exportarla y el consumidor (KPipeWire en Plasma) importarla: el daemon dibuja directamente en
los búferes del stream, sin leer el fotograma de vuelta ni copiarlo con la CPU, también con batería.
Donde la GPU no puede dibujar en el formato que pide el consumidor, como las imágenes lineales en
las GPU de NVIDIA, dibuja en una textura propia de cada pantalla y la copia en la GPU al búfer del
stream.
El daemon compone y envía solo el rectángulo que cambió: en memoria de la GPU dibuja en cada búfer
lo que cambió desde la última vez que lo usó, en memoria compartida lee de la GPU y copia solo esas
filas, y los dos tipos de búfer llevan `SPA_META_VideoDamage` con lo que cambió desde el búfer
anterior para los consumidores que lo leen. Un reloj pequeño sobre una imagen 4K copia así unos 55 KiB por
fotograma en vez de 32 MiB. Las superficies de Wayland y X11 y el teléfono siguen componiendo el
fotograma entero, porque la API gráfica no dice qué contiene cada búfer de la pantalla.
Si algo falta (OpenGL, render por software, un consumidor sin DMA-BUF), el stream usa memoria
compartida sin intervención; `REIWAND_DMABUF=0` en el entorno del daemon la fuerza y `REIWAND_DMABUF=1`
pide memoria de la GPU también con render por software. `reiwand-ctl state` muestra el tipo de búfer
de cada pantalla (`buffers`: `dmabuf` o `memfd`). Con `REIWAND_SYNCOBJ=1` en el entorno del daemon,
los búferes de la GPU ofrecen además sincronización explícita (syncobj) a los consumidores y
versiones de PipeWire que la admiten; es experimental y está apagada por defecto, así que sin esa
variable los búferes se sincronizan solo de forma implícita.

Cuando el decodificador de vídeo por hardware entrega los fotogramas en memoria de la GPU (DMA-BUF en
NV12, como VA-API en las GPU de Intel y AMD) y el controlador Vulkan puede importarlos con su
modificador, el daemon los lee en su sitio: convierte el color, gira el vídeo si lo pide y lo reduce
al tamaño de la pantalla en la GPU, sin pasar el fotograma por la memoria del sistema (cero copias
con la CPU por fotograma, frente a tres en la ruta de copia: bajar el fotograma, convertirlo a RGBA y
subirlo; con un vídeo 4K a 30 fps decodificado por VA-API en una iGPU de AMD, unos 0,3 ms de CPU
por fotograma en vez de unos 16 ms). Si el decodificador es por software, si el formato o el modificador no se pueden importar
o si la importación falla, el vídeo vuelve solo a la ruta de copia. En las GPU de NVIDIA, GStreamer
1.24 no entrega DMA-BUF (NVDEC da memoria CUDA u OpenGL y el controlador VA-API sobre NVDEC,
`nvidia-vaapi-driver`, solo memoria del sistema), así que allí los vídeos siguen copiándose, aunque
la importación funciona; con lavapipe y OpenGL no hay importación. `reiwand-ctl state` muestra la ruta elegida en `decode`
(`zero-copy` o `copy`) y `REIWAND_ZERO_COPY=0` en el entorno del daemon fuerza la copia. Pausar un vídeo
lo detiene donde está, sin volver a decodificar desde el fotograma clave anterior: en un vídeo 4K
con fotogramas clave cada 10 s, una pausa pasó de unos 2,3 s de CPU a menos de 20 ms.

Medido en una GPU dedicada de NVIDIA con el controlador propietario 615, GStreamer 1.24, un
daemon de `--release` en una instancia privada y un vídeo H.264 de 30 s (1080p60) o 20 s (4K60) en
una pantalla de su mismo tamaño, durante 30 s por prueba (media de dos rondas; la potencia de la GPU
varía unos 5 W entre rondas, probablemente porque la comparten otros procesos, y en reposo, sin vídeo, midió entre
17 y 32 W). «Copia» es lo que hace hoy el daemon (NVDEC por `nvh264dec`); «VA-API» fuerza el
decodificador `varenderD129h264dec` de `nvidia-vaapi-driver` (`NVD_BACKEND=direct`,
`GST_VA_ALL_DRIVERS=1`):

| Prueba | Fotogramas por segundo | CPU por fotograma | CPU del daemon | Memoria (RSS) | Potencia de la GPU | Uso de la GPU / NVDEC |
|--------|------------------------|-------------------|----------------|---------------|--------------------|-----------------------|
| 1080p60, copia | 59,8 | 4,7 ms | 28 % de un núcleo | 673 MiB | 45,7 W | 3 % / 2 % |
| 1080p60, VA-API | 59,8 | 4,5 ms | 27 % de un núcleo | 730 MiB | 46,3 W | 3 % / 2 % |
| 4K60, copia | 59,7 | 18,9 ms | 113 % de un núcleo | 1124 MiB | 48,4 W | 8 % / 10 % |
| 4K60, VA-API | 59,5 | 19,0 ms | 113 % de un núcleo | 1112 MiB | 48,0 W | 8 % / 8 % |

El VA-API de NVIDIA decodifica bien (NV12, a todos los fotogramas por segundo), pero no cambia nada
medible: sus fotogramas llegan igual en memoria del sistema y siguen la misma ruta de copia, con unos
50 MiB más de memoria y cerca de 1 GiB más de memoria de vídeo. El controlador sí sabe exportar NV12
como DMA-BUF (dos capas separadas, una por plano, con el modificador de bloques de NVIDIA
`0x0300000000606014`), pero no capas compuestas, y el plugin `va` de GStreamer 1.24 no consigue
exportar (`vaExportSurfaceHandle: an unsupported memory type was supplied`), así que no ofrece
DMA-BUF. Por eso el daemon no lo prefiere ni lo detecta: darle cero copias en NVIDIA exigiría
decodificar con NVDEC o `libva` directamente, o un GStreamer que acepte capas separadas.

La configuración vive en `~/.config/reiwand/config.toml` y solo la puede leer su usuario.
Si se edita a mano, los valores fuera de rango se corrigen al leerla. Si tiene un error de
sintaxis, el daemon arranca con los valores por defecto y muestra el error en `reiwand-ctl state`
(`config_error`); el archivo roto se copia a `config.toml.broken` antes de guardar otro. Al
guardar un cambio (desde la app, la bandeja, `reiwand-ctl config` o el teléfono) el archivo conserva
sus comentarios, sus líneas en blanco, el orden de las claves (también dentro de cada proveedor) y
cómo están escritos los valores: solo se reescribe lo que cambió.

Para pruebas, `reiwand-ctl stream <salida> <ancho> <alto>` crea un flujo que sigue vivo al salir
la orden y `reiwand-ctl release <salida>` lo destruye. Con `--wait`, el flujo dura lo que dure la
orden, igual que el de un escritorio.

La biblioteca, las listas, los perfiles, los proveedores y los paquetes se manejan igual
(`reiwand-ctl <grupo> --help` muestra cada orden). Las órdenes que cambian algo (`set`, `mute`,
`covered`, `input grant`, `lib rate`, `playlist rm`…) confirman con una línea lo que hicieron. Los
mensajes de error, los avisos de `scene check` y la salida de las órdenes salen en el idioma del
sistema; `scripts/translations.sh check` falla si en `apps/reiwand-ctl` aparece un texto fijo en
inglés sin pasar por la traducción. La CLI apaga el ruido de la biblioteca gráfica (`libEGL
warning`) con `EGL_LOG_LEVEL=fatal` salvo que la variable ya esté definida.

```sh
reiwand-ctl lib add ~/Imágenes --wait              # añadir una carpeta y esperar al escaneo
reiwand-ctl lib search 'tag:mar sort:random'       # buscar; los elementos se nombran por su uid
reiwand-ctl lib search 'provider:yes kind:image'   # solo las fotos que trajeron las fuentes de fotos
reiwand-ctl lib apply 1R1AB2 -o DP-1               # mostrar un elemento (uid o sus últimos 6+)
reiwand-ctl lib sources                            # las carpetas; `lib scan` y `lib unsource` aceptan el id corto que muestra o la ruta
reiwand-ctl playlist new Mar --query tag:mar       # lista inteligente
reiwand-ctl playlist start Mar && reiwand-ctl next    # rotar una lista y avanzar
reiwand-ctl profile save Escritorio                # guardar lo que muestra cada pantalla
reiwand-ctl provider add diario bing               # una fuente en línea
reiwand-ctl provider wallhaven add arte --ratios 16x9,landscape --colors 660000 --sorting random --no-secret
reiwand-ctl remote add 'https://example.org/r/fondos'   # un servidor de casa con su certificado: --trust-certificate HUELLA
reiwand-ctl account sign-in 'https://example.org/r/privado'   # cuentas por repositorio: sign-in, ls, use, unlink, rm
reiwand-ctl export config.reiwand && reiwand-ctl import config.reiwand --list   # llevar la configuración a otro equipo en un archivo
reiwand-ctl pkg install org.example.dawn --wait    # instalar un paquete de un repositorio
reiwand-ctl pkg import fondo.wpak otro.wpak --wait  # importar paquetes locales; --allow-older acepta una versión más antigua
reiwand-ctl jobs | watch | schema                  # trabajos, cambios en vivo, claves válidas
reiwand-ctl input decide org.example.dawn -a audio,pointer -x network   # una respuesta para todo lo que pide la escena
reiwand-ctl input grant org.example.dawn audio     # dejar que la escena que se ve siga el sonido (-o elige pantalla)
reiwand-ctl input status                           # permisos y si se captura el sonido o el puntero
reiwand-ctl input clicks on -o DP-1                # dejar que los clics de esa pantalla lleguen a su fondo
```

## Recorrer fondos con el teclado

La tira que aparece sobre el fondo al recorrerlos está en [Cambiar de fondo con el teclado](cambiar-con-el-teclado.md).

`reiwand-ctl next --preview` y `reiwand-ctl prev --preview` muestran cuál sería el fondo siguiente o el
anterior sin cambiar nada: empiezan a recorrer la lista que muestra esa pantalla (o la biblioteca,
con `--from library` o si la pantalla no tiene lista) y cada llamada avanza o retrocede uno.
`reiwand-ctl browse apply` deja puesto el que estás mirando (con `--transition fade` o `none` solo
para ese cambio), `reiwand-ctl browse cancel` deja todo como estaba y `reiwand-ctl browse show` dice cuál
es. Si no tocas nada durante 30 segundos se deja de recorrer solo.

Con `reiwand-ctl config shortcuts.enabled true` el programa pide al escritorio dos atajos globales
(propone `LOGO+ALT+N` y `LOGO+ALT+P`, que se cambian con `shortcuts.next` y `shortcuts.previous`):
en Plasma, GNOME y Hyprland el escritorio los registra y puede mostrarte su propio diálogo para
elegir otras teclas. En Sway y los demás compositores wlroots, y en X11, no hay portal de atajos:
ata las órdenes a teclas en tu compositor, por ejemplo en Sway

```sh
bindsym $mod+Alt+n exec reiwand-ctl next --preview
bindsym $mod+Alt+p exec reiwand-ctl prev --preview
bindsym $mod+Alt+Return exec reiwand-ctl browse apply
bindsym $mod+Alt+Escape exec reiwand-ctl browse cancel
```

y en X11 lo mismo con el gestor de atajos de tu entorno (o `xbindkeys`).

El atajo, la bandeja y `reiwand-ctl next --preview` dibujan sobre el fondo una tira con el actual, los
anteriores y los siguientes (los atajos y la rueda recorren, el clic en el central aplica); la guía
`docs/usar/cambiar-con-el-teclado.md` la explica con sus límites.

El proveedor Wallhaven usa la API oficial v1 y admite todas las opciones de su búsqueda: palabras
(`query`), categorías y pureza, orden (`date_added`, `relevance`, `random`, `views`, `favorites`,
`toplist`, `hot`), sentido, rango del `toplist` (`1d` a `1y`), resolución mínima y resoluciones
exactas, proporciones (`16x9`, `16x10`, `landscape`, `portrait`…), colores de su paleta, «parecidos
a» un fondo (`similar`), una etiqueta (`tag`), un usuario (`uploader`) y colecciones (`collection =
usuario/id`, públicas o, con tu clave de API, privadas), además de la página inicial, cuántas
páginas se leen por sincronización (1 a 10) y el máximo de imágenes (hasta 240). Cada opción es una
opción de `reiwand-ctl provider wallhaven add|set` (`--help` las explica; `set` cambia varias a la vez y
comprueba el conjunto), un campo de `Providers1` y un control del diálogo «Opciones de búsqueda» de
la app: casillas para categorías y pureza, botones para proporciones y resoluciones, una paleta de
colores y, para las colecciones, un selector que las lista con `reiwand-ctl provider wallhaven
collections` (con clave de API incluye las tuyas). Con orden `random` el daemon elige una semilla nueva en cada
sincronización y la usa en todas sus páginas para que no repitan imágenes; `reiwand-ctl provider wallhaven
reseed` (o «Nuevo orden aleatorio» en la app) sincroniza al momento. En la consulta también se puede
pegar una dirección de búsqueda de wallhaven.cc: sus parámetros pasan a las opciones. Una sincronización pide solo las páginas
configuradas (una por defecto), nunca pasa de las 45 peticiones por minuto de la API y espera y
reintenta cuando responde 429. `sketchy` y `nsfw` siguen apagados por defecto y piden la clave de
API, que solo va en la cabecera `X-API-Key` y se guarda en el llavero. En el teléfono no hay ajustes
de proveedores.

Los repositorios aguantan los fallos. Las búsquedas leen todos los repositorios y los paquetes
instalados de una vez, así que nunca mezclan un catálogo con otra lista de instalados; dos
actualizaciones del mismo repositorio desde procesos distintos nunca lo devuelven a un estado
anterior, y una instalación comprueba otra vez, al terminar la descarga y antes de pasar a la
biblioteca, que el repositorio sigue ahí y que ni la versión ni su clave se revocaron. Si la copia
guardada de un repositorio se daña (falta su `state.json`, por ejemplo), `reiwand-ctl remote ls` y la
app lo muestran como roto sin afectar a los demás: `reiwand-ctl remote refresh` lo descarga otra vez
de su dirección y, si esta se perdió, se repara añadiéndolo de nuevo con su URL o se quita. Los
espejos `file://` de un repositorio firmado solo se usan cuando el propio repositorio se añadió
desde una carpeta o una URL `file://`. Cancelar una descarga la para en una décima de segundo
aunque el servidor no envíe nada (antes esperaba hasta 30 s), también en las copias desde
`file://`. Las vistas previas de las versiones anteriores se descargan una sola vez y se borran
con su repositorio. Un proveedor corta una descarga en cuanto no cabe en `cache_mb` (por su
`Content-Length` o al llegar a ese tamaño), su limpieza de la caché resuelve los enlaces simbólicos
y nunca borra fuera de su carpeta, y quitarlo sin `--purge` borra también su estado de
sincronización, así que al añadirlo de nuevo sincroniza enseguida. Si el llavero está bloqueado o
no responde cuando se añade un proveedor con el nombre de uno anterior, el secreto viejo nunca se
usa: `reiwand-ctl provider ls` y la app lo avisan, y se borra solo en cuanto el llavero se desbloquea
o vuelve.

La vista previa animada de un paquete (`previews.animated`) se mueve en los detalles de la app,
en la miniatura de cada pantalla de la app y de la bandeja, y en la página del teléfono de un
paquete que este no puede reproducir; solo mientras se ve, en el escritorio con el equipo
enchufado y, si el paquete es para adultos o fotosensible, después de permitirlo. Al explorar los
repositorios en la app también se mueve la vista previa animada del paquete que tiene el puntero o
el foco: se descarga entonces y solo con el equipo enchufado, dentro de su tamaño y su hash firmados
(nunca si pasa de los 8 MiB que admite un paquete), se guarda con las demás vistas previas y se
borra con su repositorio; nunca se mueve la de un paquete fotosensible ni la que sigue difuminada. `reiwand-ctl pkg preview --animated PAQUETE` da la misma
ruta.

La biblioteca de la app pide de nuevo solo los elementos que nombra cada cambio (con `uid:`, que
también acepta `reiwand-ctl lib search 'uid:<uid>,<uid>'` con uids completos), y al cambiar o repetir una búsqueda
conserva la selección y la posición de los elementos que siguen en ella.

Las consultas aceptan `--json` e imprimen la respuesta del daemon tal cual; sin él muestran
tablas para leer, no para scripts. Lo que lanza un trabajo imprime su número y termina; con
`--wait` muestra el progreso y el resultado, y Ctrl-C deja de esperar sin parar el trabajo
(`reiwand-ctl cancel <n>` lo para). Un trabajo que falla siempre dice por qué: si el error no trae
descripción, el daemon envía `failed without giving a reason` con el nombre del error, la app lo
muestra en tu idioma y el teléfono da también un motivo traducido; en una pregunta (como instalar desde un repositorio caducado),
Ctrl-C la deja sin respuesta y no cambia nada. Las contraseñas y claves de los proveedores se piden
sin eco o se leen de la primera línea de la entrada con `--secret-stdin`; nunca van en la orden.
`reiwand-ctl input grant` da el permiso al paquete que se ve con ese id; si varias pantallas muestran
paquetes distintos con el mismo id, `-o` elige la pantalla, y si el daemon pregunta en una
notificación (en el Flatpak, siempre), lo avisa y termina bien: la escena recibe la entrada cuando
la permites allí. Los clics y toques necesitan dos permisos: el del paquete (`reiwand-ctl input grant
<escena> clicks`) y el de la pantalla (`reiwand-ctl input clicks on|off [-o pantalla]`, sin `-o` todas),
que empieza apagado en todas; la app los ofrece en los ajustes de la escena de cada pantalla y la
bandeja en cada pantalla cuya escena los pide. Códigos de salida: 0 bien, 1 error, 2 uso incorrecto, también cuando es el
daemon quien rechaza un argumento (un ajuste o una regla que no existe, un valor fuera de rango).
`reiwand-ctl preset rm` acepta rutas con `:` (`~/fotos/a:b.png:Noche`).

La biblioteca vigila sus carpetas, también los enlaces duros y simbólicos y las carpetas movidas o
borradas, y un cambio durante el escaneo de una carpeta trae otro escaneo al terminar. El escaneo
lee los archivos (tamaño, huella rápida y manifiesto de los paquetes) fuera de la base de datos y
guarda los cambios en tandas cortas, sin escribir los archivos que no cambiaron, así que lo que
cambias en la app o la bandeja no espera a que termine de leer: al revisar 1500 archivos sueltos de
1 MiB que cambiaron, otra escritura esperó como mucho 18 ms en vez de 0,73 s, y si no cambió
ninguno la revisión ya no escribe nada (12 ms en vez de 51 ms). Una etiqueta que quitas de un
paquete no vuelve cuando el archivo cambia o llega otra versión (sí si la añades otra vez), y al
renombrar una etiqueta el nombre viejo tampoco vuelve. En una lista aleatoria, *Anterior* y después
*Siguiente* repiten exactamente lo que se mostró, en orden (se recuerdan los últimos 32 fondos),
también después de cambiar la lista o reiniciar el daemon. Un archivo
que el decodificador tarda demasiado en leer (15 s más 1 s por MiB, 2 min como mucho) o que lo hace
caer no queda
marcado como roto: se vuelve a intentar al arrancar el daemon o cuando cambia. Si la biblioteca no
puede guardar algo (disco lleno, base de datos bloqueada), el fondo aplicado se queda y la posición
de una lista se guarda en cuanto se pueda, sin avanzar otra vez. En una lista con intervalo y horas
fijas, un cambio de zona horaria recalcula las horas en la zona nueva y el intervalo sigue contando
desde el último cambio. Si el llavero no responde, `reiwand-ctl provider ls` espera como mucho 1 s y
muestra el proveedor como bloqueado hasta que el llavero conteste; durante el minuto siguiente,
nada vuelve a esperarlo. Una imagen se decodifica con límites de tamaño (32768 píxeles de lado y
1 GiB de memoria), los mismos en la biblioteca, el daemon, la app y `reiwand-ctl`, así que una foto
que tiene miniatura también se puede poner de fondo; una foto JPEG de hasta 300 megapíxeles se
decodifica directamente a un tamaño reducido, y una cabecera que promete un tamaño enorme se
rechaza al momento en vez de llenar la memoria.

Las ediciones no tocan la imagen: se guardan en `<imagen>.reiwand.json` a su lado (o en
`~/.local/share/reiwand/edits/` si la carpeta no se puede escribir), las aplica la GPU y el
daemon las vuelve a leer en cuanto cambian. La app las edita desde la ficha de cada imagen
(«Editar…»). A la izquierda están siempre a la vista las secciones, cada una con su icono
(*Recortar y girar*, *Pantallas*, *Luz y color*, *Curvas*, *Efectos*, *Máscaras*, *Profundidad* y
*Paralaje*; las flechas recorren la lista), y debajo las *Operaciones, en orden*; al lado, los
controles de la sección elegida, y a la derecha la imagen editada. La dibuja la propia app en la
GPU, con el mismo revelado que el daemon y `reiwand-ctl edit export`, reducida (1024 píxeles de lado
como mucho, 512 con render por software) y solo cuando cambia un valor o llega un mapa de
profundidad nuevo; si no puede dibujarla, un aviso dice por qué, y otro avisa si la imagen cambió
después de editarla (también lo dicen `reiwand-ctl edit` y el registro del daemon). En *Recortar y
girar* se ve la imagen entera con el recorte encima para moverlo y cambiar su tamaño, en *Máscaras*
las máscaras lineales y radiales se colocan arrastrando sus tiradores sobre la imagen y las de
pincel se pintan sobre ella (con su tamaño, su dureza y *Borrar*; cada trazo es un paso que se
puede deshacer), y en
*Pantallas* el recorte de cada pantalla sobre el resultado y las pantallas dibujadas a escala en
milímetros: se arrastran hasta donde están de verdad, con el ancho de sus marcos, y un patrón de
líneas y círculos sobre las pantallas reales permite comprobarlo (las líneas siguen rectas de una
pantalla a otra). *Deshacer* y *Rehacer* (Ctrl+Z y Ctrl+Mayúsculas+Z) recorren los mismos pasos que
`reiwand-ctl edit undo` y `redo`, *Comparar con el original* (tecla `\`) muestra la imagen sin
operaciones y *Exportar…* guarda el resultado en PNG, JPEG, WebP sin pérdida o AVIF. *Ver en el
escritorio* lleva además la edición sin guardar al fondo real, allí donde esa imagen sea el fondo,
y la ventana lo avisa bajo la imagen. Cada acción del editor existe también en `reiwand-ctl edit`, que
las cambia sin daemon:

```sh
reiwand-ctl edit set foto.jpg ev=0.3 contrast=10 temperature=5200 tint=4
reiwand-ctl edit crop foto.jpg 0.1,0.05,0.8,0.9 --rotate 90
reiwand-ctl edit crop foto.jpg --aspect 16:9 -o DP-1      # recorte propio de una pantalla
reiwand-ctl edit overlay foto.jpg '#ff8800' --blend soft-light --opacity 0.3 --mask linear:0.5,0,0.5,0.5
reiwand-ctl edit preview foto.jpg ev=1                  # verlo en el escritorio 60 s sin guardar
reiwand-ctl edit export foto.jpg -o foto-editada.png    # mismo resultado, sin daemon
reiwand-ctl edit export foto.jpg -o foto.avif --quality 70   # o .jpg y .webp; --before sin operaciones
reiwand-ctl edit undo foto.jpg                          # y redo, también tras cambios hechos en la app
reiwand-ctl edit set foto.jpg ops.bl.mask=cielo         # dar una máscara a una operación; vacío la quita
reiwand-ctl edit mask foto.jpg cara 'brush:0.04,0.6:0.41,0.33;0.43,0.35'   # un trazo de pincel; erase: borra
reiwand-ctl edit fit foto.jpg center                    # su propio ajuste; none vuelve al de la pantalla
reiwand-ctl set foto.jpg -f span && reiwand-ctl edit span foto.jpg --zoom 1.2
reiwand-ctl config layout.bezel_mm 8                    # ancho del marco de cada pantalla al extender
reiwand-ctl config layout.monitors.DP-1 0,0,597,336     # colocar una pantalla a mano, en milímetros
reiwand-ctl config layout.follow.STREAM-1 DP-1         # una salida sin ubicación muestra la parte de DP-1
reiwand-ctl calibrate on                                # patrón para comprobarlo; off lo quita
```

Una imagen fija puede moverse en profundidad (paralaje): con un mapa de profundidad en gris (blanco
cerca), el daemon desplaza más lo cercano que lo lejano siguiendo una órbita lenta, el puntero o el
acelerómetro del equipo, solo mientras el fondo se ve, al ritmo que permite la pantalla, con las
mismas pausas que el vídeo y quieto si el sistema pide reducir el movimiento. Elegir el puntero
como fuente para una imagen es tu permiso para seguirlo mientras se ve, igual que en las
escenas. Con render por software (sin GPU) la imagen queda quieta salvo que actives
`playback.software_parallax`, que la mueve a 15 fotogramas por segundo como mucho.
El mapa puede ser propio (`reiwand-ctl edit depth import`, con la misma proporción que la imagen) o
calcularse en el propio equipo con Depth Anything V2 Small (Apache-2.0) mediante ONNX Runtime, en
un proceso aparte y nunca dentro del daemon (solo las capas de más abajo se calculan en un trabajo suyo en segundo plano); gana el mapa más reciente y la caché de mapas
calculados no pasa de 256 MB. Ni el modelo (99 MB) ni ONNX Runtime (11 MB) vienen incluidos: solo
se descargan de GitHub cuando lo pides, después de mostrar tamaño, licencia y servidores, se
comprueban con su SHA-256 fijado, se guardan junto al texto completo de su licencia (y ONNX
Runtime junto a los avisos de terceros de su archivo) y la imagen no sale nunca del equipo.
`models.allow_download = false` impide cualquier descarga y `reiwand-ctl models install` instala
archivos obtenidos por otra vía. En la app, al terminar la descarga el mapa se calcula solo; se ve
ya ajustado (cerca, lejos, curva, suavizado y bordes), en gris o en falso color, y el modelo corre
en el procesador o, con un ONNX Runtime compilado con CUDA (por ejemplo indicado en
`ORT_DYLIB_PATH`), en la tarjeta gráfica NVIDIA; *El dispositivo más rápido* la usa cuando puede y
si falla vuelve al procesador:

```sh
reiwand-ctl models list                                 # tamaño, licencia, servidor y estado
reiwand-ctl models download depth-anything-v2-small     # pregunta antes de descargar
reiwand-ctl edit depth generate foto.jpg                # mapa calculado aquí; Ctrl-C cancela
reiwand-ctl edit depth generate foto.jpg --device cuda --bench   # medir el modelo en la tarjeta gráfica
reiwand-ctl edit depth export foto.jpg mapa.png --adjusted --colour   # el mapa ajustado en falso color
reiwand-ctl edit depth foto.jpg --dilate 4 --smooth 2   # ajustar cerca, lejos, curva y bordes
reiwand-ctl edit depth remove foto.jpg                  # borrar el mapa calculado, el importado y el de al lado
reiwand-ctl edit parallax foto.jpg --source pointer --strength 0.02 --focus 0.3
```

*Parallax 3D calculado una vez por imagen.* Una imagen solo se mueve en capas, sin estirar nada:
el daemon prepara una vez, en segundo plano (un trabajo `Layers` con su progreso, que se ve en
`reiwand-ctl jobs`) y nunca por fotograma, un juego de capas de la imagen. Afina el mapa con un filtro
guiado por la propia imagen, busca los saltos de profundidad y parte la imagen en regiones conexas
entre ellos (lo fino, como un mechón, sigue con su objeto; los pedazos chicos se unen al vecino con
el que más borde comparten) y reparte regiones enteras en 4 a 6 capas (`layers` de la edición de cada imagen, 5 por
defecto) según su profundidad media, de modo que una superficie continua nunca se corta. Dentro de
cada capa la profundidad se suaviza y su pendiente se limita para que nada se estire más de un 4 %
con la fuerza elegida: un suelo, un puente o una pared se mueven con su profundidad, y lo que no
cabe en ese límite se aplana hacia una tarjeta rígida. El borde de cada objeto cercano toma la
transparencia de la propia imagen, sin franjas del fondo, y detrás se marca la franja que el
movimiento puede descubrir, que se rellena sin copiar el borde del objeto según el `fill` de la edición de cada imagen:
`lama` (por defecto en el escritorio) usa el modelo LaMa en recortes de 512 px a la resolución de
la imagen, `simple` extiende por pirámide el fondo más cercano. Ambos se eligen por imagen en el
panel de paralaje (*Relleno*, *Capas de profundidad*) y en el teléfono. El juego se calcula a la resolución que pide la pantalla (1024, 1920, 2560 o
3840 px de lado mayor) y se guarda en la caché de mapas
(`~/.cache/reiwand/depth/<SHA-256>-<clave>.layers.png`, `.fields.png` y `.layers.json`, dentro
de los mismos 256 MB) con una clave que reúne la imagen, el modelo, el relleno, el número de capas,
la fuerza, la resolución y los ajustes del mapa. El movimiento se recorta por imagen para no pasar
nunca de la franja rellenada y la ampliación se calcula con la fuerza, el enfoque y la forma de la
imagen para que nunca se vea el borde del cuadro. Mientras se calculan las capas, si fallan o si su profundidad no permite
moverla sin estirarla, la imagen queda quieta, y la app, `reiwand-ctl edit parallax`, `reiwand-ctl edit depth
layers` y el teléfono dicen por qué en una línea; si falta el modelo de profundidad lo dicen al activar
el paralaje, con su tamaño y el botón o la orden que lo descarga (nunca se descarga solo). Al moverse, la GPU dibuja las capas de atrás adelante y en reposo
muestra la imagen original; un recorte, giro, enderezado o volteo gira también las capas. LaMa (big-lama en ONNX, 208 MB, Apache-2.0) no viene incluido:
se descarga con `reiwand-ctl models download lama-big-fp32` (o desde los ajustes de la app) después de
mostrar tamaño, licencia y servidor, con las mismas reglas que el modelo de profundidad; sin él, o
sin ONNX Runtime, se usa `simple`. Corre en el procesador, o en CUDA si `ORT_DYLIB_PATH` apunta a
un ONNX Runtime con CUDA, con prioridad mínima y como mucho cuatro hilos.

```sh
reiwand-ctl models download lama-big-fp32                          # el relleno LaMa (208 MB)
reiwand-ctl edit depth layers foto.jpg                             # calcular las capas con los ajustes
reiwand-ctl edit depth layers foto.jpg --fill simple --layers 4 --force
reiwand-ctl edit depth layers foto.jpg --list                      # los juegos de capas de la imagen
reiwand-ctl edit depth layers foto.jpg --clear                     # borrarlos
reiwand-ctl edit depth show foto.jpg --layer 0 --out capa.png      # inspeccionar una capa
reiwand-ctl edit depth show foto.jpg --layer 1 --out capa.png --band franja.png   # y su franja a rellenar
reiwand-ctl edit depth compare foto.jpg --out cmp.png --layer 1    # LaMa a la izquierda, relleno simple a la derecha
reiwand-ctl edit depth brush foto.jpg "0.3,0.4;0.5,0.4" --raise --by 0.2 --radius 0.05   # pincel de profundidad
reiwand-ctl edit depth cuts foto.jpg 0.2,0.45,0.7                  # cortes de capa a mano (--auto vuelve a los automáticos)
reiwand-ctl edit depth fill-mask foto.jpg "0.5,0.5" --protect      # zonas que siempre (--refill) o nunca (--protect) se rellenan
reiwand-ctl edit parallax foto.jpg --fill simple --layers 6        # relleno (lama o simple) y capas de esta imagen
```

*Herramientas de autor para las capas.* En la página de edición de una imagen con mapa de profundidad,
la herramienta *Profundidad* muestra la imagen entera y sin girar y trae, bajo los ajustes del mapa, las
herramientas con las que el autor decide cómo se parte y se rellena la imagen, cada una con su orden en
`reiwand-ctl edit depth` y con deshacer:

- *Profundidad sobre la imagen*: muestra el mapa ya ajustado, con los trazos, encima de la imagen, en gris
  o en falso color y con un control de opacidad.
- *Pincel de profundidad*: sube o baja la profundidad con un radio, una fuerza y un borde suave a elegir;
  cada trazo es un paso del historial y se aplica al mapa ajustado (`edit depth brush`).
- *Cortes de capa*: sobre el histograma de profundidad se ven los cortes como asas que se arrastran
  (un doble clic quita una, un clic en la franja añade otra) mientras la imagen muestra el contorno
  teñido de cada capa en vivo; al soltar se vuelven a calcular las capas con esos cortes, que el
  agrupamiento automático ya no pisa hasta pulsar *Volver a automático* (`edit depth cuts`).
- *Pincel de relleno*: pinta en la imagen lo que siempre debe rellenarse detrás de un objeto cercano
  (aunque quede fuera del alcance del desplazamiento) o lo que nunca se rellena, y las capas se
  recalculan al soltar (`edit depth fill-mask`).
- *Capas y relleno*: elige una capa para verla sola, con la franja que el movimiento puede descubrir
  resaltada (`edit depth show --band`), o compara el relleno de LaMa a un lado de un divisor que se
  arrastra y el simple al otro (`edit depth compare`); lo que falta se calcula entonces, con su
  progreso, y la comparación solo se ofrece con LaMa instalado.

Las capas se recalculan solas medio segundo después del último cambio mientras la herramienta está
abierta, y un cambio nuevo cancela el cálculo en curso. En *Ajustes*, junto a la descarga de LaMa, hay
botones para comprobarlo (`models verify`) y quitarlo (`models remove`), y *Recalcular las capas*, que
borra los juegos de capas guardados y hace que el daemon prepare de nuevo los de las imágenes que
muestra; el botón indica cuántos juegos hay guardados o cuánto lleva el cálculo del daemon.

`reiwand-ctl pack` (crear, firmar, comprobar, actualizar y validar paquetes `.wpak`) y `reiwand-ctl repo`
(claves, firma, publicación, rotación y verificación de un repositorio) funcionan sin daemon. La
contraseña de una clave se pide en la terminal o se lee de `REIWAND_KEY_PASSWORD`.

La ayuda de `reiwand-ctl` (`--help` y la orden `help` de cada grupo, cada argumento con su valor
predeterminado y sus valores posibles, los títulos *Uso*, *Argumentos*, *Opciones* y *Órdenes*) y
los avisos con que rechaza una orden mal escrita (argumento inesperado, valor no válido, argumento
obligatorio que falta, consejos) salen en el idioma del escritorio con el catálogo
`po/<idioma>/reiwand-ctl.po`; los nombres de las órdenes, de las opciones y de sus valores quedan en
inglés para que los scripts sigan funcionando igual en cualquier idioma.

En *Ajustes* del teléfono, *Cuenta* lista las cuentas de los servidores con los repositorios donde se usa cada
una (`reiwand-ctl account ls`): se puede dejar de usar una en un repositorio o quitarla. Una cuenta se
inicia al agregar un repositorio privado o al publicar, con un código que se aprueba en el navegador; su
acceso se guarda cifrado con una clave del almacén de claves de Android que no sale del teléfono y solo
se envía al servidor de su repositorio. No hay sincronización por la red: lo que cada equipo tiene es
local, y para llevar la configuración de uno a otro (repositorios, proveedores, listas, favoritos,
valoraciones, etiquetas y ajustes) se exporta a un archivo y se importa en el otro, del teléfono al
ordenador o al revés (`reiwand-ctl export` e `import` en el ordenador; *Exportar la configuración* e
*Importar una configuración* en *Cuenta*, con el selector de archivos). Las claves de los proveedores solo salen si
lo marcas, con el archivo cifrado. La página
*Fuentes* reúne de dónde vienen los fondos, con o sin servidor propio, y es lo que ofrece el botón
*Conseguir fondos de internet* de la biblioteca vacía y la opción *Desde internet* de *Añadir*. Arriba están las
*Imágenes de internet*, los proveedores en el teléfono (imagen del día de Bing, imagen astronómica de la NASA,
imagen del día de Wikimedia, búsqueda en Wallhaven y carpetas de un servidor WebDAV, Nextcloud o Immich): cada
uno nace apagado, dice con quién se conecta y qué ve esa parte, y no toca la red hasta que lo enciendes; apagado, ni
la descarga ahora ni la automática hacen nada; si cambia la dirección a la que apunta, vuelve a apagarse y olvida su
clave. Descarga solo imágenes, cada seis horas como mucho y con las reglas de red y batería de los ajustes
(*Usar también datos móviles en segundo plano*; *Descargar ahora* lo pide la persona y va sin esa espera), guarda
cada una en la biblioteca con la etiqueta `from-ID`, conserva las últimas *N* (no quita favoritas, de listas ni la
puesta) y muestra los créditos. Por HTTPS, el teléfono (fuentes, repositorios y servidor propio) confía en las
raíces públicas que trae el programa y en las autoridades de certificación que instalaste en los ajustes de Android,
que lee al abrir la app y vuelve a mirar en cada conexión nueva; si un servidor presenta un certificado que no pasa
esa comprobación, la fuente lo dice debajo de su nombre en vez de quedarse sin descargar. Debajo están los *Repositorios*: se añade uno pegando su dirección en un solo campo (si es privado pide una cuenta; si su servidor usa un certificado propio, muestra su huella para que la confirmes), y
al añadirlo se abre *Explorar*, que busca sus paquetes (los marca si son para adultos y los oculta con *Ocultar el
contenido para adultos*), enseña la vista previa de cada uno (una por vez que se ve la fila, solo del
repositorio que añadiste, con un hueco gris mientras llega, sin pedir las de adultos salvo con
*Mostrar sin difuminar*, y guardadas hasta 16 MiB, que se borran con el repositorio) y los instala en la biblioteca, comprobados con el índice firmado
del repositorio; un paquete instalado así es el mismo elemento de la biblioteca en el teléfono y
en el ordenador. Una vez al día, con las reglas de red y batería de los ajustes (*Usar también datos móviles en
segundo plano*), solo si hay repositorios y mientras *Buscar actualizaciones en los repositorios cada día*
(`repos.auto_refresh`, activado de serie como en el escritorio, en los ajustes del teléfono) siga
activado, el teléfono actualiza sus índices como el escritorio: la
pestaña muestra un aviso con cuántos paquetes tienen versión nueva, *Actualizaciones* los lista con
*Actualizar* (la versión nueva ocupa el lugar de la vieja, con sus listas y ajustes) y *Buscar
actualizaciones* lo hace al momento. Con *Actualizar los repositorios automáticamente*
(`repos.auto_update`, desactivado de serie porque actualizar sin preguntar cambia lo que se ejecuta;
en los ajustes del teléfono, bajo los fondos; en el ordenador, en los ajustes junto a los
repositorios, y `reiwand-ctl config repos.auto_update true`) los paquetes con versión nueva se
actualizan solos tras esa comprobación, con las mismas reglas de red y batería y todas las
comprobaciones de una instalación; una versión que pide un permiso nuevo no se instala sola:
queda en la lista con una nota que dice qué pide ahora. Nunca hay notificación de actualizaciones.

Para usar un repositorio de la red de casa sin un dominio ni un certificado público, un ordenador puede
servir `reiwand-server` con su propio certificado (`REIWAND_TLS=self-signed`, ver
[Servidor propio](servidor.md)). Ese servidor no se acepta a ciegas: al agregar su repositorio la app
muestra la huella SHA-256 de su certificado para compararla con la que da `reiwand-server fingerprint`,
y solo si eliges *Confiar en él* la fija; desde entonces el teléfono solo habla con ese servidor si
presenta ese mismo certificado, y la pestaña lo dice.
