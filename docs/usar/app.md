# La aplicación de escritorio

Una sección por pantalla, en el orden del menú de la izquierda. Los nombres son los que ves en la
aplicación en español. Para empezar desde cero, lee antes los [primeros pasos](LEEME.md).

## Teclas

La línea de abajo a la derecha muestra solo las teclas que sirven donde estás:

- **← → ↑ ↓** («Moverse»): dentro de un bloque (el menú, una fila, una lista).
- **Tab** («Siguiente parte»): pasa al bloque siguiente (menú, «ahora», acciones, página).
- **Intro** («Abrir»): hace lo principal del bloque; en el menú, abre la palabra elegida.
- **Esc** («Volver» o «Menú»): sube un nivel. Si no hay nivel que subir, lleva el foco al menú.
  En los editores no sale: sale la palabra de vuelta de arriba.
- El foco se ve con un anillo claro. Un botón que cambia de función en su sitio ignora el ratón
  medio segundo, para que un doble clic no haga lo que no pediste.

## Biblioteca

Para qué sirve: ver y poner los fondos que ya tienes.

- **Estante**: una fila de tapas abajo. Bajo la tapa encendida está su ficha con el nombre, el tipo,
  el tamaño y las acciones.
- **Poner de fondo** (Intro): lo pone enseguida. Con varias pantallas dice «Poner de fondo en …».
- **Opciones ›**: abre el panel del fondo (ver abajo).
- **Editar una copia…**: en una escena guardada como paquete; la copia es un proyecto nuevo y el
  original no cambia.
- **Clic derecho**, o la tecla de menú, sobre una tapa abre un menú con las mismas acciones.
- **Marcas en la tapa**: «En uso», o «En …» con el nombre de la pantalla; una nota si el archivo
  falta o está dañado, o si tiene destellos y están apagados.
- **Filtros** de la barra: Todos, Escenas, Imágenes, Vídeos, Web, De fuentes y Favoritos, cada uno
  con su número. «Más» abre Paquetes, Recursos y las etiquetas. El buscador dice «Buscar en
  Escenas» (o el filtro elegido) y acepta `tag:`, `kind:`, `fav:`, `provider:` y `sort:`.
- **Añadir fondos…**: abre el selector de archivos del escritorio y pregunta si copiarlos a la
  biblioteca o dejarlos donde están. «Escribir las rutas…» deja pegar una ruta por línea. Soltar
  archivos sobre la ventana los añade sin copiarlos.
- **Ver todos**: la colección en una rejilla. Primero van los fondos puestos ahora. «Ver el
  estante» vuelve.
- **Varios fondos**: Ctrl+clic, el círculo de la tapa o Espacio eligen. Mayús con una flecha elige
  un tramo y Ctrl+A elige todos. La línea de la selección ofrece «Rotar estos cada…», «Lista»,
  «Etiquetas» y «Más» (favorito, valoración, destellos, quitar). «Dejar de elegir» o Esc salen.
- **Teclas**: ← → mueven; ↑ sube a los filtros; ↓ abre la rejilla; Intro pone el fondo; Tab pasa a
  los botones de la ficha.
- Si el servicio de fondos no responde, la página lo dice y ofrece «Reiniciar el servicio».

### Opciones de un fondo

Ruta «Biblioteca › nombre» (Esc vuelve). Arriba, «Favorito» y «Poner de fondo». Contiene la vista
previa; «Opciones del autor» (lo que la escena deja ajustar); «Mis preajustes»; «Tuyo» (etiquetas y
listas) y «Más». «Reproducir igual» aparece si una escena pesada quedó quieta en su póster.
En «Más» están los permisos de esa escena, el sonido, el nombre, la valoración y
«Quitar de la biblioteca…», que pregunta antes.

## Explorar

Para qué sirve: instalar fondos de los repositorios.

- Cada tarjeta muestra un fondo. El botón principal es «Instalar», que pasa a «Poner de fondo»;
  «Instalar y poner de fondo» hace las dos cosas. También hay «Actualizar a …», «Detalles» y
  «Desinstalar…».
- **Detalles** dice autores, versión, permisos que pide y, si tienes una cuenta en el servidor del
  repositorio, deja votar a favor o en contra.
