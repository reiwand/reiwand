# Guía para autores de scripts

Un script es un módulo JavaScript del paquete que la escena nombra en `scripts` (como mucho 64 y 1 MiB
entre todos). En el ordenador todos los de una escena comparten un proceso aislado, y en el teléfono
un hilo, también cuando varias pantallas
muestran la misma escena con los mismos ajustes, con 32 MiB de memoria y 4 ms de CPU por fotograma
para todos juntos (100 ms para cargar y para el primer fotograma). Lo que devuelven llega al
fotograma siguiente, así que un script lento nunca frena el dibujo, y solo viaja lo que cambió:
cada campo que un script escribe se compara con el que ya tenía, sin pasar la salida entera a texto
en cada fotograma, y un fotograma sin cambios no envía nada. `docs/specs/scene.md` describe la
interfaz completa; lo esencial:

- **Ciclo de vida**: el módulo exporta las funciones que necesita: `init(ctx)` en el primer
  fotograma, `update(frame)` en cada uno, `destroy()` al parar, `resize({screen, canvas})` al cambiar
  o girar la pantalla, `properties(changed)` con los ajustes que cambian (todos al
  empezar), `settings({language, fps, quality})` y `timelineEvent(e)`. Sin `update` el script solo
  despierta por eventos y la escena no dibuja fotogramas de más.
- **Un valor, una función**: `{"script": "scripts/olas.js", "fn": "vaiven", "params": {"velocidad":
  2}}` llama cada fotograma a `vaiven(valor, ctx)` con lo que devolvió la vez anterior y `ctx.path`,
  `ctx.layer` y `ctx.params`; un mismo módulo sirve así a muchas propiedades. Los `params` se declaran
  solo en la entrada del script en `scene.json`, con tipo, valor por defecto y límites: el módulo no
  los exporta y `reiwand-ctl scene act CARPETA import_script` no los propone, pero los conserva al importar de nuevo
  el módulo. En el editor se escriben en una tabla tipada por módulo (id, tipo, valor por defecto y,
  en los números, mínimo y máximo) y cada propiedad los sigue con *Seguir un script*, que lista las
  funciones del módulo.
- **Capas**: `scene.layer(id)` lee la posición, la escala, el giro, la opacidad, el color, el tamaño,
  el texto y lo demás tal como la escena los evaluó, y escribe esos campos, que duran hasta que el
  script los cambie o llame a `reset()`. `layer.effect("nombre")` apaga un efecto, cambia sus
  parámetros dentro de su rango o vacía lo que guarda. Con plantillas (`templates`),
  `scene.spawn("barra", {position: [x, y]})` crea hasta 256 copias, que con las capas de la escena y
  las de dentro de cada copia suman como mucho 512 (una plantilla tiene como mucho 256), `destroy()`
  las quita y `scene.order(id, índice)` las reordena: así se hacen los visualizadores. Una plantilla
  puede ser también una capa `mesh` (o un grupo con ellas): cada copia recibe su propio títere, con
  huesos, clips, física, IK y adjuntos como las mallas de la escena (64 títeres como mucho en total);
  un `attach`, un `parent`, un objetivo `layer:<id>`, una máscara de capa (de la capa o de un
  efecto) o un parámetro de efecto que toma una capa, de dentro de la plantilla y que nombra otra capa
  de ella, apunta en cada copia a la capa de esa misma copia, los toques en los huesos de una malla
  copiada nombran el hueso de esa copia; los sistemas de partículas son de toda la escena, pero
  `particles(capa).from(n, "copia.capa")` y `.wall(n, …)` hacen que una capa de partículas nazca de
  la capa de una copia o choque con ella, y el botón
  *Hacer plantilla* del editor copia la capa elegida a `templates` con un nombre libre
  (`<id>_template`, luego `<id>_template_2`…; `reiwand-ctl scene act CARPETA duplicate_layer`
  hace lo mismo) y la barra de capas muestra una plantilla sola en el lienzo, como se dibuja una copia;
  mientras se ve, el panel de capas, las manijas del lienzo (mover, huesos, cadenas, pesos, puntos y
  enganches) y cada acción de capas trabajan sobre las capas de esa plantilla con sus propios ids
  (`reiwand-ctl scene --inside PLANTILLA …` igual), y como una plantilla tiene una sola capa las nuevas van
  dentro de su grupo. Un enganche, un objetivo de cadena o una máscara de una capa de la plantilla
  pueden nombrar también una capa de la escena (el inspector ofrece *Seguir una malla de la escena*,
  con su hueso o punto), y lo que no trata de capas (propiedades, condiciones, ajustes, recursos,
  lienzo, partículas…) actúa sobre toda la escena aunque se vea una plantilla, así que renombrar o
  quitar una propiedad ve y cambia también los enlaces de la escena.
