# Servidor propio

`reiwand-server` publica un repositorio firmado de paquetes, un catálogo web sin JavaScript, cuentas
con subidas y moderación, y la sincronización de la biblioteca, las listas y los ajustes entre
dispositivos, cifrada de extremo a extremo por defecto. Es un solo binario en un contenedor sin
shell, que corre como root por defecto (con una línea `user:` en `compose.yaml` puede correr como otro usuario); todo vive en la carpeta
`./data`, montada como `/data`, y la copia de seguridad es copiar esa carpeta. No envía telemetría ni se conecta a terceros.

Se levanta de dos maneras, con Docker o sin él, y deja el mismo servidor. Para usarlo no hace falta un
navegador en el equipo del servidor: todo lo que pide la web de administración tiene su orden de
terminal, más abajo.

**Con Docker**, con la imagen publicada:

```sh
cp .env.sample .env                           # editar REIWAND_PUBLIC_URL, la única variable obligatoria
docker compose up -d                          # baja la imagen publicada y la arranca
docker compose exec reiwand reiwand-server repo init --name "Fondos de casa"
docker compose exec reiwand reiwand-server setup-token   # token de un solo uso del primer administrador
```

Los datos quedan en `./data`. En el resto de la sección, `reiwand-server …` es
`docker compose exec reiwand reiwand-server …` si usas Docker.

**Sin Docker**, con el binario `reiwand-server`, que se compila con Rust (la versión mínima está en
`Cargo.toml`) en una copia de este repositorio; la imagen de Docker se construye con solo eso, sin Qt
ni PipeWire:

```sh
cargo build --release -p reiwand-server          # deja target/release/reiwand-server
export REIWAND_DATA=$HOME/fondos-servidor        # sin esto usa /data
export REIWAND_PUBLIC_URL=http://127.0.0.1:8080  # la dirección con la que lo escribirán los equipos
export REIWAND_LISTEN=127.0.0.1:8080             # sin esto escucha en todas las interfaces (0.0.0.0:8080)
target/release/reiwand-server repo init --name "Fondos de casa"
target/release/reiwand-server serve              # queda en primer plano; paralo con Ctrl-C
```

Mientras no hay ningún administrador, `serve` escribe en su registro una línea como `no admin account
yet: open http://127.0.0.1:8080/setup and enter the one-time setup token reiwand_s_…` (con Docker,
`docker compose logs reiwand | grep setup`); `reiwand-server setup-token` da otro token en cualquier
momento. Con un navegador se abre `<REIWAND_PUBLIC_URL>/setup`, se pega el token y se elige el nombre y
una contraseña de 10 caracteres o más. Sin navegador, la misma página es un formulario que se envía
con `curl`, y la respuesta `303` dice que el administrador ya existe; `galleta` guarda su sesión para
los pasos siguientes (`$REIWAND_PUBLIC_URL` es la dirección del servidor; si no la tienes exportada, escríbela, por
ejemplo `http://localhost:8080` con Docker):

```sh
curl -sS -o /dev/null -w '%{http_code}\n' -c galleta --data-urlencode "token=<token>" \
  -d handle=admin --data-urlencode "password=<contraseña de 10 caracteres o más>" \
  "$REIWAND_PUBLIC_URL/setup"
curl -sS -c galleta -o /dev/null -d handle=admin --data-urlencode "password=<contraseña>" \
  "$REIWAND_PUBLIC_URL/login"                    # una sesión nueva, más adelante
curl -sS -b galleta -H 'content-type: application/json' -d '{"role": "user"}' \
  "$REIWAND_PUBLIC_URL/api/v1/admin/invites"     # una invitación para otra cuenta: imprime su code y su url
```

`repo init` crea las claves del repositorio y muestra una sola vez la clave raíz, que hay que guardar
fuera del servidor; si responde `error: a repository already exists`, ese servidor ya tiene su
repositorio y no hay nada que hacer (`reiwand-server repo status` dice su estado y su dirección). El servidor solo conserva la clave en línea, con la
que vuelve a firmar el índice cada día (cada firma caduca a los 7). Desde *Administración* (o con la
API, como arriba) se crean invitaciones y desde *Cuenta*,
tokens, dispositivos enlazados con un código y subidas de paquetes `.wpak` de las dos versiones del
manifiesto. El servidor acepta los paquetes para adultos que lo declaran en su manifiesto: llevan
la etiqueta `mature` en el catálogo, en la búsqueda (`content=mature`, `facet=mature`) y en la API, y
la web difumina sus miniaturas hasta que eliges verlas, una por paquete y sin JavaScript (la web no
guarda la elección, así que vuelve a difuminarlas en cada visita). El
catálogo filtra por clasificación y por faceta y avisa en la página de un paquete fotosensible.

