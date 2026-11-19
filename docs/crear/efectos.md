# Los efectos

Un efecto cambia cómo se ve una capa: la desenfoca, la tiñe, la hace ondular. Esta página explica
cómo se ponen los efectos, qué efectos trae el programa, cómo se limitan con una máscara y cómo
escribir uno propio.

Los ejemplos salen de [`ejemplos/efectos-apilados`](ejemplos/efectos-apilados/scene.json) y de
[`ejemplos/efecto-propio`](ejemplos/efecto-propio/scene.json), y pasan `reiwand-ctl pack check`.

## Poner efectos en una capa

Cada capa tiene una lista `effects` de hasta 16 efectos. Se aplican en orden: el primero actúa
sobre la capa tal cual, el segundo sobre el resultado del primero, y así.

```json
    { "blend_space": "canvas", "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [
        { "effect": "color_adjust", "name": "color", "enabled": { "prop": "color_vivo" },
          "params": { "saturation": 1.8, "contrast": 1.15 } },
        { "effect": "waves", "params": { "amplitude": 10, "frequency": 5, "speed": 1.5 },
          "mask": { "texture": "mancha", "channel": "r", "feather": 0 } },
        { "effect": "blur", "params": { "radius": 6 },
          "mask": { "shape": { "rect": { "size": [1920, 260] } }, "invert": true, "strength": 0.8 },
          "optional": true }
      ] }
```

Cada elemento de la lista es un uso del efecto:

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `effect` | obligatorio | El id del efecto, de los de abajo o uno del paquete. |
| `params` | los de cada efecto | Los parámetros que cambias. Cada uno puede ser un valor fijo o seguir una propiedad, el tiempo, el sonido o el puntero. |
| `enabled` | `true` | `false` lo apaga sin quitarlo. Puede seguir una propiedad. |
| `mask` | sin máscara | Limita el efecto a una parte de la capa. Ver "Máscaras". |
| `optional` | `false` | `true` deja fuera el efecto, y no la escena, donde no pueda correr. |
| `name` | | Un nombre propio de este uso, con el que lo llaman los scripts y los enlaces. |

Los parámetros que no pones valen lo que dice su descriptor. En los parámetros de tipo imagen
(`texture`) escribes el id de un recurso de imagen, y en los de tipo tabla (`lut`) el de un
recurso `.cube`. Una imagen también puede ser `{ "layer": "id" }`, una capa dibujada aparte (ver
[escena.md](escena.md)).

Los puntos como `center` o `source` son fracciones de la capa (del lienzo, en una capa de ajuste).
Los que tienen un interruptor `follow` pasan al puntero mientras está en la pantalla y vuelven
suavemente a su sitio en un cuarto de segundo cuando se va.

Para ponerlos desde la terminal:

```
reiwand-ctl scene act MI-CARPETA effects '{"add":{"layer":"fondo","effect":"waves","params":{"amplitude":6,"frequency":3}}}'
reiwand-ctl scene act MI-CARPETA effects '{"set":{"layer":"fondo","index":0,"params":{"amplitude":10}}}'
reiwand-ctl scene act MI-CARPETA effects '{"move":{"layer":"fondo","index":0,"to":1}}'
reiwand-ctl scene act MI-CARPETA effects '{"remove":{"layer":"fondo","index":0}}'
```

(`set`, `move` y `remove` toman el lugar del efecto en la lista, empezando por 0.)
En el editor se hace desde la lista de efectos de la capa, con *Añadir un efecto*.

### Efectos sobre todo lo de abajo

Una capa de ajuste (`adjustment`) aplica sus efectos a todo lo que hay debajo de ella. Es la forma
de dar un acabado a toda la escena, como el resplandor y el grano del ejemplo:

```json
    { "blend_space": "canvas", "id": "pantalla", "type": "adjustment",
      "effects": [
        { "effect": "bloom", "params": { "threshold": 0.7, "intensity": 0.8, "radius": 24 } },
        { "effect": "vignette", "params": { "intensity": 0.45, "color": "#000010" } },
        { "effect": "film_grain", "params": { "amount": 0.04, "fps": 12 } }
      ] }
```

El lienzo también tiene un resplandor propio, `canvas.bloom`, que hace lo mismo que un `bloom`
arriba del todo (ver [escena.md](escena.md)).

### Calidad y costo

El nivel de calidad baja con la batería, el calor o la potencia del equipo. Entonces los efectos
cambian así:

- Los efectos **ligeros** corren en todos los niveles.
- Los **medios** se quitan en `q0`, y en `q1` si los marcaste `optional`.
- Los **pesados** corren sus pasadas interiores a un cuarto de tamaño en `q1`.
- La pasada que escribe el resultado siempre va a tamaño completo.

El catálogo dice qué efectos son ligeros, medios o pesados. Un efecto que no corre en el nivel
actual no pide su entrada, y tampoco uno cuyo `enabled` está apagado por una propiedad. Lo que
cuesta cada uno y cuánto cuesta la escena entera se explica en [escena.md](escena.md).

Los desenfoques y las distorsiones tratan los bordes de la pantalla como los bordes de la imagen:
una capa que llega más allá de la pantalla conserva el color de sus píxeles más exteriores.

## Máscaras

Una máscara limita un efecto a una parte de la capa, y puede ser una imagen, otra capa o una forma:

```json
    { "blend_space": "canvas", "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [
        { "effect": "color_adjust", "name": "color", "enabled": { "prop": "color_vivo" },
          "params": { "saturation": 1.8, "contrast": 1.15 } },
        { "effect": "waves", "params": { "amplitude": 10, "frequency": 5, "speed": 1.5 },
          "mask": { "texture": "mancha", "channel": "r", "feather": 0 } },
        { "effect": "blur", "params": { "radius": 6 },
          "mask": { "shape": { "rect": { "size": [1920, 260] } }, "invert": true, "strength": 0.8 },
          "optional": true }
      ] }
```