- **Datos de entrada**: `audio.buffers(32)` devuelve las bandas izquierda, derecha y media en
  `Float32Array` que el motor rellena solas en cada fotograma; `input.cursor` da la posición en
  fracciones, en píxeles del lienzo y de la pantalla y el botón izquierdo; `engine` da el tamaño del
  lienzo y de la pantalla, la orientación, el idioma, la calidad, los fotogramas por segundo y si es
  fondo, pantalla de bloqueo o editor. El sonido y el puntero siguen necesitando su permiso.
- **Puntero y clics**: una capa con `"solid": true` (o `{"alpha": 0.5}` para contar solo donde es
  bastante opaca) recibe el puntero. Sin scripts, una línea de tiempo con `"trigger": "click:boton"`,
  `"enter:boton"` o `"leave:boton"` arranca al hacer clic, entrar o salir, y un sonido con ese
  `play_on` suena. Con scripts, `scene.layer("boton").on("click", (e) => …)` recibe `{type, layer,
  local, scene}` para `click`, `down`, `up`, `enter`, `leave` y `move`; la capa que recibe `down` se
  queda el puntero hasta soltar, aunque salga de ella, así que arrastrar es seguir `move` entre `down`
  y `up`, y un clic solo cuenta si se suelta sobre la misma capa. Se declara en `events` y la escena
  pide las entradas `pointer` y `clicks`: el usuario las permite por paquete y, los clics, también
  por pantalla, donde empiezan apagados. En el teléfono un toque es un clic.
- **Padres, cámara y ajustes de escena**: con `uses: ["hierarchy"]`, `layer.parent`,
  `layer.children` y `layer.world` (una `Mat3`) dicen a quién sigue una capa, quién la sigue y dónde
  queda en el lienzo, y `layer.setParent("sol", true)` la hace seguir a otra capa sin moverla ni
  cambiar el orden de dibujo; `lookAt(desde, hacia)` de `reiwand/math` da el ángulo para mirar hacia un
  punto. Con `uses: ["camera"]`, `scene.camera = {zoom: 2, shake: {amplitude: 6}}`,
  `scene.parallax`, `scene.bloom` y `scene.background` cambian la cámara, el paralaje, el resplandor
  y el color de fondo de la escena, y `null` los devuelve a lo que dice `scene.json`. El resplandor
  también se enciende en escenas que no lo declaran (con los valores por defecto de `canvas.bloom`
  para lo que el script no nombra) y se apaga con `null`; sigue las mismas reglas de calidad y batería
  y no cuesta nada hasta que un script lo pide; el script lo declara con `bloom` en `uses` (el editor
  lo añade solo al ver `.bloom` en el código) y el costo de la escena, el de cada nivel en el editor
  y `reiwand-ctl pack check` lo cuentan desde el principio.
- **Formas propias**: con `uses: ["geometry"]` y un `pools` en `scene.json`,
  `scene.mesh("barras").create({vertices, indices, format: "xyuv"})` crea triángulos con
  `Float32Array` que `update()` cambia y `destroy()` quita, dentro de los límites del pool y de 1 MiB
  por fotograma. Se dibujan en la imagen del pool, que una capa `video` muestra con `"video":
  "barras"`; un pool que no cambia no pide fotogramas.
- **Guardar datos**: con `uses: ["storage"]`, `storage.set("visitas", n)` y `storage.get` guardan
  JSON del paquete o, con `"output"`, de la pantalla: como mucho 64 KiB cada uno, solo en el
  dispositivo (sin sincronizar), en un archivo privado que se escribe como mucho cada 5 s y al parar
  y se borra al quitar o desinstalar el paquete. Las pantallas que muestran el paquete comparten lo
  del paquete (lo que guarda una llega a las demás en su siguiente fotograma) y las vistas previas
  del editor lo leen sin escribirlo.