Los moderadores pueden ocultar un paquete sin retirarlo, con un motivo que ve su autor: sale del
catálogo y de la búsqueda, pero quien lo tiene instalado lo conserva y recibe sus actualizaciones.
Se hace desde *Moderación* en la web, con la API o con
`reiwand-ctl moderate --server https://fondos.example.org hide org.example.fondo --reason "…"`
(`unhide` lo vuelve a mostrar y `hidden` los lista), con un token personal con el permiso `mod` en
`REIWAND_SERVER_TOKEN`.

Con `repo init --online-authors`, cada cuenta registra en *Cuenta* las claves con las que firma sus
paquetes, sin pasar por la clave raíz. Cada cliente se fía de la clave que firmó la primera versión
que instaló de cada paquete y solo acepta otra si la anterior la autorizó: `reiwand-ctl pack rotate-key
vieja.key nueva.pub --author u<id>` firma ese traspaso, que se guarda en *Cuenta*. Así, quien robe la
clave en línea del servidor puede añadir autores y paquetes nuevos, pero no actualizar un paquete
ya instalado. Los autores que lista la raíz tienen prioridad.

En el editor, el diálogo de construir tiene *Claves…* (crea el par de claves de autor en la carpeta
de claves de la aplicación, con permiso solo para ti y contraseña opcional, nunca muestra una clave
secreta, dice cuál de los dos archivos es el secreto y cuál el público, pregunta antes de reemplazar un par y firma el traspaso de un autor en línea a otra clave) y
*Revisar el paquete…* (cada hallazgo con su nivel, el archivo y un salto al campo, dónde se reproduce
el paquete y si la firma de la carpeta o de un `.wpak` coincide con la clave pública del autor). La licencia
del paquete se elige en una lista de diez, cada una con su explicación, y se puede escribir otra. Desde
la terminal: `reiwand-ctl pack check CARPETA`, `pack validate CARPETA|ARCHIVO --key autor.pub`,
`repo keygen autor` (la contraseña se pregunta, o se toma de `REIWAND_KEY_PASSWORD` sin terminal) y
`pack rotate-key`; `pack build` no arma un paquete con una firma de otra versión del manifiesto:
pide `pack sign` otra vez.

*Editar una copia…*, en *Detalles* de cada elemento de la biblioteca (al final del panel) y, cuando la
biblioteca tiene paquetes guardados como archivo `.wpak`, en la lista de *Escenas*, abre
una copia de cualquier tipo de paquete (escenas, páginas, imágenes y vídeos) en una carpeta nueva y
con un id nuevo, así que el instalado nunca cambia, y conserva la licencia y los créditos. El diálogo
muestra la licencia del original, qué permite y qué pide (por ejemplo, poner tu nombre en los
autores). Si la licencia no permite obras derivadas (por ejemplo `CC-BY-ND-4.0`) no la abre, salvo que
marques la casilla que confirma que el paquete es tuyo o que su autor lo permite, y que solo sale en
ese caso. Desde la terminal: `reiwand-ctl scene open CARPETA --from
paquete.wpak [--id ID] [--mine] [--facts]`.

`compose.yaml` baja `ghcr.io/reiwand/reiwand-server` con la etiqueta de `REIWAND_VERSION` (`latest` si no
se fija). Cada versión publica esa imagen (`linux/amd64`, con las etiquetas `<versión>` y `latest`),
firmada con cosign sin clave propia como los archivos de *Releases*. Lleva adjuntos los SBOM SPDX de
BuildKit (los paquetes de la imagen y de la etapa de compilación) y su procedencia, y una atestación
SPDX firmada con las dependencias de Rust de las fuentes con las que se construyó. Para fijar una
imagen concreta se toma su resumen, se comprueban la firma y la atestación y se pone el resumen
en lugar de la etiqueta (`image: ghcr.io/reiwand/reiwand-server@sha256:<resumen>`):

```sh
IMAGE=ghcr.io/reiwand/reiwand-server
DIGEST=$(docker buildx imagetools inspect "$IMAGE:<versión>" --format '{{ .Manifest.Digest }}')
ID=https://github.com/reiwand/reiwand/.github/workflows/ci.yml@refs/heads/master
ISSUER=https://token.actions.githubusercontent.com
cosign verify "$IMAGE@$DIGEST" --certificate-identity "$ID" --certificate-oidc-issuer "$ISSUER"
cosign verify-attestation "$IMAGE@$DIGEST" --type spdxjson \
  --certificate-identity "$ID" --certificate-oidc-issuer "$ISSUER" > reiwand-server.sources.spdx.dsse
docker buildx imagetools inspect "$IMAGE@$DIGEST" --format '{{ json .SBOM }}' > reiwand-server.sbom.json
```

Para construir la imagen desde este repositorio en lugar de bajarla, se suma el archivo de
construcción; el servidor así construido se identifica con la versión de `Cargo.toml` y el sufijo
`-local`, y con `REIWAND_VERSION` en el entorno muestra la de una versión publicada:

```sh
docker compose -f compose.yaml -f deploy/compose.build.yaml up -d --build
```