- **Repositorios…** abre la lista de repositorios; «Agregar un repositorio» (siempre visible en
  Explorar y al comienzo de esa lista) añade uno pegando su dirección. Si es privado, el cuadro pide
  una cuenta: «Usar la cuenta X» o «Iniciar sesión» (se aprueba en el navegador). Si el servidor usa
  su propio certificado, muestra su huella y «Confiar y agregar». Si pegas la dirección de un
  servidor, dice que es un servidor y nombra las direcciones de sus repositorios públicos (la lista
  completa está en la página «Explorar repositorios» de ese servidor, con «Abrir en Reiwand»). Si la firma de un
  repositorio cambia, la app avisa y no aplica nada. Un enlace `reiwand://add?url=…` abre el cuadro con la
  dirección puesta; nada se agrega hasta que pulses «Agregar». Los ejemplos incluidos no necesitan
  conexión.
- **Buscar novedades** y **Actualizar mis fondos** traen lo nuevo.
- **Más** reúne las herramientas para quien publica: «Subir un paquete…» y «Repositorio estático…».
- Si un repositorio falla, la tarjeta lo dice («No hay conexión con …», «La firma no coincide») y
  ofrece «Probar de nuevo» o «Ver los ejemplos». Nunca se instala algo cuya firma no coincide.

## Listas

Para qué sirve: grupos de fondos que se turnan.

- Filtro «Todas» y «En marcha», «Nueva lista…» y un estante de tapas con cuatro miniaturas.
- La ficha dice cuántos fondos tiene, cada cuánto cambia y si está detenida, con
  «Poner en marcha» y «Editar ›».
- **Editar**: Nombre; «Cambia cada» (10 min, 30 min, 1 h, Cada día u «Otro…»); Pantalla; los fondos
  con Quitar y el asa para ordenar; «Añadir fondos…» y «Añadir una carpeta…». Arriba, «Cambia en»,
  «Pausar la lista» y «Detener».
- **Más**: Búsqueda (una lista con búsqueda se llena sola), «Convertir en manual», «Seguir una
  carpeta…», las reglas, el Historial y «Quitar la lista» (con «Deshacer» en el aviso).
- Ctrl+↑ y Ctrl+↓ mueven el fondo elegido.

## Fuentes

Para qué sirve: traer fotos de internet.

- Cada fuente es una tarjeta con su estado y «N fotos, actualizado hace…». Botones: «Rotar estas
  fotos», «Actualizar» y «Detalles».
- «Añadir una fuente»: Bing, NASA y Wikimedia se añaden con un clic; las demás piden antes lo que
  necesitan (búsqueda, servidor, clave). Las claves nunca se vuelven a mostrar.
- «Más opciones» (Wallhaven): qué fotos y cómo buscarlas. Cada cambio se guarda al momento.
- En «Detalles», «Fotos guardadas» dice cuánto ocupan y ofrece «Borrar…»; «Quitar esta fuente»
  quita la fuente.
- Las fotos son de sus autores y valen como fondo, no para publicarlas.

## Crear

Para qué sirve: empezar o abrir un proyecto propio. Es para quien hace fondos; el resto de la
aplicación no lo necesita. Ofrece «Escena vacía», Partículas, «Paralaje por capas», «Reloj de
texto», Imagen, Vídeo, «Página web», Preajuste y «Paquete de recursos», más tus proyectos
recientes. Todo lo demás está en [Crear escenas](../crear/LEEME.md).

## Ajustes

Tres pestañas. El buscador de arriba filtra las filas de las tres. Las filas que abren algo llevan
a un lugar con ruta; Esc vuelve.

### Fondos

- «Qué se muestra»: «Contenido para adultos» (Oculto, Difuminado, Visible) y «Destellos»
  (Deshabilitados, Permitidos).
- «Tu biblioteca»: «Carpetas de la biblioteca» (cada una ofrece revisarla, rotar sus fotos o
  quitarla) y «Listas por pantalla».
- «Más»: atajos de teclado para cambiar de fondo, «Novedades de los repositorios» (solo cuando las
  pides, buscarlas solas o instalarlas solas), «Caché de miniaturas», «Mostrar el número de votos»
  y «Vigilar la biblioteca».

### Pantallas

