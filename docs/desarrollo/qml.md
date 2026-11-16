# Trampas conocidas de QML

## Una lista declarada de componentes (`list<Component>`)

Con Qt 6.12.0 (y quizá otras versiones), llenar una propiedad declarada `list<Component>` con
varios `Component { … }` escritos en el mismo archivo deja un puntero suelto en la memoria del
motor de guiones:

```qml
readonly property list<Component> pages: [
    Component { LibraryPage {} },
    Component { PlaylistsPage {} }
]
```

Al agregar cada elemento, `QQmlVMEMetaObject::list_append` guarda un puntero directo al arreglo
de la lista y después crea el envoltorio del componente, que pide memoria. Si en ese momento corre
un paso de la limpieza de memoria, el arreglo se libera mientras la lista todavía apunta a él. Lo
que después ocupa ese lugar queda pisado, y el proceso muere más tarde en otro sitio, por ejemplo
cuando la limpieza destruye el contexto de dibujo de un `Canvas`
(`Heap::QQuickJSContext2D::destroy`).

Es raro con la limpieza normal (una vez cada varios cientos de arranques), pero se ve siempre con
`QV4_MM_AGGRESSIVE_GC=1`: muere al cargar, dentro de `list_append`, sin código propio. Con
`list<QtObject>` o `list<Kirigami.Action>` no pasa.

Forma segura: declarar cada componente con su `id` y juntar los `id` en un arreglo común.

```qml
readonly property var pages: [libraryPage, playlistsPage]

Component {
    id: libraryPage

    LibraryPage {
    }
}
```

`Main.qml` usa esta forma. La prueba `no_page_fills_a_declared_list_of_components`
(`apps/reiwand-app/src/tests/styled.rs`) falla si algún archivo de `apps/reiwand-app/qml` vuelve a
declarar `list<Component>`.

Para comprobarlo a mano: `QV4_MM_AGGRESSIVE_GC=1 QT_QPA_PLATFORM=offscreen reiwand-app --check`
debe pasar la carga sin morir (es muy lento: sirve para ver el arranque y las primeras unidades).
