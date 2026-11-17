# Cambiar de fondo con el teclado

Con un atajo se abre una ventana en el centro de la pantalla, sobre todo lo demás, que se atenúa.
Muestra el fondo que quedaría en grande y, a los lados, los anteriores y los siguientes como miniaturas
nítidas; debajo del central se leen su nombre y su posición («3 de 12»). Se abre ya movida un paso,
sigue la lista que reproduce esa pantalla (o tu biblioteca) y nada cambia hasta que la aplicas.

## Con los atajos

Los atajos son globales: no hace falta que nada tenga el foco.

- **Siguiente** y **anterior** (`LOGO+ALT+N` y `LOGO+ALT+P`): abren la ventana y mueven la selección.
- **Aplicar** (`LOGO+ALT+Return`): deja el del centro, con la transición que corresponda.
- **Cancelar** (`LOGO+ALT+Escape`): cierra la ventana; el fondo real sigue igual.
- Si no tocas nada durante 5 segundos (ni teclas, ni atajos, ni rueda, ni mover el puntero sobre la ventana) se cierra sola y no cambia nada; mientras mantienes pulsado un botón sobre una miniatura no se cierra. Si la ventana desaparece de golpe, el demonio descarta la selección en unos 30 segundos.

Con la ventana abierta: las flechas ← y → y la rueda del ratón mueven la selección; Intro, un clic
en el central o el atajo de aplicar lo aplican y cierran; Esc, un clic fuera o el atajo de cancelar
cierran sin cambiar. Un clic en cualquier otra miniatura la elige. Si hay un único
fondo, la ventana lo dice y ofrece abrir la biblioteca; si algo falla, nombra el problema y deja
reintentar.

Sigue el esquema claro u oscuro del sistema. Si pediste menos movimiento (o lo pide tu sistema) la
selección salta sin deslizar y el central no se anima.

Al cerrarla, el proceso de la ventana queda oculto durante 5 minutos, sin trabajar (no consume
procesador; guarda unos 180 MB de memoria) y, pasado ese tiempo sin usarla, se cierra. Así la
siguiente vez aparece al instante (unos 50 ms); la primera vez tras iniciar sesión, o tras 5 minutos
sin usarla, tarda unos 300 ms porque tiene que arrancar.

Solo el fondo del centro se mueve: las escenas y los vídeos se ven en marcha dentro del recuadro,
dibujados por el demonio en un destino aparte, sin sonido y sin tocar el fondo real; las demás
miniaturas son imágenes quietas. Al pasar a otro fondo o al cerrar la ventana el destino se libera, así
que no queda nada trabajando. Sigue quieto si el equipo va con batería o si el fondo es fotosensible. La animación usa
los módulos QML de KPipeWire, los mismos que las tarjetas animadas de la biblioteca; sin ellos el
central queda como imagen quieta.

## Activar los atajos

```sh
reiwand-ctl config shortcuts.enabled true
```

El programa pide al escritorio cuatro atajos (`shortcuts.next`, `shortcuts.previous`,
`shortcuts.apply` y `shortcuts.cancel`; también se cambian en Ajustes). En Plasma, GNOME y Hyprland
el escritorio los registra y puede mostrarte su propio diálogo para elegir otras teclas.

## Donde el atajo no se registra solo

En Sway y los demás compositores wlroots, y en X11, el escritorio no ofrece atajos a los programas.
Ata tú las órdenes a teclas; la ventana se abre igual:

```sh
bindsym $mod+Alt+n exec reiwand-ctl next --preview
bindsym $mod+Alt+p exec reiwand-ctl prev --preview
bindsym $mod+Alt+Return exec reiwand-ctl browse apply
bindsym $mod+Alt+Escape exec reiwand-ctl browse cancel
```

En X11 haz lo mismo con el gestor de atajos de tu entorno (o `xbindkeys`). Con `--output NOMBRE`
la ventana se abre solo en esa pantalla.

## Límites

- Los cambios de la biblioteca hechos con la ventana abierta se ven la próxima vez que la abras; de
  la biblioteca se toman como máximo dos mil fondos.
- Con varias pantallas y ninguna elegida, la ventana se abre en la pantalla donde está el puntero (Plasma) y recorre la lista de esa pantalla; con `--output NOMBRE` se abre en la que indiques. Fuera de Plasma se usa la primera pantalla.
- El escritorio puede no aceptar las teclas propuestas (por ejemplo, si ya las usa otra cosa): elige
  otras en su diálogo o ata las órdenes a mano.
- Las pantallas se nombran por su conector (`DP-1`, `HDMI-A-1`).
- La ventana no muestra el autor ni el repositorio del fondo: eso está en la biblioteca.
- Solo se anima el central y solo si su vista previa es un GIF o WebP.
