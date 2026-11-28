# Known QML pitfalls

## A declared list of components (`list<Component>`)

With Qt 6.12.0 (and perhaps other versions), filling a property declared as `list<Component>` with
several `Component { … }` written in the same file leaves a dangling pointer in the memory of the
script engine:

```qml
readonly property list<Component> pages: [
    Component { LibraryPage {} },
    Component { PlaylistsPage {} }
]
```

When each element is appended, `QQmlVMEMetaObject::list_append` stores a direct pointer to the
list's array and then creates the component's wrapper, which asks for memory. If at that moment a
garbage collection step runs, the array is freed while the list still points to it. What later
occupies that place is overwritten, and the process dies later somewhere else, for example when the
collector destroys the drawing context of a `Canvas` (`Heap::QQuickJSContext2D::destroy`).

It is rare with normal collection (once every several hundred starts), but it always shows with
`QV4_MM_AGGRESSIVE_GC=1`: it dies on load, inside `list_append`, with no code of our own. With
`list<QtObject>` or `list<Kirigami.Action>` it does not happen.

Safe form: declare each component with its `id` and gather the `id`s in a common array.

```qml
readonly property var pages: [libraryPage, playlistsPage]

Component {
    id: libraryPage

    LibraryPage {
    }
}
```

`Main.qml` uses this form. The test `no_page_fills_a_declared_list_of_components`
(`apps/reiwand-app/src/tests/styled.rs`) fails if any file in `apps/reiwand-app/qml` declares
`list<Component>` again.

To check it by hand: `QV4_MM_AGGRESSIVE_GC=1 QT_QPA_PLATFORM=offscreen reiwand-app --check` must
get through the load without dying (it is very slow: it serves to see the startup and the first
units).
