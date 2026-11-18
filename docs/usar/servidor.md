# Servidor propio

`reiwand-server` publica un repositorio firmado de paquetes, un catálogo web sin JavaScript, cuentas
con subidas y moderación, y el inicio de sesión de dispositivos con un código. Es un solo binario en un contenedor sin
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

`repo init` crea las claves del repositorio; la clave raíz la guarda el servidor, cifrada, en su carpeta
de datos (entra en las copias de seguridad y nunca se muestra). Para más repositorios, *Repositorios* en el
encabezado de la administración pide solo un nombre y arma la dirección sola. Si `repo init` responde `error: a repository already exists`, ese servidor ya tiene su
repositorio y no hay nada que hacer (`reiwand-server repo status` dice su estado y su dirección). El servidor usa la clave en línea para volver a firmar el índice cada día (cada firma caduca a los 7). Desde *Administración* (o con la
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
| `REIWAND_TLS` | `off` | `self-signed`: sirve HTTPS con un certificado propio, que los clientes aceptan solo cuando confirmas su huella |

La lista completa de variables y la API HTTP están en
[`docs/specs/server.md`](../specs/server.md).

## Actualizar desde 6.x a 7.0

La 7.0.0 cambia los formatos de paquetes, repositorios, ajustes y base: el servidor ya no lee los de la
6.x. Actualiza primero los equipos de escritorio y los teléfonos (`instalacion.md`, «Actualizar desde 6.x
a 7.0») y después el servidor.

- La imagen ejecuta `reiwand-upgrade server --data /data` antes de arrancar. Sin la imagen, córrela a mano con
  el servidor parado; `--check` dice qué cambiaría sin escribir nada.
- Convierte la base (esquema 16) y los ajustes guardados: la 7.0 escribe todas las claves de ajustes
  (registro, publicación automática, TLS…) con los valores de tu entorno de hoy, y se niega a arrancar si falta
  alguna: «the saved settings lack "CLAVE"; run `reiwand-upgrade server --data <data folder>` once». Si la
  base es de otro esquema dice «the database has schema N but this program reads M; … run
  `reiwand-upgrade server --data <data folder>` once».
- Guarda una copia de cada archivo que cambia en `upgrade-copias/<tiempo>/` de la carpeta de datos, con su
  `index.txt`, y repetirla no cambia nada.
- Los paquetes ya publicados se guardan tal como los subieron sus autores y la conversión no los reescribe
  (sus firmas son de cada autor). Un cliente 7.0 que descarga uno con manifiesto de la 6.x
  (`reiwand.wpak/2`) lo rechaza diciendo que se convierta con `reiwand-upgrade file`: cada autor convierte su
  carpeta, firma y sube una versión nueva. Quien ya lo tenía instalado lo conserva convertido en su equipo,
  como «Local · Sin firma verificada».

## Actualizar desde 1.x

El servidor 2.0.0 solo atiende `/api/v1/r/<nombre>/…`: ya no existen `/repo/`, las rutas sin nombre de
repositorio ni el campo `repository` del descubrimiento, y los envíos exigen `repo`. La imagen
ejecuta `reiwand-upgrade server --data /data` antes de arrancar: guarda una copia de la base
(`db/reiwand.sqlite.before-v16-from-vN`), pasa el primer repositorio a `repos/main/` y su clave en línea a
`keys/main.online.key`, y no pierde ningún paquete, versión, voto ni cuenta. Quien lo ejecuta sin la
imagen lo hace con el servidor parado; si el servidor encuentra datos de una versión anterior, se
niega a arrancar y nombra ese comando.

El servidor ya no sincroniza bibliotecas: no guarda listas, favoritos, ajustes, archivos ni claves de
los equipos. Lo que un servidor anterior guardaba de eso no se borra sin más: `reiwand-upgrade server`
lo escribe en `/data/sync-export/<usuario>.json` (los registros tal como estaban, con la carga en
hexadecimal; la que estaba cifrada sigue cifrada, porque el servidor nunca tuvo su clave) y pasa los
archivos privados de cada cuenta a `/data/sync-export/<usuario>/`. Entrégalos a sus dueños y borra la
carpeta. Los equipos de escritorio y los teléfonos conservan su biblioteca tal cual, y para llevarla a
otro equipo se usa un archivo exportado (`reiwand-ctl export`). `reiwand-upgrade client` baja una sola
vez lo que solo guardaba el servidor anterior para esa cuenta (listas, favoritos y archivos privados
sincronizados) a `from-server.reiwand` en la carpeta de datos, que se suma con `reiwand-ctl import`; por
eso se actualiza cada equipo **antes** que el servidor. Si el servidor no responde, no cambia nada y lo
reintenta en el próximo arranque (`--skip-download` sigue sin bajarlo). Además convierte la cuenta del servidor enlazado de la 1.x en una cuenta del repositorio de ese
servidor, con su sesión y el certificado fijado si lo tenía.

## Repositorios y cuentas

Un servidor aloja uno o varios repositorios (`reiwand-server repo init --name N`, o *Repositorios* en
la administración de la web). Cada uno tiene su dirección sencilla, `https://servidor/r/<nombre>`, que
se pega tal cual en la app o con `reiwand-ctl remote add`: nada de huellas ni parámetros. La página
principal del servidor, `/`, es «Explorar repositorios»: lista los repositorios con su dirección para
copiar y el botón «Abrir en Reiwand». Un repositorio **público** se agrega sin pedir nada; uno
**privado** pide una cuenta al conectarse, y el administrador da a cada persona derecho de lectura o
de publicación por repositorio (*Personas*). Un repositorio puede aceptar subidas de cualquier cuenta
(a revisión) o solo de sus miembros.

Una **cuenta** es de una persona en un servidor y se vincula a los repositorios en los que se usa; se
pueden tener varias. Se inicia sesión con un código que se aprueba en el navegador, y la sesión se
guarda en el llavero del sistema (sin llavero, `reiwand-ctl provider store file` la guarda en un
archivo privado de la carpeta de datos, tras un aviso). El acceso solo se envía al servidor de su
repositorio. No hay sincronización: la biblioteca y los ajustes de cada equipo son locales.

```sh
reiwand-ctl remote add https://fondos.example.org/r/main     # público: se agrega
reiwand-ctl remote add https://192.168.1.10:8443/r/main      # servidor de casa con su propio certificado: la primera vez muestra su huella
reiwand-ctl remote add https://192.168.1.10:8443/r/main --trust-certificate <huella>   # … y la confirmas
reiwand-ctl account sign-in https://fondos.example.org/r/privado   # muestra un código y una dirección; apruébalo en el navegador
reiwand-ctl account ls                                       # tus cuentas y los repositorios donde se usa cada una
reiwand-ctl account use https://fondos.example.org/r/otro a1  # usa esa cuenta en otro repositorio del mismo servidor
reiwand-ctl account unlink https://fondos.example.org/r/otro # deja de usarla ahí; la cuenta se conserva
reiwand-ctl account rm a1                                    # la olvida en este equipo, con su acceso guardado
reiwand-ctl server upload fondo.wpak                         # sube un paquete con la cuenta del repositorio
reiwand-ctl server upload fondo.wpak --unlisted              # … oculto: fuera del catálogo, instalable con su enlace
reiwand-ctl server upload fondo.wpak --repo https://fondos.example.org/r/otro   # … en ese repositorio, con la cuenta vinculada a él
reiwand-ctl server packages --repo https://fondos.example.org/r/otro   # tus subidas en ese repositorio, con su estado y visibilidad
reiwand-ctl server uploads                                   # las sesiones de subida (en curso o terminadas); --cancel ID cancela una
reiwand-ctl server author-key list                           # las claves con las que firmas tus paquetes en línea
reiwand-ctl server author-key add autor.pub                  # registra una clave; `rotate`, `remove` la cambian o la quitan
reiwand-ctl server visibility org.example.fondo public       # cambia la visibilidad de un paquete o de una versión
reiwand-ctl export config.reiwand                            # la configuración de este equipo en un archivo (ver abajo)
reiwand-ctl import config.reiwand                            # … y se suma en otro equipo
```

Si agregas un repositorio de un servidor donde ya tienes una cuenta, la app pregunta si usas esa
cuenta u otra; en la terminal, `account use` o `account sign-in`. Pegar la dirección de un servidor
(sin `/r/<nombre>`) no lista repositorios: dice que es un servidor y nombra las direcciones de sus
repositorios públicos. Al publicar se elige el repositorio de destino; si no hay una cuenta vinculada a
él, se pide ahí, sea público o privado.

Con más de un repositorio con cuenta, `server packages`, `uploads`, `author-key` y `visibility` piden
`--repo DIRECCIÓN` (la dirección o el id del repositorio) y, sin él, fallan nombrando las direcciones entre
las que elegir. En la app, *Cuenta* muestra en cada cuenta, por repositorio, *Mis subidas* (estado y
visibilidad, que se cambia ahí; las sesiones sin terminar se cancelan) y *Claves de autor* (lista,
registrar desde un archivo, traspasar y quitar). Una cuenta que debe iniciar sesión de nuevo lo dice en su
fila y al publicar o votar con ella la app ofrece *Iniciar sesión de nuevo* allí mismo.

**Tu primera cuenta, sin navegador en el equipo** (la cuenta del administrador ya existe y `galleta` es
su sesión, de la sección anterior):

```sh
reiwand-ctl provider store file                           # solo sin llavero del sistema
reiwand-ctl account sign-in http://127.0.0.1:8080/r/main  # imprime el código ABCD-EFGH y la dirección, y espera
curl -sS -b galleta -H 'content-type: application/json' \
  -d '{"user_code": "ABCD-EFGH", "approve": true}' \
  http://127.0.0.1:8080/api/v1/auth/device/approve     # {"approved":true}; con navegador, abre la dirección y escribe el código
reiwand-ctl account ls                                    # la cuenta, con el repositorio donde se usa
```

`http://` solo vale para este mismo equipo: un servidor que está en otro equipo necesita HTTPS (más
abajo, con su propio certificado o con un proxy). Aprueba solo un código que muestre la pantalla de tu
propio equipo: quien te envía un código intenta entrar en tu cuenta. Aprobar pide una sesión web de la
cuenta, no el acceso de un equipo, así que quien robe el acceso de un equipo no puede añadir otros.
Cambiar la contraseña en la web cierra las demás sesiones y borra los tokens.

## Mover la configuración a otro equipo

`reiwand-ctl export ARCHIVO` (o *Ajustes > Equipo > Mover mi configuración*) escribe un archivo con los
repositorios agregados, los proveedores de fotos, las listas, los favoritos, las valoraciones, las
etiquetas, los fondos instalados (como referencias, que se vuelven a bajar) y los ajustes que tienen
sentido en otro equipo. `--own` suma tus fondos propios; `--secrets` suma las claves de los proveedores
y las sesiones de tus cuentas, y cifra el archivo con una contraseña. `reiwand-ctl import ARCHIVO` solo
agrega y nunca reemplaza (`--list` muestra antes lo que trae). Con una cuenta importada, el servidor
da a cada equipo su propia sesión, que se revoca sola. El detalle está en
[`docs/specs/exportar.md`](../specs/exportar.md).

Al subir un paquete eliges si es público u oculto (en el diálogo de subida de *Repositorios*, en la
web del servidor o con `--unlisted`): uno oculto se publica y se firma igual, pero el catálogo y la
búsqueda no lo muestran, y quien tenga su enlace o su id lo ve y lo instala. Oculto no es privado:
sigue en los archivos firmados del repositorio, que cualquiera puede descargar y leer, con su título y
su descripción; lo privado va en un repositorio privado. La página *Cuenta* de la web lista tus
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
muestra la versión de la raíz, cuántas claves la firmaron y la dirección del repositorio, la firma con un
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
`/admin/featured` del servidor), y, con una cuenta en el servidor, cada paquete del repositorio de ese servidor muestra sus votos a favor y
en contra, con tu voto marcado, y botones para votar (`reiwand-ctl remote vote ID up|down|none`, `remote votes ID…`,
también como `REPO/ID`); los paquetes de otros repositorios no muestran votos y sus ids nunca se
envían al servidor. El
servidor cuenta un voto por cuenta y paquete, guarda solo los recuentos y tu propio voto, nunca una
lista de quién votó ni de quién instaló qué. El ajuste *Mostrar los votos del servidor en el
catálogo* (`repos.vote_counts`, también en los ajustes del teléfono) deja de pedir y mostrar los
recuentos. Tus valoraciones locales de la
biblioteca siguen siendo privadas y solo salen en un archivo exportado.


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
para que los clientes no vean una versión anterior.