(En el primer efecto con máscara, `waves` solo actúa donde la imagen `mancha` es clara; en el
segundo, `blur` solo actúa fuera de un rectángulo, y con la fuerza al 80 %.)

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `texture` | | Un recurso de imagen. Debe ser `linear` si es una máscara de datos. |
| `layer` | | El id de otra capa. Se dibuja aparte y enmascara por donde cae en pantalla. |
| `shape` | | Una forma (ver [capas.md](capas.md)), centrada en la capa y medida en sus píxeles. |
| `channel` | `luminance` con imagen, `a` con capa o forma | `r`, `g`, `b`, `a` o `luminance`. |
| `invert` | `false` | Enmascara el exterior. |
| `feather` | 0 | Suaviza el borde. |
| `strength` | 1 | De 0 a 1: `mezcla(1, máscara, fuerza)`. |

Pon exactamente una de `texture`, `layer` y `shape`. Desde la terminal:

```
reiwand-ctl scene act MI-CARPETA effects '{"mask":{"layer":"fondo","index":1,"shape":{"rect":{"size":[1920,260]}},"invert":true}}'
```

Con `scene act CARPETA paint_mask` se pinta una máscara de imagen con trazos y, en el editor, con un pincel
(ver [editor.md](editor.md)).

## Los efectos que trae el programa

Cada efecto lleva su nombre en español como aparece en la app, su costo y sus parámetros, con lo
que valen prellenados y el rango que el programa acepta. Los rangos se comprueban al validar:
`/layers/2/effects/0/params/radius is 300, expected 0..=128`.

Algunos datos que valen para varios:

- `tilt_shift` desenfoca fuera de una franja o, si le das un `depth`, según un mapa de profundidad.
- `glitch` se queda quieto hasta que pones `bursts` o `beat`.
- `film_grain` solo cambia con el tiempo si `fps` es mayor que 0.
- `depth_parallax` no cuesta nada mientras no mueva nada (sin órbita, con el puntero lejos o con
  `strength` en 0). `overscan` agranda la capa para que no se vean los bordes.
- Los efectos con una fase (`phase`) son imágenes `linear` cuyo brillo desplaza el tiempo de cada
  punto, para que las partes vecinas no se muevan juntas.
- Las imágenes `gradient` son de una sola fila, y se leen de izquierda a derecha.

### Movimiento

**`caustics`** (Cáusticas, ligero). Luces móviles como el sol a través del agua.

Parámetros: `color` (color, valor prellenado `#cfefff`); `scale` (número, valor prellenado 2; de 0,1 a 16); `speed` (número, valor prellenado 0,5; de 0 a 8); `strength` (número, valor prellenado 0,6; de 0 a 4); `perspective` (número, valor prellenado 0; de 0 a 1).

**`clouds`** (Nubes y niebla, medio). Nubes o niebla que pasan, hechas con ruido.

Parámetros: `color` (color, valor prellenado `#ffffff`); `shade` (color, valor prellenado `#a7b0c2`); `opacity` (número, valor prellenado 0,8; de 0 a 1); `coverage` (número, valor prellenado 0,45; de 0 a 1); `softness` (número, valor prellenado 0,3; de 0,01 a 1); `size` (número, valor prellenado 320; de 8 a 16384); `speed` (`[x, y]`, valor prellenado `[6, 0]`; de -1024 a 1024); `warp` (número, valor prellenado 0,8; de 0 a 4); `octaves` (número, valor prellenado 5; de 1 a 8); `height` (número, valor prellenado 1; de 0 a 1).

**`electric`** (Eléctrico, ligero). Líneas finas y crepitantes donde se cruzan dos ruidos que se desplazan.

Parámetros: `color_a` (color, valor prellenado `#66ccff`); `color_b` (color, valor prellenado `#ffffff`); `bounds` (`[x, y]`, valor prellenado `[0.4, 0.6]`; de 0 a 1); `scale` (número, valor prellenado 2; de 0,1 a 16); `smoothness` (número, valor prellenado 0,1; de 0 a 1); `speed` (número, valor prellenado 1; de 0 a 8); `multiply` (número, valor prellenado 1; de 0 a 8); `blend` (modo de mezcla, valor prellenado `add`).

**`fire`** (Fuego, ligero). Levanta y hace parpadear las llamas que ya hay en la imagen, y las colorea según el brillo con un degradado.

Parámetros: `speed` (número, valor prellenado 1; de 0 a 8); `strength` (número, valor prellenado 8; de 0 a 64); `scale` (número, valor prellenado 3; de 0,1 a 16); `gradient` (imagen).

**`flow`** (Flujo, ligero). Deforma la imagen a lo largo de un mapa de flujo; sin mapa usa un campo de flujo propio.

Parámetros: `map` (imagen); `strength` (número, valor prellenado 8; de 0 a 512); `speed` (número, valor prellenado 0,25; de 0 a 8); `direction` (`[x, y]`, valor prellenado `[1, 0]`; de -1 a 1); `noise` (sí/no, valor prellenado no); `phase` (imagen).

**`fluid`** (Fluido, pesado). Humo, fuego o líquido simulados sobre la capa, con emisores de punto y de línea y el puntero. Cada emisor es `{ "from", "to", "angle", "strength" }`, en fracciones de la capa (de -2 a 4); `angle` de -360 a 360 (-90 es hacia arriba) y `strength` de 0 a 64; hasta 64 emisores.

Parámetros: `render` (opción, valor prellenado 0): 0 degradado, 1 color del emisor, 2 la propia imagen, 3 solo distorsión; `gradient` (imagen); `color` (color, valor prellenado `#ffffff`); `velocity_decay` (número, valor prellenado 0,98; de 0 a 1); `dye_decay` (número, valor prellenado 0,97; de 0 a 1); `push` (número, valor prellenado 1; de 0 a 8); `emitters` (lista de emisores); `strength` (número, valor prellenado 1; de 0 a 8); `blend` (modo de mezcla, valor prellenado `normal`); `perspective` (número, valor prellenado 0; de 0 a 1); `opaque` (sí/no, valor prellenado no).

**`glitch`** (Fallo digital, ligero). Fallo digital con filas desplazadas y colores partidos. Se queda quieto hasta que pones `bursts` o `beat` (un golpe lo mantiene `length` segundos).