La configuración va en variables de entorno, en el `.env` junto a `compose.yaml` (`.env.sample` las
lista todas con su valor por defecto); `VARIABLE_FILE` lee el valor de un archivo, para los secretos.
`REIWAND_VERSION` y `REIWAND_BIND` son solo de compose:

| Variable | Por defecto | Uso |
|----------|-------------|-----|
| `REIWAND_PUBLIC_URL` | obligatoria en compose | URL pública del servidor |
| `REIWAND_VERSION` | `latest` | Etiqueta de la imagen que baja compose |
| `REIWAND_BIND` | `127.0.0.1:8080` | Dirección y puerto que publica compose |
| `REIWAND_REGISTRATION` | `invite` | `closed`, `invite` u `open` |
| `REIWAND_AUTO_PUBLISH` | `trusted` | Publicar sin revisión: `never`, `trusted` o `all` |
| `REIWAND_LICENSES` | `CC0-1.0,CC-BY-4.0,CC-BY-SA-4.0` | Licencias aceptadas |
| `REIWAND_LEGAL_OPERATOR`, `REIWAND_LEGAL_CONTACT` | vacías | Responsable y contacto de las páginas legales |
| `REIWAND_LOG_CLIENTS` | `off` | IP en un registro de abusos: `off`, `prefix` o `full`; se borra a los `REIWAND_LOG_CLIENTS_DAYS` días (7) |
| `REIWAND_NOTICE_DAYS` | `180` | Días que un aviso legal guarda el nombre, el correo y la firma de quien lo envía |
| `REIWAND_TAKEDOWN_DAYS` | `90` | Días que se guarda fuera de la vista pública un paquete retirado, como prueba y para una contranotificación |
| `REIWAND_TLS` | `off` | `self-signed`: sirve HTTPS con un certificado propio, que los clientes aceptan solo por su huella fijada |
| `REIWAND_ADVERTISE` | `false` | Anunciar el servidor en la red local por mDNS (`_reiwand._tcp`) con un nombre al azar en cada arranque |

La lista completa de variables y la API HTTP están en
[`docs/specs/server.md`](../specs/server.md).

## Enlazar los equipos

El servicio de cada equipo se enlaza con el servidor desde *Cuenta* en la app o con la CLI:

```sh
reiwand-ctl server add https://fondos.example.org
reiwand-ctl server add https://192.168.1.10:8443 --fingerprint <huella>   # un servidor de casa con su propio certificado
reiwand-ctl server link --name Escritorio          # muestra un código para /link o para:
reiwand-ctl server connect --name Portátil         # solo conectar: repositorios, catálogo, instalar y votos, sin sincronizar
reiwand-ctl server syncing on                       # … más tarde, enciende la sincronización; `off` la apaga y sigue conectado
reiwand-ctl server approve BCDF-GHJK                # … en un equipo ya enlazado, tras ver quién lo pide y con la contraseña
reiwand-ctl server join "Device 1" --wait           # equipo nuevo de una cuenta cifrada: muestra sus cifras y pide las del otro
reiwand-ctl server share "Device 2"                 # en el equipo con la clave: muestra sus cifras y pide las del otro
reiwand-ctl server status                           # el estado, los equipos de la cuenta y por qué falló el último enlace
reiwand-ctl server revoke "Device 3"                # quita un equipo de la cuenta (también en Cuenta > Quitar equipo…)
reiwand-ctl provider store file                     # sin llavero: guarda todos los secretos en un archivo privado, tras el aviso
reiwand-ctl server rotate                           # clave nueva; los demás equipos la reciben solos
reiwand-ctl server reset                            # sin ningún equipo con la clave: rehace los datos desde este
reiwand-ctl server upload fondo.wpak                # sube un paquete con el token del equipo
reiwand-ctl server upload fondo.wpak --unlisted     # … oculto: fuera del catálogo, instalable con su enlace
reiwand-ctl server packages                         # tus subidas, con su estado y visibilidad
reiwand-ctl server uploads                          # las sesiones de subida (en curso o terminadas); --cancel ID cancela una
reiwand-ctl server author-key list                  # las claves con las que firmas tus paquetes en línea
reiwand-ctl server author-key add autor.pub         # registra una clave; `rotate`, `remove` la cambian o la quitan
reiwand-ctl server visibility org.example.fondo public   # cambia la visibilidad de un paquete o de una versión
reiwand-ctl server repo 1a2b3c4d                    # añade un repositorio que otro equipo sincronizó, tras revisarlo
reiwand-ctl server files share ./fondos             # sube tus propios fondos, cifrados; en otro equipo:
reiwand-ctl server files ls                         # … los lista y `server files fetch <id>` descarga uno
reiwand-ctl server away                             # lo que otros equipos sincronizaron y solo existe en ellos
reiwand-ctl server escrow                           # guarda la clave en el servidor, cerrada con una frase
reiwand-ctl server restore                          # equipo nuevo sin ningún otro: recupera la clave con la frase
```