En una red de casa sin dominio, el servidor puede servir HTTPS él mismo con un certificado propio
con la dirección que el administrador da a mano:

```sh
REIWAND_TLS=self-signed REIWAND_LISTEN=0.0.0.0:8443 \
  REIWAND_PUBLIC_URL=https://192.168.1.10:8443 reiwand-server serve
reiwand-server fingerprint                    # la huella SHA-256 del certificado, también en el registro
reiwand-ctl remote add https://192.168.1.10:8443/r/main --trust-certificate <huella>
```

El certificado se crea en el primer arranque en `/data/tls` y se conserva. Al agregar el repositorio,
la primera vez la app y `remote add` dicen que el servidor usa su propio certificado y muestran su
huella; compárala con la de `reiwand-server fingerprint` y confírmala («Confiar y agregar», o
`--trust-certificate`). Queda fijada solo para ese servidor: si más tarde presenta otro, se avisa y no
se aplica nada. Con un servidor de la red de casa todo funciona igual que con uno público, sin salir de
la red.

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
`Connection ""` hacia el servidor y sin guardar las subidas en disco. El servidor no usa WebSockets ni conexiones largas.

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
clics y enciende o apaga los clics de esa pantalla. Los preajustes propios son del paquete (siguen tras actualizarlo) y viajan en el archivo exportado.

Los autores describen sus propiedades en el manifiesto (`docs/specs/scene.md`): colores con
transparencia, deslizadores con paso, decimales, unidades, escala logarítmica y casilla para
escribir el número, listas con iconos y etiquetas traducidas, textos y textos secretos (que se
guardan en el llavero del sistema y nunca llegan a `config.toml`, ni a los registros), separadores, etiquetas y grupos plegables, botones que lanzan eventos de la escena,
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
