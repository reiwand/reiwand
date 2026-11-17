# Sway, Hyprland, river, niri y COSMIC

Aquí no hace falta plugin: `reiwandd` pone en cada pantalla una superficie en la capa de fondo
(`wlr-layer-shell`) y dibuja en ella con la GPU, sin copias. Se instala con el `.tar.gz` y se
arranca desde la configuración del compositor:

```sh
exec reiwandd                       # Sway (~/.config/sway/config)
exec-once = reiwandd                # Hyprland (~/.config/hypr/hyprland.conf)
riverctl spawn reiwandd             # river (~/.config/river/init)
spawn-at-startup "reiwandd"         # niri (~/.config/niri/config.kdl)
```

Si tu configuración de Sway tiene una línea `output … bg …` (o usas otro programa de fondo en la
capa de fondo), conviene saber cómo las ordena el compositor: en la misma capa se ve encima la
superficie más antigua, no la más nueva. `reiwandd` mantiene viva su superficie (al quitar el fondo la
deja sin imagen en vez de destruirla), así que conserva su sitio al quitar y recuperar el fondo y
cuando recargas Sway (`swaymsg reload` vuelve a lanzar `swaybg`, que pasa a ser el más nuevo). Lo que
el protocolo no permite es subir una superficie ya creada: si `swaybg` se creó antes que la de
`reiwandd` (al arrancar `reiwandd` después de él, o cuando una pantalla se conecta o se vuelve a activar y
`swaybg` llega primero), el fondo de Sway queda encima del de `reiwandd`. En ese caso ejecuta `swaymsg
reload` una vez, o quita la línea `output … bg …`; sin ella, al quitar el fondo se ve el color liso
propio de Sway.

En COSMIC, añádelo a las aplicaciones de inicio y elige un color sólido como fondo propio del
escritorio. Si `install.sh` dejó activo el servicio de usuario, pásale el entorno gráfico desde la
misma configuración en vez de lanzar el binario: `systemctl --user import-environment
WAYLAND_DISPLAY && systemctl --user restart reiwandd` (en Hyprland, importa también
`HYPRLAND_INSTANCE_SIGNATURE`).

`reiwandd` elige solo: con Plasma (`plasmashell` en el bus o `KDE` en `XDG_CURRENT_DESKTOP`) usa los
streams del plugin y en otro escritorio Wayland con `wlr-layer-shell`, sus propias superficies.
`reiwandd --backend streams|layer-shell|x11` lo fuerza. Mientras no hay fondo elegido (primer arranque,
o después de *Quitar el fondo* o `reiwand-ctl clear`) `reiwandd` no pone nada en la pantalla: en
layer-shell destruye su superficie, así que se ve el fondo del propio escritorio, y en X11 destruye
la ventana y devuelve a la ventana raíz lo que mostraba antes de que `reiwandd` dibujara (el pixmap de
`_XROOTPMAP_ID`, o una copia de lo que se veía; X no dice el color liso de `xsetroot`, por eso vuelve
como copia). Sin fondo no hay fotogramas, temporizadores ni trabajo de GPU en esa pantalla, y la
superficie se crea de nuevo al poner o recuperar un fondo. En Plasma el plugin no puede apartarse
solo: muestra el color de fondo, y *Usar el fondo propio de Plasma* (o `reiwand-ctl plasma-back`) pide a
Plasma, con su interfaz de scripts, que vuelva al tipo de fondo que tenía; poner o recuperar un
fondo en reiwand-ctl lo vuelve a activar. Quitar de la biblioteca el elemento que es el fondo actual
también deja libres sus pantallas. Las pantallas se llaman como las anuncia el
compositor (`DP-1`, `HDMI-A-1`…; las muestran `swaymsg -t get_outputs`, `hyprctl monitors`,
`niri msg outputs` o `wlr-randr`) y esos nombres son los de `-o` y de la configuración. Las
pantallas que se conectan o se quitan con la sesión abierta se siguen solas, y la escala
fraccional se respeta. Un cambio de pantallas se aplica cuando lleva medio segundo estable, así que
una pantalla que el compositor anuncia dos veces, quita y vuelve a poner, o deja un instante sin
tamaño no rehace su superficie ni reinicia su vídeo. Si el compositor corta la conexión por un error
de protocolo, `reiwandd` sale con estado 70 y el servicio de usuario lo arranca de nuevo.