Parámetros: `bursts` (número, valor prellenado 0; de 0 a 10); `beat` (sí/no, valor prellenado no); `length` (número, valor prellenado 0,2; de 0,02 a 2); `rows` (número, valor prellenado 24; de 1 a 256); `threshold` (número, valor prellenado 0,7; de 0 a 1); `shift` (número, valor prellenado 24; de 0 a 512); `split` (número, valor prellenado 4; de 0 a 64); `jitter` (número, valor prellenado 2; de 0 a 64); `speed` (número, valor prellenado 12; de 0 a 60).

**`scroll`** (Desplazamiento continuo, ligero). Desplaza la imagen y la repite en los bordes (o no, con `repeat_x` y `repeat_y` apagados).

Parámetros: `speed` (`[x, y]`, valor prellenado `[0.01, 0]`; de -4 a 4); `repeat_x` (sí/no, valor prellenado sí); `repeat_y` (sí/no, valor prellenado sí).

**`shake`** (Sacudida, ligero). Sacude la imagen a lo largo de una dirección o de un mapa de flujo pintado, con ruido o con un empuje periódico.

Parámetros: `strength` (número, valor prellenado 0; de 0 a 256); `speed` (número, valor prellenado 2; de 0 a 60); `direction` (`[x, y]`, valor prellenado `[1, 1]`; de -1 a 1); `noise` (número, valor prellenado 0; de 0 a 1); `friction` (número, valor prellenado 0; de 0 a 1); `bounds` (`[x, y]`, valor prellenado `[1, 1]`; de 0 a 1); `phase` (imagen); `flow` (imagen); `motion` (opción, valor prellenado 0): 0 ruido, 1 empuje periódico; `side` (opción, valor prellenado 0): 0 a los dos lados, 1 solo hacia delante, 2 solo hacia atrás. El empuje periódico es un empuje suave con el retorno que fija `friction`, no el diente de sierra exacto del efecto de paquete `flow_shake`; al migrar un paquete puedes conservar su efecto.

**`spin`** (Giro, ligero). Gira la parte de la imagen cercana a un centro, con menos fuerza hacia `radius`.

Parámetros: `center` (punto, valor prellenado `[0.5, 0.5]`); `angle` (número, valor prellenado 0; de -360 a 360); `speed` (número, valor prellenado 0; de -720 a 720); `radius` (número, valor prellenado 0,5; de 0 a 2).

**`sway`** (Vaivén, ligero). Balancea la imagen como algo colgado, o con `foliage` hace temblar las hojas.

Parámetros: `amplitude` (número, valor prellenado 6; de 0 a 256); `speed` (número, valor prellenado 0,4; de 0 a 10); `stiffness` (número, valor prellenado 2; de 0 a 8); `spread` (número, valor prellenado 1,5; de 0 a 64); `swing` (sí/no, valor prellenado no); `angle` (número, valor prellenado 3; de 0 a 90); `pivot` (`[x, y]`, valor prellenado `[0.5, 0]`; de -1 a 2); `double` (sí/no, valor prellenado no); `phase` (imagen); `foliage` (sí/no, valor prellenado no); `ratio` (número, valor prellenado 1; de 0,1 a 10).

**`trail`** (Estela de movimiento, medio). Deja copias que se desvanecen de lo que dibujaron los efectos anteriores, igual a cualquier ritmo de fotogramas.

Parámetros: `persistence` (número, valor prellenado 0,8; de 0 a 0,99).

**`water_rings`** (Anillos en el agua, ligero). Anillos que se expanden, como gotas en el agua.

Parámetros: `amplitude` (número, valor prellenado 4; de 0 a 128); `wavelength` (número, valor prellenado 48; de 2 a 4096); `speed` (número, valor prellenado 32; de -1024 a 1024); `falloff` (número, valor prellenado 400; de 1 a 16384); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2).

**`water_ripple`** (Ondulaciones del agua, medio). Ondas de agua que reaccionan al puntero y a los clics.

Parámetros: `strength` (número, valor prellenado 32; de 0 a 256); `damping` (número, valor prellenado 0,96; de 0,5 a 0,97); `radius` (número, valor prellenado 24; de 1 a 256); `cursor` (número, valor prellenado 1; de 0 a 8); `click` (número, valor prellenado 1; de 0 a 8); `beat` (número, valor prellenado 0; de 0 a 8); `specular` (número, valor prellenado 0,15; de 0 a 2); `perspective` (número, valor prellenado 0; de 0 a 1); `reflection` (imagen); `tint` (color, valor prellenado `#ffffff`); `walls` (imagen); `speed` (número, valor prellenado 1; de 0 a 4).

**`waves`** (Ondas, ligero). Distorsiona la imagen con una o dos ondas.

Parámetros: `amplitude` (número, valor prellenado 8; de 0 a 256); `frequency` (número, valor prellenado 4; de 0 a 64); `speed` (número, valor prellenado 1; de -20 a 20); `direction` (número, valor prellenado 0; de -360 a 360); `count` (entero, valor prellenado 1; de 1 a 2); `angle2` (número, valor prellenado 90; de -360 a 360); `perspective` (número, valor prellenado 0; de 0 a 1); `phase` (imagen).

### Luz

**`bloom`** (Resplandor, medio). Hace brillar hacia fuera las partes claras. Con `tint` tiñe el resplandor.

Parámetros: `threshold` (número, valor prellenado 0,8; de 0 a 4); `intensity` (número, valor prellenado 1; de 0 a 8); `radius` (número, valor prellenado 16; de 0 a 128); `knee` (número, valor prellenado 0,5; de 0 a 1); `tint` (color, valor prellenado `#ffffff`).

**`god_rays`** (Rayos de luz, medio). Rayos de luz que salen de una fuente brillante.