- «Pantalla que ajustas» (con varias).
- «Calidad de las escenas»: Automática, Ahorrar batería o Máxima; los siete niveles están en
  «Más de …». Lo que fijas siempre se respeta.
- «Transición al cambiar de fondo» y «Menos movimiento» (Como el sistema, Siempre, Nunca).
- «Pausar»: pantalla completa, batería y bloqueo.
- «Más de …»: fluidez, color de relleno y HDR.
- «Disposiciones»: Usarla, Reglas automáticas, Cambiar el nombre y Eliminarla.

### Equipo

- «Modo privado» (ver más abajo).
- «Permisos de las escenas»: cada permiso de cada escena es una fila que lo retira con Intro.
- «Sonido de las escenas», «Lo que estás escuchando» y «Luces RGB» (con OpenRGB).
- «Avanzado»: «Tarjeta gráfica», HDR, «Calidad de la profundidad», Mantenimiento («Recargar los
  ajustes», «Revisar toda la biblioteca»), la versión y «Licencias».

## Cuentas

Para qué sirve: entrar a los repositorios privados y publicar. Todo es opcional y nada se conecta sin
que lo pidas. Las cuentas se vinculan a un repositorio y puedes tener varias; no hay sincronización
entre tus equipos.

- Al agregar un repositorio privado, el cuadro pide una cuenta: «Usar la cuenta X», «Usar otra cuenta»
  o «Iniciar sesión» (un código que se aprueba en el navegador).
- *Repositorios* dice qué cuenta usa cada repositorio.
- Al publicar eliges el repositorio de destino; si no tiene una cuenta vinculada, se pide ahí, sea
  público o privado.
- Las contraseñas no viajan entre equipos, salvo dentro de un archivo exportado y cifrado.

## Mover la configuración

Ajustes › Equipo › «Mover mi configuración» exporta a un archivo los repositorios, proveedores,
listas, favoritos, valoraciones, etiquetas, fondos instalados y ajustes de este equipo (los fondos
propios y las claves solo si lo marcas; las claves cifran el archivo con una contraseña), e importa
uno hecho en otro equipo: muestra qué trae, solo suma lo que marcas y deja unir o tener las dos
listas con el mismo nombre. Es el mismo archivo que usa el teléfono y `reiwand-ctl export` e `import`.

## Cambiar de fondo con vista previa

Desde la bandeja, un atajo o `reiwand-ctl next --preview` se abre una ventanita centrada en tu
pantalla, con el resto atenuado. El fondo elegido se ve grande en el centro y los vecinos, nítidos, a
los lados; debajo, su nombre y su posición. ← y →, la rueda o un clic en un vecino mueven la selección;
Intro o un clic en el central aplican y cierran; Esc o un clic fuera cierran sin cambiar nada. Más en
[cambiar de fondo con el teclado](cambiar-con-el-teclado.md).

## La bandeja

En Plasma, el ícono de la bandeja abre un panel con: lo que suena ahora («“…”, 2 de 5»),
Pausar o Reanudar, Siguiente, «Volumen general», «Silenciar todo» y, por pantalla, «Elegir…» y
«Cambiar con vista previa…». «Más» trae favorito, ajuste, filtro y quitar. «Abrir Reiwand» y
«Ajustes…» abren la aplicación. Si el servicio no corre, el panel lo dice.

## Preguntas de permisos

Una escena puede pedir el puntero, el sonido del equipo, clics y toques, lo que suena o la red.
Antes de usar algo, te pregunta una vez por escena: «“nombre” pide permiso para usar», con una
línea por cosa y «Permitir» o «Sin esto». La escena ya funciona sin los permisos; si algo queda
afuera, un aviso dice dónde cambiarlo. En Plasma la pregunta también puede llegar como notificación
del escritorio. Lo respondido se recuerda y se retira en Ajustes › Equipo › «Permisos de las escenas».

## Modo privado

En Ajustes › Equipo, primera fila. Corta toda la red: no se actualizan fuentes ni repositorios, no
se descargan carátulas, los fondos web solo cargan archivos de tu equipo y no se habla con tus
servidores. Lo que ya tienes sigue funcionando. Explorar, Fuentes y Repositorios
muestran «Modo privado: la red está cortada.» con el botón «Desactivar».
