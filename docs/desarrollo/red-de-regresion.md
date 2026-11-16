# Red de regresión

Una sola orden dice si el programa sigue dibujando lo mismo, igual de rápido y descansando igual:

```sh
cargo build -p reiwandd -p reiwand-ctl && cargo build -p reiwand-pipewire --example probe
cp target/debug/examples/probe target/debug/
scripts/regression.sh target/debug          # comprueba
scripts/regression.sh --bless target/debug  # guarda los valores nuevos a propósito
```

Corre sin red, sin GPU real y sin el bus de la sesión: el render es el de software (lavapipe; la orden
elige su controlador de Vulkan si está instalado) y el daemon usa un bus y un PipeWire propios, como
`scripts/integration-test.sh`. Con una GPU real y sin lavapipe la parte de imágenes se salta con un
aviso. En la CI corre dentro del trabajo «End-to-end test» y, si falla, sube las imágenes de la
diferencia como el artefacto `regression-net`.

`--bless` nunca corre solo: se usa cuando un cambio altera a propósito lo que se dibuja o lo que
cuesta, y el commit lleva las imágenes y los números nuevos para que se revisen.

## Imágenes de referencia

`REIWAND_NET=1 cargo test -p reiwand-ctl --test net` dibuja cada escena de `scenes/` (empaquetada como `.wpak`, con sus
scripts) a 160×90 en los segundos 0, 1 y 5, con reloj de pared, semilla y entradas fijos, y además una
imagen fija, la misma imagen con relieve de profundidad (tres desplazamientos) y un vídeo (los
segundos 0, 1 y 5 de `tests/fixtures/quadrants-64x32.webm`). Cada caso es una hoja con sus cuadros en
`apps/reiwand-ctl/tests/net/pictures/`; dibuja también dos veces el primer cuadro de cada escena y falla
si no salen iguales.

Una imagen falla cuando más del 0,4 % de sus píxeles se aparta más de 24 en algún canal o la
diferencia media pasa de 1,5. Entonces escribe `NOMBRE-got.png` (lo dibujado), `NOMBRE-expected.png`
(la referencia) y `NOMBRE-difference.png` (la diferencia por cuatro) en `target/tmp/net/`, o en la
carpeta de `REIWAND_NET_OUT`.

## Tiempos de cuadro

La misma prueba guarda en `apps/reiwand-ctl/tests/net/counters.json`, por caso:

| número | qué es |
|---|---|
| `needed` | cuadros que hubo que dibujar en 70 a 60 cuadros por segundo (`reiwand-ctl scene bench`); 1 en una escena quieta |
| `passes`, `draws`, `textures` | pasadas, llamadas de dibujo y texturas del último cuadro |
| `created`, `compiled` | recursos y pipelines creados después del calentamiento: deben ser 0 |
| `particles`, `steps` | huecos de partículas y pasos de simulación |
| `allocations` | reservas de memoria por cuadro estable en el hilo que dibuja |
| `ms` | tiempo de pared medio por cuadro |

Los contadores no dependen de la máquina: fallan si crecen más del 10 % (al menos 1) sobre lo
guardado. `ms` sí depende: solo falla por encima de ocho veces lo guardado más 20 ms, para atrapar un
cambio de orden de magnitud y no el ruido de la máquina.

Las reservas de memoria incluyen las del controlador, así que el archivo guarda también qué
controlador las midió (`adapter`). La prueba usa siempre Vulkan; con otro controlador (otra versión
de lavapipe, por ejemplo en la CI) las reservas solo fallan si pasan del doble más 50, y la prueba
lo avisa al empezar. Sin `REIWAND_NET` la prueba no hace nada: un `cargo test` de varios paquetes
juntos activa funciones de otros crates y cambia las reservas, así que la red corre solo con la
compilación propia de `scripts/regression.sh`. La prueba fija además `RUST_LIB_BACKTRACE=0`: en una
compilación de depuración la GPU guarda una traza de pila en cada cerrojo si `RUST_BACKTRACE` está
puesto, y eso sumaría reservas que no son del programa. Los pánicos siguen mostrando su traza.

## Coste en reposo

`scripts/integration-idle.sh` arranca un daemon privado con una salida virtual de 160×90 y la mira con `probe`, como
lo haría un compositor, y mide durante 3 s los cuadros dibujados de la salida y las veces que el daemon se despierta (la suma de
cambios de contexto de todos sus hilos en `/proc`) en tres casos: una imagen fija, una escena tapada
por una ventana y una escena en pausa. Antes comprueba que la escena se dibuja cuando se ve. Los topes
están en `apps/reiwand-ctl/tests/net/idle.txt` (`caso cuadros despertares`); `--bless` guarda los cuadros
medidos y el doble de los despertares más 20.

El motor del teléfono no entra en esta red: sus pruebas comprueban cuándo pausa
(`crates/reiwand-android/src/power/tests.rs`) y a qué ritmo dibuja (`power/pace.rs`), pero no cuentan
cuadros ni despertares en reposo; eso lo mide `scripts/android-power.sh` en un teléfono.