Parámetros: `source` (`[x, y]`, valor prellenado `[0.5, 0.15]`; de -1 a 2); `threshold` (número, valor prellenado 0,7; de 0 a 4); `density` (número, valor prellenado 0,8; de 0 a 1); `decay` (número, valor prellenado 0,96; de 0,5 a 1); `exposure` (número, valor prellenado 0,6; de 0 a 4); `color` (color, valor prellenado `#fff4e0`); `samples` (número, valor prellenado 12; de 4 a 16); `follow` (sí/no, valor prellenado no); `mode` (opción, valor prellenado 0): 0 el mismo corrimiento alejándose del centro, 1 el mismo corrimiento según `angle`, 2 estrella; `angle` (número, valor prellenado 90; de -360 a 360); `noise` (número, valor prellenado 0; de 0 a 1); `edges` (entero, valor prellenado 6; de 2 a 16).

**`iris`** (Iris, ligero). Oscurece fuera de un círculo que sigue al puntero, como un foco. Necesita el puntero.

Parámetros: `range` (número, valor prellenado 12; de 0 a 512); `follow` (número, valor prellenado 0,05; de 0 a 1); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `saccades` (sí/no, valor prellenado sí); `rate` (número, valor prellenado 1; de 0,1 a 10); `background` (sí/no, valor prellenado no).

**`lens_flare`** (Destello de lente, medio). Un núcleo, un halo y reflejos a lo largo de la línea que va de una luz al centro.

Parámetros: `source` (punto, valor prellenado `[0.3, 0.3]`); `follow` (sí/no, valor prellenado no); `intensity` (número, valor prellenado 1; de 0 a 4); `ghosts` (entero, valor prellenado 4; de 0 a 8); `halo` (número, valor prellenado 0,3; de 0 a 1); `color` (color, valor prellenado `#ffffff`).

**`light_shafts`** (Haces de luz, ligero). Haces de luz suaves y de color hechos con ruido, que añaden luz (se ven también sobre una capa `solid` transparente).

Parámetros: `direction` (opción, valor prellenado 0): 0 lineal, 1 el mismo corrimiento alejándose del centro, 2 desde la esquina; `angle` (número, valor prellenado 30; de -360 a 360); `center` (punto, valor prellenado `[0.5, 0]`); `color` (color, valor prellenado `#fff2cc`); `gradient` (imagen); `exponent` (número, valor prellenado 1; de 0,1 a 8); `intensity` (número, valor prellenado 1; de 0 a 4); `feather` (número, valor prellenado 0,3; de 0 a 1); `smoothness` (número, valor prellenado 0,5; de 0 a 1); `speed` (número, valor prellenado 0,2; de -8 a 8).

**`reflection`** (Reflejo, ligero). Un reflejo ondulado bajo la imagen.

Parámetros: `line` (número, valor prellenado 0,6; de 0 a 1); `opacity` (número, valor prellenado 0,6; de 0 a 1); `fade` (número, valor prellenado 1; de 0 a 1); `amplitude` (número, valor prellenado 2; de 0 a 128); `frequency` (número, valor prellenado 12; de 0 a 512); `speed` (número, valor prellenado 0,5; de -20 a 20); `angle` (número, valor prellenado 0; de -360 a 360).

**`shine`** (Destello, ligero). Pasa una banda de brillo, un anillo o un destello por la imagen.

Parámetros: `angle` (número, valor prellenado 30; de -360 a 360); `width` (número, valor prellenado 0,12; de 0 a 1); `softness` (número, valor prellenado 0,6; de 0 a 1); `intensity` (número, valor prellenado 0,4; de 0 a 4); `color` (color, valor prellenado `#ffffff`); `speed` (número, valor prellenado 0,25; de -4 a 4); `gap` (número, valor prellenado 1; de 0 a 8); `phase` (número, valor prellenado 0; de 0 a 1); `blend` (modo de mezcla, valor prellenado `add`); `gradient` (imagen); `style` (opción, valor prellenado 0): 0 línea, 1 doble línea, 2 anillo, 3 destello.

**`vignette`** (Viñeta, ligero). Oscurece las esquinas.

Parámetros: `intensity` (número, valor prellenado 0,35; de 0 a 1); `radius` (número, valor prellenado 1; de 0 a 2); `softness` (número, valor prellenado 0,65; de 0 a 1); `roundness` (número, valor prellenado 1; de 0 a 1); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `color` (color, valor prellenado `#000000`).

### Color

**`chroma_key`** (Croma, ligero). Quita un color y lo hace transparente, como una pantalla verde.

Parámetros: `key` (color, valor prellenado `#00ff00`); `tolerance` (número, valor prellenado 0,1; de 0 a 1); `softness` (número, valor prellenado 0,08; de 0 a 1); `spill` (número, valor prellenado 0,5; de 0 a 1); `distance` (obligatorio; 0 compara solo el croma y sirve para pantallas verdes o azules con luz despareja, 1 compara el color completo, con su claridad, y es lo que quita un blanco, un gris o un negro sin llevarse los demás tonos); `keep` (obligatorio, número de 0 a 1: la opacidad que conserva el color quitado, 0 lo deja transparente del todo). El editor escribe ambos al añadir el efecto; si faltan en un archivo, la escena se rechaza.

**`color_adjust`** (Ajustes de color, ligero). Ajusta exposición, contraste, saturación y tono.

Parámetros: `exposure` (número, valor prellenado 0; de -5 a 5); `contrast` (número, valor prellenado 1; de 0 a 4); `saturation` (número, valor prellenado 1; de 0 a 4); `hue` (número, valor prellenado 0; de -180 a 180).

**`color_lut`** (Etalonaje, ligero). Etalonaje de color con una tabla `.cube`.

Parámetros: `lut` (tabla); `intensity` (número, valor prellenado 1; de 0 a 1).

**`posterize`** (Posterizar, ligero). Reduce la imagen a pocos niveles de color.

Parámetros: `levels` (número, valor prellenado 6; de 2 a 64); `dither` (número, valor prellenado 0,5; de 0 a 1).

**`pulse`** (Pulso, ligero). Hace latir el brillo (0), el color (1), la opacidad (2) o una luz teñida (3), con el tiempo, con ruido o con una banda del sonido (`audio_band`, de 1 a 64; necesita la entrada de audio solo entonces).

