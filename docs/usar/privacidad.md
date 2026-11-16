# Privacidad

- **Modo privado** (Ajustes › Equipo, o `reiwand-ctl private on`): corta toda la red del programa de
  una vez. Mientras está encendido no se actualizan fuentes ni repositorios, no se descarga nada, no
  se piden carátulas de internet, no se habla con tu servidor ni con tu teléfono, las luces de
  OpenRGB se apagan (hablan por TCP aunque estén en este equipo) y los fondos web solo cargan archivos
  de este equipo; tampoco se busca ningún nombre en la red. Lo que ya tienes sigue funcionando:
  biblioteca, ejemplos incluidos, escenas, videos y fondos web locales. Está apagado de fábrica y se
  recuerda al reiniciar. No existe ninguna comprobación de versión que cortar: el programa nunca la
  hace.
- Sin telemetría ni estadísticas de uso. El servicio solo se conecta a internet para los
  proveedores de fondos, los repositorios y el servidor propio que añadas tú. Al arrancar solo contacta con un
  proveedor si le toca su sincronización automática; la comprobación diaria de los repositorios
  (`repos.auto_refresh`, también en los ajustes del teléfono) empieza 10 minutos después. Un repositorio solo puede redirigir a su propia
  dirección y a las réplicas que declara en su raíz firmada.
- Los proveedores de fondos en línea (Bing, APOD, Wikimedia, Wallhaven, WebDAV/Nextcloud, Immich)
  solo se usan si los configuras. Los servicios públicos se usan siempre por HTTPS; todos llevan un
  User-Agent genérico (`reiwand/<versión>` y la dirección del proyecto, nada tuyo), sin cookies ni cabecera Referer, y las claves de API
  y contraseñas solo se envían al servidor configurado. Se guardan en el llavero del sistema
  (Secret Service) y en ningún otro sitio: ni en la configuración, ni en los registros, ni en la
  línea de órdenes, ni se sincronizan entre equipos (los proveedores y sus claves son de cada
  equipo). Un equipo sin llavero puede guardar todos los secretos (proveedores, scripts de escena y
  acceso al servidor) en un archivo privado (0600) que eliges tú sabiéndolo
  (`reiwand-ctl provider store file`, o el aviso de la página de proveedores): un aviso visible se
  queda mientras esté así, y se puede volver al llavero sin quitar nada
  (`reiwand-ctl provider store keyring`). La dirección de tu propio servidor puede usar `http://`
  sin cifrar, siempre permitido, con un aviso permanente de que la contraseña viaja sin cifrar.
- Los fondos web no tienen red salvo que la declaren y la permitas; lo que ven de ti (zona horaria,
  idioma, tarjeta gráfica y sus datos guardados) se avisa en la app, la bandeja, `reiwand-ctl` y el
  teléfono, y puedes ocultarlo por fondo. En el teléfono el WebView va con sus métricas y la
  navegación segura apagadas.
- Lo que suena en el reproductor multimedia solo se lee mientras un fondo visible con tu permiso lo
  usa, y nunca se guarda, se registra ni se envía; las carátulas de internet solo se descargan con
  `media.remote_covers`, apagado por defecto.
- La iluminación RGB está apagada por defecto; encendida, solo manda colores (nunca otra cosa) al
  servidor de OpenRGB de este equipo, o a la dirección que elijas con un aviso visible, y solo de los
  paquetes a los que se lo permites.
- El modelo de profundidad, el de relleno LaMa y ONNX Runtime solo se descargan si lo pides y aceptas su tamaño,
  licencia y servidores; llevan el mismo User-Agent genérico, sin cabecera Referer ni redirecciones
  de HTTPS a HTTP, se verifican con SHA-256 y los mapas se calculan en el equipo, en un proceso
  aparte. ONNX Runtime funciona con su telemetría desactivada: no guarda ningún identificador ni se
  conecta a nadie. En Android, ONNX Runtime se descarga junto con el modelo y solo después de que lo aceptes,
  comprobado con su SHA-256, y se usa con la telemetría apagada.
  `models.allow_download` se queda en cada equipo y, si la configuración no se puede leer, cuenta
  como desactivado.
- La configuración se escribe con permisos `0600` dentro de una carpeta `0700`: solo tu usuario
  puede leerla.
- La caché de descargas se guarda en una carpeta privada.
- La biblioteca y las copias importadas (`~/.local/share/reiwand`), las miniaturas y las
  entradas de paquetes (`~/.cache/reiwand`) se guardan con permisos `0600` en carpetas `0700`.
  Las miniaturas y el análisis de la biblioteca (tamaño, duración, color) los hace un proceso aparte
  con la prioridad más baja de CPU y disco. La vista del editor de imágenes se guarda como un
  archivo `0600` en la carpeta de ejecución de tu sesión (`XDG_RUNTIME_DIR`, o la carpeta temporal
  si no la hay) y se borra al dibujar la siguiente o al cerrar el editor.
- El servicio de systemd solo puede abrir conexiones de red (IPv4 e IPv6) para los proveedores y
  repositorios que añadas tú, y sus registros no incluyen rutas, nombres de archivo, servidores
  propios ni el modelo de GPU salvo con `RUST_LOG=debug`.