**El primer equipo, paso a paso**, también sin navegador (la cuenta del administrador ya existe y
`galleta` es su sesión, de la sección anterior):

```sh
reiwand-ctl server add http://127.0.0.1:8080              # la dirección del servidor
reiwand-ctl provider store file                           # solo sin llavero del sistema, ver abajo
reiwand-ctl server link --name Escritorio --wait          # imprime code = ABCD-EFGH y url = …/link, y espera
curl -sS -b galleta -H 'content-type: application/json' \
  -d '{"user_code": "ABCD-EFGH", "approve": true}' \
  http://127.0.0.1:8080/api/v1/auth/device/approve     # {"approved":true}; con navegador, abre la url y escribe el código
reiwand-ctl server status                                 # state = ready, la cuenta, este equipo y los de la cuenta
```

`server add` acepta `https://`, y `http://` solo para este mismo equipo: un servidor que está en otro
equipo necesita HTTPS (más abajo, con su propio certificado o con un proxy). Enlazar necesita un
llavero del sistema donde guardar el acceso (GNOME Keyring, KWallet…): sin él, `server link` falla al
momento y lo dice, sin dejar un equipo fantasma en la cuenta, y `provider store file` lo guarda en un
archivo privado tras un aviso (se puede volver al llavero con `provider store keyring`). Los equipos siguientes se aprueban desde uno
ya enlazado con `server approve CÓDIGO`, que pide la contraseña de la cuenta.

Un equipo puede quedar **conectado sin sincronizar** (`server connect`): usa los repositorios de la
cuenta, el catálogo, instala y vota, pero no envía ni recibe nada de la biblioteca, y su estado dice
«conectado, sin sincronizar», que es normal y no un error. `server syncing on` enciende la
sincronización después (con la clave de otro equipo, como `server join`); `server syncing off` la
apaga y el equipo sigue conectado, con su clave guardada. `server link` hace los dos pasos juntos.
Apagarla no borra nada: el equipo conserva la clave de la cuenta y los registros que ya recibió, y
en el servidor quedan los datos sincronizados; al volver a encenderla sigue desde ahí sin otro
emparejamiento. En el teléfono se apaga al instante; en el escritorio (la fila *Sincronizar mi
biblioteca* de *Cuenta* o `server syncing off`) es un trabajo corto que termina enseguida.

La sincronización mueve registros (títulos, etiquetas, valoraciones, favoritos y ediciones de la
biblioteca, listas, perfiles, preajustes y ajustes), no archivos de paquetes ni lo que está puesto:
qué fondo hay en cada pantalla y cómo se coloca queda en cada equipo, y un paquete que se nombra en
un registro se obtiene de un repositorio. Lo que existe solo en un equipo
(por ejemplo un preajuste hecho a mano y nunca publicado) no aparece en los demás, y lo local no se
sincroniza solo. Cuando un registro sincronizado nombra algo que este equipo no tiene y que ningún
repositorio conocido ofrece, la biblioteca, la lista que lo nombra y la pantalla en la que se aplicó
muestran una nota («existe solo en otro equipo; publicarlo en el servidor o en un repositorio lo hace
disponible aquí»), en la aplicación, en el teléfono y en `reiwand-ctl` (`lib search`, `playlist show`,
`state` y `server away`). No se descarga nada ni se reintenta en bucle: la nota desaparece en la
siguiente sincronización, cuando el elemento llega.

Para pasar un archivo a otro equipo, elige los fondos en la biblioteca y pulsa *Enviar los archivos a mis
otros equipos* (o `reiwand-ctl server files share`): se suben cifrados, y el otro equipo los ofrece en su
biblioteca y en *Cuenta* con un botón *Descargar* (o `reiwand-ctl server files fetch`) cuando termina su
siguiente sincronización. Nada se descarga solo.

