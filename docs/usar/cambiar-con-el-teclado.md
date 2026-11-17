# Cambiar de fondo con el teclado

Con un atajo puedes recorrer tus fondos sin abrir la aplicación: sobre el propio fondo, atenuado,
aparece una tira centrada con el fondo actual, los anteriores y los siguientes. El del centro va
dentro de un marco y, si es un GIF o WebP, se anima; los demás son miniaturas quietas y debajo del
central se lee su nombre. No se abre ninguna ventana y nada cambia hasta que lo aplicas.

## Con los atajos

Los atajos son globales: no hace falta que nada tenga el foco.

- **Siguiente** y **anterior** (`LOGO+ALT+N` y `LOGO+ALT+P`): empiezan a recorrer y mueven la tira.
  Si la pantalla reproduce una lista, se recorre esa lista; si no, tu biblioteca.
- **Aplicar** (`LOGO+ALT+Return`): deja el del centro, con la transición que corresponda.
- **Cancelar** (`LOGO+ALT+Escape`): cierra la tira; el fondo real sigue igual.
- Si no haces nada durante 30 segundos la tira se cierra sola y no cambia nada.

Con el ratón, mientras la tira está abierta: la rueda mueve la selección, un clic en la miniatura
central la aplica y un clic en una de los lados la elige. El ratón solo actúa donde la ventana del
fondo recibe el puntero.

La tira se desliza al moverse. Si pediste menos movimiento (o lo pide tu sistema) la selección salta
sin deslizar y el central no se anima. Los vídeos y las escenas se ven como una imagen quieta; se
mueven al aplicarlos.

## Activar los atajos

```sh
reiwand-ctl config shortcuts.enabled true
```

El programa pide al escritorio cuatro atajos (`shortcuts.next`, `shortcuts.previous`,
`shortcuts.apply` y `shortcuts.cancel`; también se cambian en Ajustes). En Plasma, GNOME y Hyprland
el escritorio los registra y puede mostrarte su propio diálogo para elegir otras teclas.

## Donde el atajo no se registra solo

En Sway y los demás compositores wlroots, y en X11, el escritorio no ofrece atajos a los programas.
Ata tú las órdenes a teclas; la tira se dibuja igual:

```sh
bindsym $mod+Alt+n exec reiwand-ctl next --preview
bindsym $mod+Alt+p exec reiwand-ctl prev --preview
bindsym $mod+Alt+Return exec reiwand-ctl browse apply
bindsym $mod+Alt+Escape exec reiwand-ctl browse cancel
```

En X11 haz lo mismo con el gestor de atajos de tu entorno (o `xbindkeys`). Con `--output NOMBRE`
la tira recorre solo esa pantalla.

## Límites

- Los fondos que se recorren quedan fijos al empezar: lo que añadas o quites mientras la tira
  está abierta aparece la próxima vez. De la biblioteca se toman hasta 2000.
- Con varias pantallas y ninguna elegida, la tira aparece en todas a la vez y se mueven juntas.
- El escritorio puede no aceptar las teclas propuestas (por ejemplo, si ya las usa otra cosa): elige
  otras en su diálogo o ata las órdenes a mano.
- Las pantallas se nombran por su conector (`DP-1`, `HDMI-A-1`).
- La tira no muestra el autor ni el repositorio del fondo: eso está en la biblioteca.
- Solo se anima el central y solo si su vista previa es un GIF o WebP.