Parámetros: `what` (opción, valor prellenado 0): 0 brillo, 1 color, 2 opacidad, 3 luz teñida; `speed` (número, valor prellenado 1; de 0 a 16); `bounds` (`[x, y]`, valor prellenado `[0, 1]`; de 0 a 4); `tint_low` (color, valor prellenado `#000000`); `tint_high` (color, valor prellenado `#ffffff`); `noise` (número, valor prellenado 0; de 0 a 1); `noise_speed` (número, valor prellenado 1; de 0 a 16); `phase` (imagen); `audio_band` (entero, valor prellenado 0; de 0 a 64).

**`tint`** (Tinte, ligero). Tiñe la imagen con un color (modo 0) o la colorea (modo 1: el color toma el brillo de cada punto).

Parámetros: `color` (color, valor prellenado `#ff8800`); `blend` (modo de mezcla, valor prellenado `multiply`); `amount` (número, valor prellenado 1; de 0 a 1); `mode` (opción, valor prellenado 0): 0 tinte, 1 colorear.

### Desenfoque

**`blur`** (Desenfoque, medio). Desenfoca la imagen con un radio fijo. `blend` y `amount` mezclan la imagen desenfocada con la nítida.

Parámetros: `radius` (número, valor prellenado 8; de 0 a 128); `blend` (modo de mezcla, valor prellenado `normal`); `amount` (número, valor prellenado 1; de 0 a 1).

**`blur_motion`** (Desenfoque de movimiento, medio). Desenfoca en una dirección, como si se moviera rápido. Con `follow` lo hace según el movimiento real de la capa, en píxeles por segundo por `shutter`, hasta `length`.

Parámetros: `angle` (número, valor prellenado 0; de -360 a 360); `length` (número, valor prellenado 16; de 0 a 256); `samples` (número, valor prellenado 12; de 2 a 32); `follow` (sí/no, valor prellenado no); `shutter` (número, valor prellenado 0,05; de 0 a 0,5).

**`blur_radial`** (Desenfoque radial, medio). Desenfoca hacia fuera desde un centro, como un zoom.

Parámetros: `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `strength` (número, valor prellenado 0,1; de 0 a 1); `samples` (número, valor prellenado 8; de 2 a 16); `follow` (sí/no, valor prellenado no).

**`local_contrast`** (Contraste local, medio). Da contraste alrededor de los bordes sin cambiar la exposición general; conviene marcarlo `optional`.

Parámetros: `radius` (número, valor prellenado 8; de 1 a 64); `amount` (número, valor prellenado 0,5; de 0 a 4); `luma_only` (sí/no, valor prellenado sí).

**`tilt_shift`** (Efecto miniatura, medio). Desenfoca fuera de una franja enfocada, como una foto en miniatura; con `depth` desenfoca según una imagen de profundidad.

Parámetros: `focus` (número, valor prellenado 0,5; de 0 a 1); `width` (número, valor prellenado 0,15; de 0 a 1); `softness` (número, valor prellenado 0,25; de 0 a 1); `angle` (número, valor prellenado 0; de -360 a 360); `radius` (número, valor prellenado 12; de 0 a 64); `depth` (imagen).

### Distorsión

**`chromatic_aberration`** (Aberración cromática, ligero). Separa los canales de color cerca de los bordes, como una lente barata.

Parámetros: `amount` (número, valor prellenado 3; de -64 a 64); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `mode` (opción, valor prellenado 0): 0 crece con la distancia al centro, 1 el mismo corrimiento según `angle`, 2 el mismo corrimiento alejándose del centro, 3 crece con el cuadrado de la distancia; `angle` (número, valor prellenado 0; de -360 a 360).

**`depth_parallax`** (Paralaje de profundidad, medio). Desplaza cada punto según un mapa de profundidad, hacia el puntero (`follow`) o en una órbita lenta (`orbit`), para un efecto 3D. `depth` es un recurso `linear` donde lo claro está cerca. `reach` es obligatorio: `circle` (0) recorta la dirección del puntero a un círculo, así que en una esquina cada eje llega a 0,707 del valor del borde; `square` (1) deja que x e y vayan de −1 a 1 cada uno por su cuenta, como el paralaje de las capas, y en una esquina el relieve corre tanto como en el borde. El editor lo añade como `circle` y dibuja el recorrido del puntero. Mientras no moviera nada no cuesta ninguna pasada.

Parámetros: `depth` (imagen); `strength` (número, valor prellenado 24; de 0 a 256); `focus` (número, valor prellenado 0,5; de 0 a 1); `steps` (número, valor prellenado 16; de 8 a 24); `overscan` (número, valor prellenado 1; de 1 a 1,25); `follow` (sí/no, valor prellenado sí); `orbit` (sí/no, valor prellenado no); `scale` (`[x, y]`, valor prellenado `[1, 1]`; de -1 a 1); `smooth` (sí/no, valor prellenado no).

**`displace`** (Desplazar, ligero). Desplaza píxeles con un mapa de normales (modo 0) o de desplazamientos (modo 1), o con ruido si no das mapa.

Parámetros: `map` (imagen); `mode` (opción, valor prellenado 0): 0 normales, 1 desplazamientos; `scale` (número, valor prellenado 1; de 0,1 a 16); `strength` (número, valor prellenado 8; de 0 a 256); `scroll` (`[x, y]`, valor prellenado `[0, 0]`; de -4 a 4); `ratio` (número, valor prellenado 1; de 0,1 a 10).

**`lens_distort`** (Distorsión de lente, ligero). Distorsión de barril o de cojín, como una lente.

Parámetros: `k1` (número, valor prellenado 0,1; de -1 a 1); `k2` (número, valor prellenado 0; de -1 a 1); `fit` (sí/no, valor prellenado sí); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `background` (sí/no, valor prellenado no).

**`perspective`** (Perspectiva, ligero). Deforma las cuatro esquinas de la imagen hacia otro cuadrilátero.

Parámetros: `top_left` (`[x, y]`, valor prellenado `[0.04, 0]`; de -2 a 3); `top_right` (`[x, y]`, valor prellenado `[0.96, 0]`; de -2 a 3); `bottom_right` (`[x, y]`, valor prellenado `[1, 1]`; de -2 a 3); `bottom_left` (`[x, y]`, valor prellenado `[0, 1]`; de -2 a 3).

**`pixelate`** (Pixelar, ligero). Convierte la imagen en grandes píxeles cuadrados o hexagonales.

Parámetros: `size` (número, valor prellenado 8; de 1 a 256); `hex` (sí/no, valor prellenado no).

**`twirl`** (Remolino, ligero). Retuerce la imagen alrededor de un centro.

Parámetros: `angle` (número, valor prellenado 60; de -1080 a 1080); `radius` (número, valor prellenado 256; de 0 a 16384); `center` (`[x, y]`, valor prellenado `[0.5, 0.5]`; de -1 a 2); `aspect` (número, valor prellenado 1; de 0,1 a 10); `inner` (número, valor prellenado 0; de 0 a 1); `noise` (número, valor prellenado 0; de 0 a 1); `repeat` (sí/no, valor prellenado no).

### Estilo

**`crt`** (Pantalla CRT, ligero). Pantalla antigua de tubo, con líneas de barrido y curvatura.

Parámetros: `curvature` (número, valor prellenado 0,1; de 0 a 1); `pitch` (número, valor prellenado 4; de 2 a 64); `scan` (número, valor prellenado 0,3; de 0 a 1); `phosphor` (número, valor prellenado 0,15; de 0 a 1); `vignette` (número, valor prellenado 0,25; de 0 a 1); `glow` (número, valor prellenado 0,15; de 0 a 1).

**`edges`** (Bordes, medio). Dibuja los bordes, con un aspecto de boceto opcional.

Parámetros: `strength` (número, valor prellenado 1; de 0 a 1); `threshold` (número, valor prellenado 0,25; de 0 a 4); `width` (número, valor prellenado 1; de 0,5 a 8); `color` (color, valor prellenado `#000000`); `sketch` (sí/no, valor prellenado no); `paper` (color, valor prellenado `#ffffff`).