Al subir un paquete eliges si es público u oculto (en el diálogo de subida de *Repositorios*, en la
web del servidor o con `--unlisted`): uno oculto se publica y se firma igual, pero el catálogo y la
búsqueda no lo muestran, y quien tenga su enlace o su id lo ve y lo instala. Oculto no es privado:
sigue en los archivos firmados del repositorio, que cualquiera puede descargar y leer, con su título y
su descripción; lo privado va en la sincronización o en un repositorio propio. *Cuenta* lista tus
subidas y te deja cambiar la visibilidad de cada versión, ver y cancelar las sesiones de subida
que quedaron a medias, y registrar, cambiar (con la firma de `pack rotate-key`) y quitar las claves de
autor en línea. En un repositorio estático, `reiwand-ctl repo
add --unlisted`, `repo unlist` y `repo list-again` hacen lo mismo antes de `repo publish`, y la pestaña
*Repositorio estático* de *Repositorios* abre la carpeta del repositorio y añade paquetes (también
ocultos), oculta o vuelve a mostrar un paquete o una versión y destaca o deja de destacar, con la misma
confirmación, y *Publicar…* firma el índice con un archivo de clave en línea (con su contraseña, si
la tiene) y los días hasta que caduca, como `reiwand-ctl repo publish`. La misma pestaña revoca un
paquete o una versión con su motivo y un mensaje (`repo revoke`) y retira la revocación (`repo
unrevoke`), y *Claves de autor…* revoca una clave de autor por su huella (`repo revoke-key`).
*Repositorio nuevo…* crea uno en una carpeta vacía con su nombre, las claves raíz y en línea (archivos
`.pub`), cuántas claves raíz deben firmar, un autor con sus claves y la política (`repo init`); *Raíz…*
muestra la versión de la raíz, cuántas claves la firmaron y la consulta que la ancla, la firma con un
archivo de clave raíz y su contraseña (`repo sign-root`) y *Rotar la raíz…* escribe la siguiente con
otras claves, autores quitados o añadidos y otra política (`repo rotate-root`). Las claves que das para
un autor reemplazan las que tenía, así una clave filtrada deja de valer (`--author ID=PUB`, repetible
para varias claves); la raíz nueva dura lo mismo que la anterior salvo que digas otros días, conserva
las réplicas salvo que des otras con `--mirror`, y `--no-mirrors` (o vaciar el campo en el diálogo) las
quita todas. Cada cambio del catálogo
trabaja fuera de la ventana, uno detrás de otro, y mientras dura los demás cambios y *Abrir* esperan
desactivados. En el teléfono no hay herramientas de autor: solo se usan repositorios (añadir, explorar, instalar, actualizar, quitar).
Cada paso deja los botones desactivados hasta que terminan todos los que estén en marcha.

En el catálogo de *Repositorios* (y en *Explorar* del teléfono) los paquetes que destaca el operador
del repositorio salen primero, en su propia sección (los firma en el repositorio con `reiwand-ctl repo feature REPO ID --note …` o la pestaña *Repositorio estático* de la app de escritorio, o desde
`/admin/featured` del servidor), y, con un servidor enlazado, cada paquete del repositorio de ese servidor muestra sus votos a favor y
en contra, con tu voto marcado, y botones para votar (`reiwand-ctl remote vote ID up|down|none`, `remote votes ID…`,
también como `REPO/ID`); los paquetes de otros repositorios no muestran votos y sus ids nunca se
envían al servidor. El
servidor cuenta un voto por cuenta y paquete, guarda solo los recuentos y tu propio voto, nunca una
lista de quién votó ni de quién instaló qué. El ajuste *Mostrar los votos del servidor en el
catálogo* (`repos.vote_counts`, también en los ajustes del teléfono) deja de pedir y mostrar los
recuentos. Tus valoraciones locales de la
biblioteca siguen siendo privadas y se sincronizan cifradas.

Aprueba solo un código que muestre la pantalla de tu propio equipo nuevo: quien te envía un código
intenta entrar en tu cuenta. Aprobar desde un equipo pide la contraseña de la cuenta, porque los
tokens de los equipos ya no gestionan dispositivos: quien robe uno no puede añadir equipos ni
revocar los demás. Las dos pantallas muestran el mismo número de seis cifras solo si
nadie se ha colado entre los equipos, y en cada uno escribes las cifras del otro: la clave no sale
ni se guarda hasta que coinciden en los dos; cada intento da una única oportunidad a quien lo
intente. Al revocar un equipo, otro cambia la clave de sincronización y vuelve a cifrar todo, así
que el equipo revocado ya no puede leer lo nuevo; la clave nueva pasa solo entre equipos que
compararon sus cifras, y los demás vuelven a unirse. Cambiar la contraseña en la web cierra las
demás sesiones y borra los tokens; los equipos siguen enlazados salvo que elijas cerrarlos también.

Se sincronizan los títulos, etiquetas, valoraciones, favoritos y ediciones de las imágenes de la
biblioteca (cada parte de la edición por separado, así que lo que cambian dos equipos en partes
distintas se suma, y una parte de más de 64 KiB conserva la última copia enviada), las listas, los
perfiles, los preajustes y los ajustes (salvo el volumen, la caché, el paralaje por
software y el permiso para descargar modelos, que se quedan en cada equipo), cifrados de extremo a
extremo; sin cifrar solo con `--plaintext`, que avisa antes. El token y la clave de sincronización
viven en el llavero del sistema, y enlazar necesita uno (GNOME Keyring, KWallet…): sin llavero, *Cuenta* lo
dice con palabras claras y ofrece, con un aviso, guardarlos en un archivo privado de la carpeta de datos
(`reiwand-ctl provider store file`, el mismo archivo y la misma elección para los secretos de los
proveedores); quien pueda leer tus archivos podría usarlos, y un aviso visible se queda mientras esté así. Un intento de enlace que
falla no deja equipos fantasma en la cuenta. *Cuenta* lista los equipos y quita cualquiera (*Quitar
equipo…*, o `reiwand-ctl server revoke`); un equipo quitado, o de una cuenta borrada, muestra en toda la
app un aviso de que el servidor ya no lo acepta; el servidor ve los equipos como `Device 1`, `Device 2`… y el
nombre real viaja cifrado. Al enlazar se añade también el repositorio del servidor.