- **Carpeta elegida**: con `uses: ["folders"]` y una propiedad `folder` con `"mode": "all"`,
  `folder("fotos").files` da identificadores opacos de sus imágenes, `name(id)` el nombre del
  archivo, `on("added" | "removed" | "changed", …)` avisa de los cambios y `show("marco", id)` elige
  qué foto muestra una capa. Con `folder("fotos", {videos: true})` también lista sus vídeos si la
  propiedad acepta `video` (`kind(id)` dice `image` o `video`) y `show("pantalla", id)` reproduce uno
  en una capa `video` con su sonido, con un reloj propio que sigue el `playback` de la capa y que
  `layer.video` controla (reproducir, pausar, saltar, velocidad, volumen, fin) como los vídeos del paquete,
  hasta ocho archivos a la vez, vídeos o fotos, en el escritorio y en el teléfono. En el escritorio,
  mientras la escena se ve, la carpeta se vigila con avisos de cambios del sistema y solo se vuelve a
  leer cuando los avisos se calman (medio segundo tras el último, 5 s como mucho tras el primero, así
  una copia de muchos archivos se lee una vez), sin contar los archivos ocultos que escriben las
  herramientas de sincronización, fuera del hilo que dibuja; oculta, deja de vigilarla y la mira una
  vez al volver. Si la carpeta falta, se borra o se mueve, la espera en la carpeta de arriba que exista
  y la vuelve a vigilar cuando aparece, y deja fuera las subcarpetas que nadie puede leer. Solo en
  carpetas de red (también las de espacio de usuario que van a la red, como `sshfs` o `rclone`, no un
  disco local ni una bóveda) o con más de 8 192 subcarpetas que vigilar la mira cada 2 s mientras se ve. El teléfono vuelve a listar su copia de la
  carpeta solo cuando la copia cambia, y la rehace cuando el sistema avisa que la carpeta cambió. Los
  scripts nunca ven rutas.
- **Sonido**: con `uses: ["sounds"]`, `sound("campana")` controla una capa de sonido con `play()`,
  `pause()`, `stop()`, `playing` y `volume`, de 0 a 1 y por debajo del volumen general, que también
  la silencia. La primera llamada se queda el sonido aunque tuviera `play_on`. El teléfono no
  reproduce sonidos de escena, así que allí esas llamadas no hacen nada.
- **Tiempo y depuración**: `setTimeout` y `setInterval` siguen el reloj de la escena (como mucho 64,
  una vez por fotograma y nunca por debajo de 16 ms) y devuelven una función para cancelarlos;
  `console.log` y sus hermanas escriben en el registro y en el editor, como mucho 20 líneas por
  segundo. `shared` guarda valores comunes a todos los módulos (una copia congelada de lo que se
  asigna: para cambiarlo se asigna otra vez), que también se importan entre sí con rutas relativas
  (`import { media } from "./lib/media.js"`).
- **Matemáticas**: `reiwand/math` trae `Vec2`, `Vec3`, `Vec4`, `Mat3` y `Mat4` con sus operaciones
  (producto, inversa, descomposición, reflejo, proyección…), `smoothstep`, `mix`, `remap`, ángulos,
  `lookAt`, `rgbToHsv` y `hsvToRgb`.
- **Declarar lo que se usa**: cada entrada de `scripts` lista sus `events` y sus `uses`; un módulo
  que declara algo solo puede usar los temporizadores, las capas creadas y los efectos si los
  declara, y `pack build` deduce de ahí las funciones nuevas que pide el paquete, para que un motor
  más viejo avise en vez de ejecutarlo mal. Un módulo que no declara nada recibe la misma
  interfaz y usa lo que importa de `reiwand`; el editor lista lo que usa sin declararlo.
- **Errores**: `reiwand-ctl pack check` y el editor compilan cada script sin ejecutarlo y
  dicen, con el archivo y la línea, un error de sintaxis o un script que falta; `reiwand-ctl scene render
  --time-of-day HORA` ejecuta los scripts en su proceso aislado y escribe su consola (con `--media pista.json`, ven una pista inventada del reproductor). Una función que
  lanza un error lo escribe en la consola y el resto sigue; un módulo
  que falla tres fotogramas seguidos se detiene solo, con sus temporizadores, y la escena sigue sin
  él con un aviso traducido en la app, la bandeja y `reiwand-ctl status`. Si los scripts agotan su
  tiempo o su memoria tres fotogramas seguidos, se apagan y la escena sigue sin ellos.

```js
import { scene, audio, sound, console } from "reiwand";
import { smoothstep } from "reiwand/math";

const bandas = audio.buffers(32);
const barras = [];

export function init() {
  for (let i = 0; i < 32; i++) barras.push(scene.spawn("barra", { position: [40 + i * 60, 900] }));
  scene.layer("tambor").on("click", () => sound("golpe").play());
  console.log("barras listas");
}

export function update() {
  barras.forEach((b, i) => (b.scale = [1, 0.1 + smoothstep(0, 1, bandas.average[i]) * 4]));
}
```
