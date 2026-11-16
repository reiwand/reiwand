# Página Crear

Es donde un autor empieza: un proyecto nuevo, uno reciente, una copia de un fondo instalado o una
carpeta. Usa el esqueleto de título; el editor se abre como siempre, con `host.work`.

## Qué muestra

- «Seguir con lo tuyo»: los proyectos recientes, el más nuevo primero. Se guardan en
  `recent-projects.json`, en el directorio de estado de la app (200 como máximo, con una marca
  «reciente» por proyecto y la lista de los olvidados), y se suman las carpetas que haya en la
  carpeta de proyectos. Es estado local de la app, no del servicio: no tiene orden en `reiwand-ctl`. Una carpeta que ya no existe sigue en la fila, marcada
  «Su carpeta ya no está», con «Buscarla…» y «Quitar de la lista». La fila muestra solo los que
  caben a lo ancho y termina en «Todos mis proyectos N».
- «Todos mis proyectos»: una rejilla (la misma de «Ver todos» de la biblioteca) con todos los
  proyectos que la app conoce, el más nuevo primero, y un buscador que filtra por nombre. Intro, el
  botón del pie o un doble clic abre el proyecto; Esc o «Atrás» vuelve a la fila.
- Clic derecho o la tecla de menú sobre un proyecto (en la fila o en la rejilla) abre un menú con
  «Seguir editando» («Buscarla…» si falta la carpeta), «Quitar de recientes» (sale de la fila, sigue
  en «Todos mis proyectos») y «Quitar de la lista» (en la rejilla, o en la fila si falta la carpeta:
  la app lo olvida aunque esté en la carpeta de proyectos; el aviso ofrece «Deshacer»). Ninguna
  borra archivos. «Limpiar», junto al título de la fila, la vacía con «Deshacer». Un proyecto vuelve
  a la fila cuando se edita otra vez.
- «Empezar uno nuevo»: una escena (vacía, partículas, paralaje por capas, reloj de texto), un fondo
  de un solo archivo (imagen, vídeo, página web) y «Para reutilizar» (preajuste y paquete de recursos),
  las tres a la vista, con el mismo pie.
- «Aprender con un ejemplo»: las escenas de ejemplo que vienen dentro de la app (las de `scenes/`
  del repositorio, unos 350 KB), en toda instalación, sin red ni repositorio. Elegir una abre el paso
  «Editar una copia…» con la licencia y el nombre de la copia; la app arma antes un paquete temporal
  en su caché. Los paquetes instalados se copian desde su propia tarjeta (biblioteca o Explorar).
- «Abrir un proyecto…»: elige la carpeta de un proyecto que no está en la lista (la que tiene su
  `manifest.json` y, si es una escena, su `scene.json`); su ayuda lo dice. Si la carpeta no es un
  proyecto, el aviso dice qué se esperaba en ella y ofrece «Elegir otra».

Sin proyectos, la página abre en «Aprender con un ejemplo». El pie sigue
la tarjeta marcada: título, una línea, «Cambiar…» (título y carpeta) y el botón principal.
«Cambiar…» deja en una sola fila el nombre del punto de partida, «Título», «Carpeta» y «Crear», con
el foco en «Título»; la carpeta se muestra con `~` para la carpeta personal y también se puede
escribir así. Intro en cualquiera de los dos campos crea; Esc vuelve a las tarjetas con la misma
tarjeta marcada. En una ventana demasiado angosta los campos pasan debajo del título y «Crear»
queda en la línea del título.

## Cómo se abre el editor

`App.compose(carpeta, punto de partida, título)` crea la carpeta si hace falta y abre el editor de
escenas o el empaquetador con `host.work`. Para imagen y vídeo se elige antes el archivo: su ruta
viaja en `App.media` y el empaquetador la toma (`App.takeMedia()`, que la deja vacía) y la aplica
con `manifestEdit("media", {file})` tras crear.
Al cargar cualquier proyecto se anota en la lista de recientes.

## Límites

- Las tarjetas de «Empezar uno nuevo» son imágenes fijas o íconos, sin vista previa animada.