Los repositorios que añade un equipo llegan a los demás como propuesta: cada equipo los añade solo
cuando lo confirmas, con su huella. Las claves de los proveedores viajan solo si lo eliges para
cada proveedor, solo en cuentas cifradas, y en cada equipo se guardan solo en su llavero. Tus
propios fondos (no los de un repositorio) suben cifrados por partes si eliges el archivo o su
carpeta; si se corta, la subida sigue donde se quedó, el mismo archivo se guarda una vez y los
demás equipos lo descargan cuando lo pides. Si dos equipos cambian una lista de formas que no se
pueden juntar, se conservan las dos: la otra queda como «Lista (conflicto)». Lo que una versión
más antigua no entiende se le devuelve tal cual, y el servidor recuerda la versión de formato más
nueva que escribió en la cuenta: un equipo con una versión anterior deja de sincronizar y pide que
lo actualices, en lugar de subir nombres viejos o registros sin los campos que no conoce.

Borrar un elemento de la biblioteca en un equipo lo borra también en los demás (desinstalando el
paquete), sin tocar sus archivos; un elemento de una carpeta vigilada cuyo archivo sigue ahí se
queda, pero no vuelve a los demás. Los borrados se recuerdan 90 días y luego el servidor los purga.
Si un cambio choca tres veces seguidas o la sincronización falla con cambios sin subir, se
reintenta a los 30 s, 1, 2, 4 y 8 minutos y después cada 15, sin preguntar al servidor si no ha
cambiado nada. Si un archivo propio no se puede subir (cuota llena o archivo desaparecido), los
demás se suben igual y el error nombra cada uno que falló.

Los nombres de los elementos en el servidor son hashes con una clave de nombres que no cambia al
rotar la clave de sincronización: un equipo que la tuvo, aunque se haya revocado, y tenga el mismo
archivo puede saber que tu cuenta lo guarda. La app, `reiwand-ctl server --help` y el teléfono lo
explican.

Si temes perder todos los equipos, `server escrow` guarda la clave en el servidor cerrada con una
frase (Argon2id, 256 MiB). Quien consiga una copia del servidor puede probar frases sin límite: usa
varias palabras sin relación que no uses en ningún otro sitio. Tras revocar un equipo la frase deja
de valer: la app, `reiwand-ctl server status` y el teléfono lo dicen, y hay que guardar otra nueva y
distinta, porque quien conocía la anterior podría abrir lo que se cierre con ella; la anterior se
rechaza.

Copias de seguridad: `docker compose exec reiwand reiwand-server backup` deja en
`/data/backups/<fecha>` una instantánea coherente de la base de datos (`VACUUM INTO`) con los
paquetes, miniaturas y archivos privados que nombra, las claves y el repositorio, con la suma SHA-256 de cada archivo. Para
restaurar, con el servidor parado:

```sh
docker compose stop reiwand                                   # las copias están en ./data/backups
docker compose run --rm reiwand restore /data/backups/<fecha> --force
docker compose up -d
```

La restauración comprueba las sumas y la integridad de la base de datos, firma un índice nuevo
para que los clientes no vean una versión anterior y hace que cada dispositivo vuelva a descargar
lo sincronizado.

En una red de casa sin dominio, el servidor puede servir HTTPS él mismo con un certificado propio
y anunciarse en la red local para que el teléfono lo encuentre:

```sh
REIWAND_TLS=self-signed REIWAND_ADVERTISE=true REIWAND_LISTEN=0.0.0.0:8443 \
  REIWAND_PUBLIC_URL=https://192.168.1.10:8443 reiwand-server serve
reiwand-server fingerprint                    # la huella SHA-256 del certificado, también en el registro
reiwand-ctl server add https://192.168.1.10:8443 --fingerprint <huella>
```

El certificado se crea en el primer arranque en `/data/tls` y se conserva. Los equipos y el
teléfono lo aceptan solo por su huella, que se compara una vez y queda fijada: sin ella, `server
add` falla y dice la huella que mostró el servidor; en la app se escribe en *Añadir servidor*, en
el teléfono se confirma en un aviso, y `reiwand-ctl server status` y la app la muestran. El anuncio
(`_reiwand._tcp` por mDNS) solo dice que hay un servidor de Reiwand, con un nombre al azar que
cambia en cada arranque, y nada de la cuenta ni de los equipos. Es la misma sincronización y el
mismo cifrado de extremo a extremo que con un servidor público, solo que sin salir de la red de
casa.