Cada superficie lleva un nombre (el *namespace* de `wlr-layer-shell`), que por defecto es
`wallpaper`. Algunos compositores deciden por ese nombre si la superficie es el fondo del
escritorio, y solo entonces la muestran detrás de su vista general; si la tuya se ve negra, elige
otro nombre. Manda, en este orden, `reiwandd --layer-namespace NOMBRE`, la variable
`REIWAND_LAYER_NAMESPACE`, la clave `surfaces.layer_namespace` de `config.toml` (`reiwand-ctl config
surfaces.layer_namespace NOMBRE`, o *Ajustes* → *Superficies del escritorio* en la app) y una
elección automática para los escritorios conocidos: con `KineticWE` en `XDG_CURRENT_DESKTOP` usa
`noctalia-wallpaper`. `auto` deja pasar al siguiente, y en la clave también un valor vacío. El
nombre tiene de 1 a 64 letras ASCII, cifras, `-`, `_` o `.`; la opción rechaza uno mal escrito o
vacío, la clave rechaza uno mal escrito, y una variable vacía o mal escrita se ignora con un
aviso en el registro y en `ignored` del estado. Vale desde el siguiente arranque del daemon y solo con superficies
`layer-shell`: `reiwand-ctl state` muestra en `layer_shell` el nombre en uso, de dónde salió, si un
cambio espera a ese arranque y qué valor se ignoró, y `reiwand-ctl config surfaces.layer_namespace
NOMBRE` y la app lo avisan. No viaja en el archivo exportado ni entra en los perfiles, porque depende del
escritorio de cada equipo.

El daemon arranca como servicio de usuario de systemd, que no ve las variables ni las opciones de
tu terminal. Lo más simple es la clave o el ajuste de la app; para la variable o la opción, usa
`systemctl --user edit reiwandd` y añade `Environment=REIWAND_LAYER_NAMESPACE=NOMBRE` en `[Service]`, o
un `ExecStart=` propio con `--layer-namespace NOMBRE`, y reinicia el servicio. La elección
automática lee `XDG_CURRENT_DESKTOP` del entorno que la sesión pasa a systemd; si no llega, usa la
clave.

Los vídeos se pausan cuando una ventana tapa la pantalla: el compositor deja de pedir fotogramas
al fondo, una ventana activa a pantalla completa o maximizada lo confirma y, en Hyprland, también
su IPC; en COSMIC, sus avisos de solapamiento, cuando las ventanas, junto con los paneles, cubren
todo el fondo. Una pantalla que vuelve a conectarse no se queda tapada por un aviso de la superficie
anterior, y si el compositor cierra la superficie del fondo con la pantalla conectada, `reiwandd` la
vuelve a crear: tres veces por minuto como mucho, y si la cierra más veces, la crea otra vez en
cuanto pasa ese minuto. El Flatpak no sirve en estos escritorios: no dejan usar `wlr-layer-shell`
ni la información de ventanas a las aplicaciones aisladas.

## Sesiones X11

Con `DISPLAY` y sin sesión Wayland ni Plasma, `reiwandd` pone en cada monitor de RandR una ventana de
escritorio debajo de todas las demás (si el gestor de ventanas no las coloca, una ventana
*override-redirect* que se mantiene al fondo) y dibuja en ella con la GPU. Los monitores se llaman
como en `xrandr --listmonitors` y se siguen al conectarlos o quitarlos. Con imágenes fijas también
pone el fondo de la ventana raíz (`_XROOTPMAP_ID` y `ESETROOT_PMAP_ID`) para los compositores y las
terminales con transparencia (y lo vuelve a pintar si la ventana raíz cambia de tamaño), y los
vídeos se pausan cuando una ventana del escritorio actual está a pantalla completa o maximizada en
ese monitor según EWMH, cuando varias ventanas en mosaico lo cubren entre todas o, sin compositor,
cuando el servidor avisa de que la ventana del fondo quedó tapada del todo. La ventana de escritorio
de otro programa, como la de los iconos de un gestor de archivos, queda siempre por encima del fondo
(y, si es opaca, la pantalla cuenta como tapada), y sin gestor de ventanas `reiwandd` deja de disputar
el fondo de la pila a otro programa que mantiene allí su ventana después de bajar las suyas tres
veces en 10 s. Se arranca
con `reiwandd &` en `~/.xinitrc`, antes del gestor de ventanas, o desde el arranque automático de
este; con el servicio de usuario,
`systemctl --user import-environment DISPLAY && systemctl --user restart reiwandd`. Si el gestor de
ventanas arranca después que `reiwandd` (o se cierra), las ventanas pasan solas a ser ventanas de
escritorio gestionadas (o *override-redirect*). Cuando una ventana deja de tapar el fondo, se dibuja
otra vez el último fotograma, también con el vídeo en pausa, sin reiniciarlo.