**`film_grain`** (Grano de película, ligero). Grano de película; solo cambia con el tiempo si `fps` es mayor que 0.

Parámetros: `amount` (número, valor prellenado 0,05; de 0 a 1); `size` (número, valor prellenado 1,5; de 0,5 a 16); `fps` (número, valor prellenado 0; de 0 a 60); `mono` (sí/no, valor prellenado no); `bias` (número, valor prellenado 0; de -1 a 1); `blend` (modo de mezcla, valor prellenado `overlay`).

**`glitter`** (Destellos, ligero). Chispas pequeñas con forma de estrella en una cuadrícula.

Parámetros: `density` (número, valor prellenado 0,3; de 0 a 1); `size` (número, valor prellenado 2; de 0,5 a 16); `speed` (número, valor prellenado 1; de 0 a 8); `color` (color, valor prellenado `#ffffff`).

**`vhs`** (VHS, ligero). Cinta de vídeo gastada: línea de seguimiento que rueda, filas rotas, nieve, rayas y colores corridos.

Parámetros: `strength` (número, valor prellenado 0,5; de 0 a 2); `speed` (número, valor prellenado 1; de 0 a 8); `noise` (número, valor prellenado 0,3; de 0 a 1); `aberration` (número, valor prellenado 2; de 0 a 16); `band` (número, valor prellenado 1; de 0 a 1); `artifacts` (número, valor prellenado 0; de 0 a 3); `invert` (sí/no, valor prellenado no).

### Mezcla

**`blend_gradient`** (Mezcla por degradado, ligero). Mezcla otra imagen con un mapa de degradado que dice dónde se ve; sin mapa, de izquierda a derecha.

Parámetros: `texture` (imagen); `map` (imagen); `progress` (número, valor prellenado 0,5; de 0 a 1); `softness` (número, valor prellenado 0,1; de 0 a 1).

**`blend_texture`** (Mezclar textura, ligero). Pone una imagen encima de la capa con un modo de mezcla. Puedes encadenar varios; nunca hace la capa más opaca.

Parámetros: `texture` (imagen); `blend` (modo de mezcla, valor prellenado `multiply`); `amount` (número, valor prellenado 1; de 0 a 1); `offset` (`[x, y]`, valor prellenado `[0, 0]`; de -2 a 2); `scale` (`[x, y]`, valor prellenado `[1, 1]`; de 0,01 a 16).

**`dissolve`** (Disolución, ligero). Disuelve la imagen en un patrón granulado.

Parámetros: `progress` (número, valor prellenado 0; de 0 a 1); `size` (número, valor prellenado 48; de 1 a 4096); `edge` (número, valor prellenado 0,06; de 0 a 1); `glow` (color, valor prellenado `#ffa040`).

**`reveal`** (Revelar, ligero). Enseña otra imagen a través de un círculo o una elipse que puede seguir al puntero.

Parámetros: `texture` (imagen); `shape` (opción, valor prellenado 0): 0 círculo, 1 elipse; `radius` (número, valor prellenado 0,15; de 0 a 1); `softness` (número, valor prellenado 0,3; de 0 a 1); `follow` (sí/no, valor prellenado no); `invert` (sí/no, valor prellenado no); `center` (punto, valor prellenado `[0.5, 0.5]`).

### Transformación

**`transform`** (Transformación, ligero). Mueve, escala, gira y sesga la imagen después de los efectos anteriores, dentro de su cuadro.

Parámetros: `offset` (`[x, y]`, valor prellenado `[0, 0]`; de -2 a 2); `scale` (`[x, y]`, valor prellenado `[1, 1]`; de 0,01 a 16); `angle` (número, valor prellenado 0; de -360 a 360); `skew` (`[x, y]`, valor prellenado `[0, 0]`; de -89 a 89); `pivot` (punto, valor prellenado `[0.5, 0.5]`).

## Escribir un efecto propio

Un paquete puede traer hasta 32 efectos propios. Cada uno son dos archivos de hasta 64 KiB: un
**descriptor** (`.json`) que dice qué parámetros tiene y cuánto cuesta, y un **sombreador**
(`.wgsl`) que dice qué hace con cada píxel. Todos los sombreadores de una escena suman hasta 256 KiB.