Con dominio propio, cualquier proxy inverso con TLS sirve: el servidor escucha en `REIWAND_BIND`
(`127.0.0.1:8080` por defecto), el proxy termina el HTTPS y reenvía las peticiones, y `REIWAND_PUBLIC_URL`
lleva la dirección pública con `https://`. El servidor toma la dirección del cliente de
`X-Forwarded-For` cuando el proxy corre en el mismo equipo (también con la imagen de `compose.yaml`);
un proxy en otro contenedor o en otra máquina se nombra en `REIWAND_TRUSTED_PROXIES`. Un equipo de la
red de casa que habla directo con el servidor no puede hacerse pasar por otra dirección.
[`deploy/nginx/reiwand.conf`](../../deploy/nginx/reiwand.conf) es un ejemplo para Nginx:

```sh
sudo cp deploy/nginx/reiwand.conf /etc/nginx/conf.d/fondos.conf   # cambiar el nombre y los certificados
sudo nginx -t && sudo systemctl reload nginx
```

El ejemplo atiende `fondos.example.org` en el 443 (y redirige el 80), con las rutas de los
certificados como marcadores, `client_max_body_size 256m` (el tamaño máximo de paquete por defecto,
`REIWAND_PACKAGE_MAX_MIB`: súbelo a la par si cambias esa variable), HTTP/1.1 con
`Connection ""` hacia el servidor y sin guardar las subidas en disco. Los avisos en vivo de la
sincronización (`/api/v1/sync/events`, eventos del servidor, no WebSockets) van en un bloque aparte
sin búfer y con un tiempo de espera largo.

Los paquetes de escena (`docs/specs/scene.md`) se importan y se aplican como cualquier otro
elemento de la biblioteca. El daemon los dibuja en la GPU solo cuando algo visible cambia, con
menos fotogramas y menos calidad con batería o con render por software; las escenas con
`"clock": { "max_fps": "display" }`, como las de ejemplo, siguen la frecuencia de la pantalla:

```sh
reiwand-ctl prop ls -o DP-1                     # propiedades de la escena que muestra DP-1
reiwand-ctl prop set tint '#ff8800' -o DP-1     # se aplica al momento y se guarda en config.toml
reiwand-ctl prop set volume 60 -o DP-1          # volumen propio de una escena con sonido
reiwand-ctl prop press again -o DP-1            # pulsa un botón de la escena
reiwand-ctl prop share -o DP-1                  # los mismos valores en todas las pantallas (--own: solo esta)
reiwand-ctl prop preset load calma -o DP-1 && reiwand-ctl prop reset -o DP-1
reiwand-ctl prop reset                          # sin -o: los valores de todas las pantallas que muestran la escena
reiwand-ctl prop preset save Noche -o DP-1      # guarda los valores actuales como preajuste propio
reiwand-ctl prop preset export Noche -f noche.json -o DP-1
reiwand-ctl prop preset import noche.json --save Noche -o HDMI-A-1
reiwand-ctl scene bench escena.wpak --size 1920x1080 --frames 300   # coste por fotograma, sin daemon, en la tarjeta de render.gpu
reiwand-ctl scene bench escena.wpak --screens 2   # carga, primer fotograma y memoria con dos pantallas
reiwand-ctl scene bench escena.wpak --size 1440x3040 --quality q3 --passes --shot q3.png   # cada pasada y la imagen del último fotograma
reiwand-ctl scene bench escena.wpak --quality auto --paced 30   # 30 s en tiempo real con el nivel automático del teléfono
REIWAND_ADB_SERIAL=<serie> scripts/phone-bench.sh build && REIWAND_ADB_SERIAL=<serie> scripts/phone-bench.sh copy escena.wpak
REIWAND_ADB_SERIAL=<serie> scripts/phone-bench.sh run escena.wpak --level q3 --passes   # el mismo banco en el teléfono, en JSON
REIWAND_ADB_SERIAL=<serie> scripts/phone-bench.sh run escena.wpak --level auto --paced 30 --pace smooth   # el automático sin bajar el ritmo
REIWAND_ADB_SERIAL=<serie> scripts/phone-bench.sh costs   # lo que cuesta la etapa 3D en el teléfono: unidad, modelos y mapas de sombras
```

En la página *Ajustes* de la app, el botón *Ajustes de la escena* (o *Ajustes del vídeo*) de cada
pantalla cambia sus propiedades al momento, aplica sus preajustes, guarda, renombra y borra los
propios, copia y pega los ajustes o los exporta e importa como un archivo pequeño, decide si la
pantalla comparte sus valores con las demás, da o retira el permiso de sonido, de puntero y de
clics y enciende o apaga los clics de esa pantalla. Los preajustes propios son del paquete (siguen tras actualizarlo) y viajan con la sincronización.

