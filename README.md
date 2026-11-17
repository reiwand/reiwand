# Reiwand

Fondos de pantalla para Linux, libres y escritos en Rust: imágenes y vídeos en bucle, con
transiciones, un fondo distinto por pantalla y pausa automática cuando no se ven.

Un daemon dibuja el fondo de cada pantalla con la GPU y KDE Plasma 6 lo muestra a través
de un plugin de fondo; en Sway, Hyprland, river, niri, COSMIC y las sesiones X11 el daemon lo
dibuja él mismo. Todo se gestiona desde la app de escritorio, desde la terminal (`reiwand-ctl`) o
desde el icono de la bandeja del sistema.

## Qué puede hacer

- Imágenes, vídeos en bucle, escenas animadas, fondos web y modelos 3D como fondo, con transiciones.
- Un fondo distinto por pantalla, con listas que cambian solas o con un atajo de teclado.
- Pausa automática cuando una ventana tapa el fondo, con batería o con la pantalla bloqueada.
- Paquetes `.wpak` propios, con repositorios, firmas y un servidor propio opcional.
- Editor de escenas integrado, para crear fondos con capas, efectos, partículas, sonido y scripts.
- Iluminación RGB, reproductor multimedia y una app para Android para llevar tus fondos al teléfono.

## Instalar

Descarga `reiwand-x86_64-linux.tar.gz` de la página *Releases* y ejecuta:

```sh
tar xzf reiwand-x86_64-linux.tar.gz
cd reiwand-*-x86_64-linux
./install.sh
```

Se instala en tu directorio personal, sin root. Para desinstalar: `./uninstall.sh`. La comprobación
de la firma, Flatpak y los demás escritorios están en las páginas de la tabla de abajo.

## Primeros pasos

1. Abre *reiwand* desde el lanzador de aplicaciones, o desde el icono de la bandeja del sistema.
2. Elige un fondo de la biblioteca (o añade una imagen, un vídeo o un paquete) y ponlo.
3. En Plasma, el fondo se muestra a través del plugin; los pasos están en la página de instalación.
4. Desde la terminal, `reiwand-ctl --help` lista lo que se puede hacer.

## Dónde leer

| Tema | Página |
| --- | --- |
| Guía de uso | [`docs/usar/LEEME.md`](docs/usar/LEEME.md) |
| La app | [`docs/usar/app.md`](docs/usar/app.md) |
| El teléfono | [`docs/usar/telefono.md`](docs/usar/telefono.md) |
| Instalación sin root y versiones | [`docs/usar/instalacion.md`](docs/usar/instalacion.md) |
| Instalación con Flatpak | [`docs/usar/flatpak.md`](docs/usar/flatpak.md) |
| Sway, Hyprland, river, niri, COSMIC y X11 | [`docs/usar/otros-escritorios.md`](docs/usar/otros-escritorios.md) |
| Android | [`docs/usar/android.md`](docs/usar/android.md) |
| Uso desde la terminal | [`docs/usar/terminal.md`](docs/usar/terminal.md) |
| Cambiar de fondo con el teclado | [`docs/usar/cambiar-con-el-teclado.md`](docs/usar/cambiar-con-el-teclado.md) |
| Paquetes: clasificación, idiomas y créditos | [`docs/usar/paquetes.md`](docs/usar/paquetes.md) |
| Fondos web | [`docs/usar/fondos-web.md`](docs/usar/fondos-web.md) |
| Iluminación RGB | [`docs/usar/luces-rgb.md`](docs/usar/luces-rgb.md) |
| Reproductor multimedia | [`docs/usar/reproductor.md`](docs/usar/reproductor.md) |
| Servidor propio | [`docs/usar/servidor.md`](docs/usar/servidor.md) |
| Privacidad | [`docs/usar/privacidad.md`](docs/usar/privacidad.md) |
| Crear fondos (autores) | [`docs/crear/LEEME.md`](docs/crear/LEEME.md) |
| Desarrollo | [`docs/desarrollo/desarrollo.md`](docs/desarrollo/desarrollo.md) |
| Especificaciones | [`docs/specs/`](docs/specs) |
| Índice de todos los documentos | [`docs/LEEME.md`](docs/LEEME.md) |

## Privacidad

Sin telemetría ni estadísticas de uso. El programa solo se conecta a internet para los proveedores de
fondos, los repositorios y el servidor propio que añadas tú. El modo privado (`reiwand-ctl private on`)
corta toda la red de una vez. El detalle está en [`docs/usar/privacidad.md`](docs/usar/privacidad.md).

## Licencia

GPL-3.0-or-later. Ver [`LICENSE`](LICENSE). Las escenas de ejemplo de `scenes/` son de dominio
público (CC0 1.0, [`scenes/LICENSE`](scenes/LICENSE)).

Los modelos no vienen incluidos: se descargan cuando lo pides y cada uno se guarda con el texto de
su licencia. Depth Anything V2 Small (Apache-2.0), LaMa big-lama (Apache-2.0, el archivo
`lama_fp32.onnx` de la exportación ONNX publicada como `Carve/LaMa-ONNX` en huggingface.co, 208 044 816
bytes, SHA-256 `1faef530…68d6`, licencia Apache-2.0 declarada en esa ficha y en
[`advimman/lama`](https://github.com/advimman/lama/blob/main/LICENSE)) y ONNX Runtime (MIT).
`reiwand-ctl models list` y los ajustes muestran tamaño, licencia y servidores antes de cada descarga.
