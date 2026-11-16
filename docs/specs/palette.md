# Paleta de la interfaz

El crate `reiwand-palette` calcula los tonos de la interfaz a partir del fondo en uso. Es un módulo
puro: no decodifica imágenes y no usa dependencias; `derive` no reserva memoria (todo vive en la pila).

## Los tonos

Todos salen del fondo, salvo los cuatro fijos. La interfaz es siempre oscura y el texto va sobre Noche.

| Tono | Campo | Uso |
|---|---|---|
| Noche | `noche` | Tono profundo del fondo; superficies y fondo de paneles |
| Noche alta | `noche_alta` | Paneles elevados |
| Noche línea | `noche_linea` | Líneas y bordes |
| Bruma | `bruma` | Texto secundario |
| Bruma baja | `bruma_baja` | Pies y ayudas |
| Luna | `luna` | Acento en reposo |
| Luna honda | `luna_honda` | Acento pulsado |
| Velo | `velo` | Intensidad del velo bajo el texto (0,62 a 0,96) |
| Escarcha, Ámbar, Coral, Brote | `ESCARCHA`, `AMBAR`, `CORAL`, `BROTE` | Fijos: texto principal y estados |

## Cómo se derivan

Se trabaja en OKLab. El matiz de Noche es el de la mitad más oscura de la imagen (si esa mitad es casi gris,
el de toda la imagen; si no hay matiz, azul). Su croma es el de esa mitad por 1,3, entre 0,014 y 0,040.
Noche, Noche alta y Noche línea tienen luminosidad fija (0,185; 0,235; 0,345). Bruma y Bruma baja usan el mismo
matiz con luminosidad 0,81 y 0,69 y croma casi nulo. El acento toma el matiz más presente: los píxeles con
luminosidad mayor que 0,30 pesan su croma elevado a 1,5, se reparten en 36 sectores de matiz, se suaviza con los
vecinos y se promedia el sector ganador con sus dos contiguos; si nada pesa, usa el matiz de Noche. Luna y Luna
honda tienen luminosidad 0,82 y 0,71. Cada color se baja de croma hasta caber en sRGB. El velo es
0,50 más 0,55 por la luminosidad media, entre 0,62 y 0,96, redondeado a dos decimales.

## Garantías

Como la luminosidad es fija, vale para cualquier fondo: Noche nunca es claro, Escarcha sobre Noche tiene contraste
de al menos 12, Bruma sobre Noche al menos 8 y Bruma baja al menos 4,5. Sin fondo, `fixed()` da el conjunto fijo.

## Uso

`derive(pixels, channels, width, height) -> Option<Palette>`: bytes RGB o RGBA (el alfa se ignora). Quien llama reduce
el cartel a unos 96 × 54 píxeles; la función lee como mucho 8192 píxeles (si hay más, toma uno de cada n).
Devuelve `None` si los datos no cuadran. `Rgb` se imprime como `#rrggbb`. Pruebas contra un cálculo de referencia
independiente en `crates/reiwand-palette/tests`, con tolerancia de un paso por canal.

## Colores principales de una imagen

`dominant(pixels, channels) -> Option<[Rgb; 3]>`: bytes RGB o RGBA (los píxeles con alfa menor que 128 se ignoran). Agrupa
los píxeles en tres grupos (k-means en OKLab, 12 rondas como máximo) y devuelve el centro de cada uno, del grupo más
numeroso al menos. Es el único extractor del proyecto: lo usan el color y los matices de las miniaturas de la biblioteca
(`reiwand-render`), los colores de la portada en curso (`reiwand-scene`, también para el editor) y la app Android. Una imagen
de un solo color devuelve ese color tres veces. `oklab` y `from_oklch` son la única conversión a OKLab; también la usa
la mezcla de colores de las escenas. Devuelve `None` si no queda ningún píxel visible.
