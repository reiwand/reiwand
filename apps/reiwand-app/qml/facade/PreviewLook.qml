pragma Singleton

import QtQuick

QtObject {
    readonly property int focoAncho: 520
    readonly property int cercaAncho: 264
    readonly property int lejosAncho: 150
    readonly property int hueco: 18
    readonly property int panelLado: 36
    readonly property int panelArriba: 24
    readonly property int panelAbajo: 22
    readonly property int panelRadio: 14
    readonly property int tarjetaRadio: 9
    readonly property int anilloHueco: 4
    readonly property int anillo: 3
    readonly property int paso: 190
    readonly property int aparece: 170
    readonly property int sale: 150
    readonly property int espera: 5000
    readonly property int reposo: 5000
    readonly property real velo: 0.55
    readonly property real cercaOpaca: 0.78
    readonly property real lejosOpaca: 0.36
    readonly property real ruedaPaso: 120
    readonly property real tTitulo: 15
    readonly property real tNombre: 24
    readonly property real tSub: 15
    readonly property real tTecla: 13
    readonly property real tInsignia: 12

    function edge(pixels) {
        return [256, 512, 1024, 2048, 4096].find(size => size >= pixels) ?? 4096;
    }

    function unit(width) {
        return Math.max(0.75, Math.min(1.6, width / 1920));
    }
}