Se declaran en `scene.json`, con un id (`[a-z0-9_]{1,32}`, distinto de los de los efectos del
programa), y las capas los usan como cualquier otro:

```json
  "effects": {
    "duotono": { "descriptor": "effects/duotono.json", "wgsl": "effects/duotono.wgsl" },
    "niebla": { "descriptor": "effects/niebla.json", "wgsl": "effects/niebla.wgsl" }
  }
```

```json
    { "blend_space": "canvas", "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [{ "effect": "duotono",
        "params": { "sombra": "#12122e", "luz": "#ffcf8a", "cantidad": { "prop": "intensidad" } } },
        { "effect": "niebla", "params": { "color": "#dfe6ff", "cantidad": 0.5 } }] }
```

### El descriptor

```json
{
  "id": "duotono",
  "name": { "es": "Duotono", "en": "Duotone" },
  "mask_mode": "strength",
  "params": [
    { "id": "sombra", "type": "color", "default": "#1b1b3a" },
    { "id": "luz", "type": "color", "default": "#ffd9a0" },
    { "id": "cantidad", "type": "float", "default": 1, "min": 0, "max": 1 }
  ],
  "fuse": "color",
  "cost": { "taps": 1, "passes": 1, "scale": 1 }
}
```

| Campo | Qué hace |
|-------|----------|
| `id` | Igual que en `effects` de la escena. |
| `name` | El nombre, un texto o uno por idioma: `{ "es": "Duotono", "en": "Duotone" }`. |
| `params` | Hasta 24 parámetros: `float`, `int`, `bool`, `color`, `vec2`, `texture`, `lut`, `enum`, `blend`, `position`, `mask` y `emitters`. Cada uno con `id`, `default`, `min` y `max`. |
| `passes` | Las pasadas de dibujo. Sin `fuse`, al menos una: `{ "entry": "fs_niebla" }`. |
| `fuse` | `color` o `uv`, para efectos de una sola pasada que se pueden fundir con los vecinos. |
| `cost` | **Obligatorio**: `{ "taps", "passes", "scale" }`: lecturas de imagen por píxel, número de pasadas y tamaño de las pasadas como fracción de la capa. |
| `mask_mode` | `mix` (lo normal: mezcla con la entrada según la máscara) o `strength` (tú escalas con `mask_at(uv)`; conviene a las distorsiones). |
| `uses` | `time`, `cursor` o `audio`: lo que el efecto necesita. El programa también lo deduce de tu sombreador. |
| `audio` | Hasta 2 lecturas del sonido, cada una con `kind` (`relative`, `absolute` o `surge`), `unit` (`db` o `amplitude`) y, con `surge`, `window` (de 0.05 a 60 s), `reference` (`band` o `all`) y `hold_silence`; sin `surge` esos tres no se escriben. Tu sombreador lee las 64 bandas de la lectura `n` con `heard_band(n, i)`; un `surge` pasa de 1 cuando la banda sube tras un silencio y luego baja. `band()` sigue siendo relativo y en decibelios. |
| `reach` | Un parámetro `float`, en píxeles, en que el efecto agranda la capa. |
| `min_fps` | De 1 a 240: el ritmo con que el efecto aún se ve fluido. |
| `pace` | Cuándo cambia un efecto que depende del tiempo: `{ "rate": "<parámetro>" }`, `{ "steps": [alto, bajo] }`, `{ "sweep": [...] }` o `"outside"`. |
| `velocity` | Un parámetro `bool` que, activado, da al sombreador la velocidad de la capa. |
| `state` | `{ "scale", "settle", "fade" }`: texturas que se guardan entre fotogramas, para simulaciones. |
| `targets` | Hasta 8 texturas con nombre `{ "scale", "format" }`, para encadenar pasadas. |
| `options` | Hasta 8 variantes del sombreador (un `override` de WGSL), compiladas al cargar la escena. |
| `compose`, `solve`, `bounded`, `quality` | Opciones avanzadas: dibujar la capa sobre lo de abajo primero, resolver una homografía, limitar el trabajo a la parte iluminada de una máscara, y cambiar parámetros por nivel de calidad. |
| `stage` | `material` o `particle`, para sombreadores de un material 3D o de un dibujo de partículas (ver [3d.md](3d.md) y [particulas-y-sonido.md](particulas-y-sonido.md)). |

Un descriptor de efecto propio no admite `category` ni `description`: son solo del catálogo del
programa, y escribirlos es un error.

Cada pasada puede tener `entry`, `scale` (fracción del tamaño de la capa), `inputs` (prellenado
`["previous"]`: `previous`, `input`, `source`, `mask`, `pass:N`, `state`, `backdrop`,
`asset:<parámetro>` o `target:<nombre>`), `output`, `target`, `blend` (`replace`, `normal`, `add`,
`multiply`), `copy` y `swap`. Un efecto tiene hasta 16 pasadas.

### El sombreador

Un efecto fundido (`fuse`) define una función cuyo nombre lleva el id del efecto:

- con `"fuse": "color"`: `fn color_<id>(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32>`, sobre
  colores premultiplicados por el alfa;
- con `"fuse": "uv"`: `fn uv_<id>(uv: vec2<f32>) -> vec2<f32>`, que dice de dónde lee cada punto.

```wgsl
fn color_duotono(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let c = straight(color);
    let luma = dot(c.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let toned = mix(from_linear(param(0u).rgb), from_linear(param(1u).rgb), luma);
    let rgb = mix(c.rgb, toned, param(2u).x * mask_at(uv));
    return vec4<f32>(rgb * c.a, c.a);
}
```

Un efecto con pasadas define una función por pasada:

```wgsl
@fragment
fn fs_niebla(in: VertexOut) -> @location(0) vec4<f32> {
    let c = tap(in.uv);
    let haze = from_linear(param(0u).rgb) * c.a;
    let amount = param(1u).x * mask_at(in.uv) * in.uv.y;
    return vec4<f32>(mix(c.rgb, haze, amount), c.a);
}
```