Los autores describen sus propiedades en el manifiesto (`docs/specs/scene.md`): colores con
transparencia, deslizadores con paso, decimales, unidades, escala logarítmica y casilla para
escribir el número, listas con iconos y etiquetas traducidas, textos y textos secretos (que se
guardan en el llavero del sistema y nunca llegan a `config.toml`, a los registros ni a la
sincronización), separadores, etiquetas y grupos plegables, botones que lanzan eventos de la escena,
y archivos y carpetas que elige el usuario con el selector de archivos, también vídeos, que
sustituyen una imagen, un vídeo, una máscara o la textura de unas partículas del paquete sin
reiniciar la escena. Cada propiedad puede mostrarse o activarse según las demás con condiciones
como `mood == 2 || speed > 1.5`, de un lenguaje pequeño que se comprueba al validar el paquete y
que no ejecuta código. Los scripts nunca ven las rutas de los archivos elegidos. Una carpeta solo
se lee mientras la escena se ve, puede incluir hasta dos niveles de subcarpetas y limitar cuántos
archivos usa; `reiwand-ctl prop set` acepta rutas relativas a la carpeta actual. El formato también
describe atajos que el usuario elige para abrir con un clic y valores que cambian con el nivel de
calidad. Mientras una pantalla comparte los valores con las demás, lo que
se cambia en ella cambia en todas. Un paquete de vídeo ofrece velocidad (sin cambiar el tono ni
dibujar más fotogramas), el tramo que se repite, volumen propio, zoom y posición, con los valores
por defecto que elige el autor, y se aplican sin volver a empezar el vídeo.

La carpeta `scenes/` guarda seis escenas de ejemplo de dominio público (CC0):
capas, modos de fusión, máscaras, línea de tiempo y profundidad que sigue al puntero o una órbita
lenta con un mapa calculado a partir de sus formas (`dusk`), partículas, efectos, el sonido del
viento y las imágenes de una carpeta que elijas tras la nieve (`snowfall`), análisis del sonido
con la carátula, el título y el artista de lo que suena en el centro (`pulse`), puntero y el sonido del agua (`pond`) y un script con una imagen o un vídeo de fondo que
eliges (`orbits`) y un modelo 3D propio de menos de 64 KiB, una caja con su pomo, que gira bajo una luz con sombra (`solids`, con el modelo en `scenes/solids/assets/crate.glb`: así quien edita puede probar el 3D copiando esa escena, sin buscar un archivo). `scripts/ambient-sounds.sh` sintetiza esos dos sonidos (ruido filtrado y gotas,
también CC0) y los guarda como Ogg Vorbis.

Las escenas pueden tener sonido propio: capas `sound` con sonidos Ogg Vorbis, Opus, FLAC o WAV del
paquete (de 60 s como mucho cada uno), en bucle o de una vez, que empiezan y paran con una línea de
tiempo o una propiedad, con volumen, panorama y fundidos, mezcladas en un solo stream. Empiezan en
silencio, como los vídeos: suenan con `reiwand-ctl unmute` o el botón de sonido de la app, de la
bandeja o de cada pantalla que muestra una escena con sonido, y siguen el volumen general y la
propiedad *Volumen* que aparece en los ajustes de cada escena con sonido. Solo suenan mientras la
escena se ve en una pantalla activa y sin tapar (se pausan y siguen con ella al taparla, aunque la
escena no se pause, al pausar o con batería, y se paran al quitarla), nunca con la pantalla
bloqueada, y una escena que se ve en varias pantallas suena en una sola. `playback.scene_sound = false`
(*Dejar que las escenas reproduzcan sus sonidos* en los ajustes de la app y en la bandeja, o
`reiwand-ctl config playback.scene_sound false`) calla el sonido de todas las escenas sin tocar el de
los vídeos; el teléfono no reproduce el sonido de las escenas. Cada capa suena en el punto que marca
el tiempo de la escena: una capa que empieza con una línea de tiempo arranca en la posición de esa
línea (no desde cero), vuelve a empezar con cada vuelta de la línea, sigue sonando si la línea se
para y salta a donde está la línea al reanudarla; el daemon vuelve a alinear el sonido con la escena
cada segundo contando desde la posición que reproduce la salida de audio, más un ciclo de PipeWire
de 20 ms cuando el sonido va a PipeWire. Si la salida de audio falla o no
hay ninguna (también con el plugin de PipeWire viejo), la escena y los vídeos siguen sin sonido y lo
recuperan solos cuando aparece una salida. `install.sh` las instala en
`~/.local/share/reiwand/examples` (`--no-examples` las omite), el RPM en el paquete opcional
`reiwand-examples` y el Flatpak las incluye. El daemon las ofrece como un repositorio local
ya añadido, «Example wallpapers» (`remote ls` lo marca `bundled`), que se firma en la primera
ejecución con claves propias de cada instalación (ninguna va en el código ni en los paquetes) y se
actualiza al arrancar y una vez al día. Se instalan desde ahí como cualquier paquete. Si se quita
con `remote rm`, no vuelve a añadirse solo; `reiwand-ctl remote examples` lo añade de nuevo.