- Ninguna escena oye el sonido, sigue el puntero ni recibe clics sin tu permiso, que se da y se
  retira por escena desde la app, la bandeja o `reiwand-ctl input grant|revoke <escena>
  audio|pointer|clicks` mientras la escena se ve. Los clics, además, están apagados en todas las
  pantallas hasta que los enciendes en una (`reiwand-ctl input clicks on -o <pantalla>`, o en la app), y
  solo llegan los reales: los del compositor en Sway y demás escritorios con `wlr-layer-shell`
  (también los toques de una pantalla táctil), los de X11 que no manda otro programa con `SendEvent`
  y, en Plasma, los del escritorio (también con la vista de carpeta y sus iconos, que se quedan con
sus clics: el plugin de fondo los ve con un manejador que no los toma, así que un clic sobre un icono
o un widget llega además a la escena), que el plugin envía y el daemon solo
  acepta del intérprete de Plasma (ni de la app ni de `reiwand-ctl`) y, mientras el script de KWin
  informa del puntero, solo en la pantalla donde está. Ningún otro programa puede inyectar clics por
  D-Bus; el daemon no distingue el plugin de otro código que corra dentro de Plasma, como otro
  applet, y por eso lo acota en vez de impedirlo. El permiso queda atado al paquete que se veía: a la clave que firma un paquete
  instalado desde un repositorio (sus actualizaciones lo conservan) o al contenido exacto de
  cualquier otro (en la vista previa del editor, a la carpeta del borrador si la pide la app o
  `reiwand-ctl`, o a su contenido exacto si la pide otro programa), así que otro paquete con el mismo id no lo hereda, ni siquiera si se ve a la vez en
  otra pantalla (entonces `reiwand-ctl input grant` pide `-o` con la pantalla, y la app y la bandeja
  dan el permiso al paquete de la pantalla que preguntan); los permisos guardados por versiones
  anteriores, que solo tenían el id, se vuelven a preguntar. Si otro programa lo pide por D-Bus, el
  daemon no lo da: te pregunta con una notificación del escritorio de Reiwand que nombra la
  escena, solo cuenta tu respuesta en ella y solo la aplica a ese paquete (*Ahora no* tampoco quita
  el permiso a otros paquetes con el mismo id); si respondes *Ahora no*, la cierras o no contestas,
  esa pregunta no vuelve hasta diez minutos después, y la app, la bandeja y `reiwand-ctl input status`
  lo indican; pasado ese rato, un permiso que rechazaste se puede volver a pedir igual. Pedir un
  permiso que ya está dado no pregunta nada. Tampoco acepta posiciones del puntero de nadie salvo del
  script de KWin que él mismo carga (fuera del Flatpak, solo si lo envía un `kwin_wayland` o
  `kwin_x11` del sistema) y de `reiwand-ctl`. Cuando se aplica
  una escena que los pide sin permiso, desde donde sea (la app, la bandeja, una lista o `reiwand-ctl`),
  la app y la bandeja preguntan una vez por escena qué permitir, explicando qué se capta y que nada
  se guarda ni se envía, y sus ajustes muestran un aviso con el botón *Permitir* hasta que decides;
  *Ahora no* se recuerda y no vuelve a preguntar. `reiwand-ctl lib apply`, `playlist start`, `next` y
  `prev` imprimen la orden exacta para darlo. Aun con permiso, la captura del sonido (un stream de
  PipeWire llamado «Reiwand audio analysis», que puede encender el indicador de micrófono) y el
  seguimiento del puntero solo funcionan mientras se ve una escena que los usa, y se paran al pausar,
  tapar o bloquear la pantalla, aunque la pausa al tapar esté desactivada, y cuando nadie mira el
  stream de esa pantalla (en Plasma, cuando el escritorio no se ve). Si PipeWire se reinicia, la
  captura vuelve a conectarse sola, primero a los 0,5 s y luego esperando el doble cada vez, hasta
  30 s, sin despertar al equipo mientras está conectada. `playback.scene_capture = false`
  (*Dejar que las escenas reaccionen al sonido que suena* en los ajustes de la app, la bandeja y el
  teléfono, o `reiwand-ctl config playback.scene_capture false`) bloquea la captura para todas las
  escenas aunque tengan permiso. Del sonido solo se
  calculan bandas y niveles en memoria: no se guarda, no se registra y no sale del equipo. Del
  puntero solo se guarda en memoria la última
  posición, de los clics solo los que esperan al siguiente fotograma, y nunca se leen las teclas. Una imagen cuyo paralaje sigue al puntero lo usa con las
  mismas reglas, porque elegir esa fuente es el permiso. En Android, el permiso del micrófono se pide
  la primera vez que se activa *Reaccionar al sonido* en una escena, y solo se escucha mientras se ve
  una escena que lo usa y tiene ese permiso.
- Los scripts de las escenas se ejecutan en un proceso aparte por escena, sin archivos, red ni
  otros procesos, con límites de memoria, de CPU por fotograma, de temporizadores, de capas creadas,
  de líneas de consola y de valores compartidos, y nunca ven rutas, nombres de pantallas ni la
  batería.
- Con un servidor propio, los registros sincronizados salen cifrados con una clave que solo
  tienen tus equipos (en Android, guardada cifrada con una clave del almacén de claves del
  teléfono, que no sale de él); los identificadores de los paquetes se derivan de esa clave y el nombre de
  cada equipo solo viaja cifrado. La clave pasa a un equipo nuevo por el buzón del servidor,
  sellada para él, después de comparar un código de 6 cifras en las dos pantallas.
- Los paquetes se generan sin rutas, usuarios ni nombre del equipo de quien los compila.
- El servidor propio no guarda IP ni user agents en sus registros salvo que su administrador
  active el registro de abusos, que se borra solo; los datos sincronizados le llegan cifrados.