```json
{
  "id": "niebla",
  "name": { "es": "Niebla", "en": "Haze" },
  "mask_mode": "strength",
  "params": [
    { "id": "color", "type": "color", "default": "#c8d4f0" },
    { "id": "cantidad", "type": "float", "default": 0.4, "min": 0, "max": 1 }
  ],
  "passes": [{ "entry": "fs_niebla" }],
  "cost": { "taps": 1, "passes": 1, "scale": 1 }
}
```

Las coordenadas van de (0, 0) arriba a la izquierda a (1, 1) abajo a la derecha. Lo que el
sombreador puede usar:

| Nombre | Qué es |
|--------|--------|
| `param(n)` | El parámetro número *n* (desde `0u`), en `.x`, `.xy` o `.rgb`. |
| `tap(uv)`, `sample(t, uv)` | Leer la imagen de entrada, u otra textura. |
| `mask_at(uv)` | La máscara del uso, de 0 a 1. |
| `px(píxeles)` | Píxeles convertidos al espacio de coordenadas. |
| `straight(c)` | Quita el premultiplicado por alfa. |
| `to_linear`, `from_linear` | Pasan entre los valores del efecto y la luz lineal según `canvas.blend_space`; los efectos no leen el `blend_space` de la capa. |
| `frame.time`, `frame.dt` | El tiempo y lo que ha pasado desde el último fotograma. |
| `frame.cursor`, `frame.inside`, `frame.buttons` | El puntero. |
| `frame.bass`, `mid`, `treble`, `level`, `beat`, `band(i)` | El sonido. |
| `layer_uv(uv)`, `layer_extent()` | Dónde cae el punto dentro de la capa, y su tamaño. |
| `oklab`, `from_oklab`, `gauss`, `hash`, `hash2`, `noise1`, `value_noise`, `gradient_noise`, `fbm(p, octaves)`, `bayer(pixel)` | Ayudas de color y de ruido. |

Un efecto no puede declarar variables de enlace propias: la imagen de entrada, la máscara y hasta
ocho imágenes más llegan por `inputs` y `params`.

Reglas del sombreador: cada bucle tiene que contar con una variable entera desde un literal hasta
un límite literal o una constante (`for (var i = 0u; i < 16u; i++)`), y como mucho 1 024 vueltas;
no valen `while`. Se revisa el trabajo que puede hacer antes de llegar al controlador gráfico.
Si algo falla, el programa dice qué: `/effects/duotono: ...` con la línea y la columna del error.

### Probarlo

```
reiwand-ctl scene act MI-CARPETA import_effect '{"descriptor":"efecto.json"}'
reiwand-ctl pack check MI-CARPETA
```

`import-effect` copia el descriptor y el `.wgsl` que está junto a él a la carpeta y los
comprueba como lo hace quien abre un paquete. Con `reiwand-ctl pack build` y `scene bench` mides su
costo. En el editor, *Efecto nuevo* parte de una plantilla y enseña los errores en el código.

### Otros sombreadores

Con `"stage": "material"`, el sombreador define `fn material(s: Surface) -> Surface` y da forma al
material de un modelo (ver [3d.md](3d.md)); con `"stage": "particle"`, define
`fn particle(p: Sprite) -> Sprite` y cambia cómo se dibuja cada partícula (ver
[particulas-y-sonido.md](particulas-y-sonido.md)).

Un sombreador de partícula puede definir además `fn pixel(p: Pixel) -> vec4<f32>`, que corre por cada
píxel del sprite antes del alfa. `p.sample` es la muestra de la textura, `p.color` el color y el alfa
de la partícula (sin el brillo y sin multiplicar por el alfa), `p.brightness` el brillo del sistema,
`p.alpha` el alfa (`sample.a * color.a`), `p.uv`, `p.age` y `p.life`. Devuelve color y alfa rectos y el
motor multiplica después. Para que el núcleo llegue a blanco sin aclarar el borde translúcido:

```wgsl
fn pixel(p: Pixel) -> vec4<f32> {
    let lit = min(p.sample.rgb * p.color.rgb * p.brightness, vec3<f32>(1.0));
    return vec4<f32>(lit, p.sample.a * p.color.a);
}
```

Con `pixel`, el brillo ya no se multiplica en el color del sprite (lo aplica la función). Sin ella todo
se ve como siempre. Las estelas y las cuerdas no la llaman; `refract` solo lee su alfa.

### Una transición propia

Un fondo puede traer su propia transición para cuando entra y, si quiere, otra para cuando sale.
Se declara en `scene.json`, bajo `transition`, con `in` y `out`; cada una nombra su descriptor
(`"stage": "transition"`) y su `.wgsl`, dura de 100 a 10000 ms (`duration_ms`), lleva una curva
(`curve`, los mismos nombres que las claves) y valores fijos para sus parámetros (`params`). El
sombreador define `fn transition(c: Crossing) -> Crossing`: `before(uv)` es la imagen que se
estaba viendo, `after(uv)` la que entra y `c.progress` va de 0 a 1; el color que deja se dibuja
opaco. Mira [`ejemplos/transicion-propia`](ejemplos/transicion-propia/scene.json).

Una transición no puede usar imágenes, tablas de color, máscaras ni emisores, y cuesta a lo sumo
8 unidades: una liviana se ve en todos los niveles de calidad y una mediana solo en los niveles 2
y 3. Si no pasa las comprobaciones, la escena igual se abre: `reiwand-ctl pack check` dice por qué y
el fondo entra con el fundido de siempre. Lo que el usuario elige manda: sin transición, el cambio
es directo, y con movimiento reducido es un fundido. Al pasar de un fondo con `out` a uno con `in`,
primero se ve la salida y después la entrada. En el teléfono se ve igual, salvo que con movimiento
reducido el cambio es directo, y mientras ahorra batería o la calidad fijada no la permite entra
con el fundido.

## No cubierto aquí

- Efectos de las partículas y de los materiales 3D: [particulas-y-sonido.md](particulas-y-sonido.md)
  y [3d.md](3d.md).
- El editor de efectos, el pincel de máscaras y los editores de rampas: [editor.md](editor.md).
