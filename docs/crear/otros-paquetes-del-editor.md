# Fondos web y otros paquetes en el editor

El editor también abre y crea paquetes que no son escenas. En *Escenas*, el punto de partida
*Página web* crea una carpeta con `index.html`, `app.js` (una hora que toma el color de su propiedad
y se detiene con la página oculta) y un manifiesto `web` válido; *Abrir una carpeta de escena o
paquete…* y las copias de la biblioteca abren también páginas web, imágenes, vídeos y presets. La
página del paquete muestra la lista de archivos con su papel (la página de entrada, el script que
corre antes, la vista previa), su tamaño y el total; *Añadir archivos…* los copia a la carpeta y
cada archivo ofrece usarlo como página de entrada o como miniatura. Los problemas, deshacer,
guardar, *Poner de fondo* y *Publicar…* funcionan como en las escenas.

```sh
reiwand-ctl scene new reloj --from web --title Reloj   # index.html, app.js y el manifiesto
reiwand-ctl scene open reloj                           # los archivos con su papel y tamaño
reiwand-ctl scene act reloj manifest '{"add_file":{"file":"fondo.png","to":"img"}}'   # copia un archivo a la carpeta
```
