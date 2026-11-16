import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.ComboBox {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    padding: 0
    spacing: Look.e2
    leftPadding: !mirrored || !indicator ? 0 : indicator.width + spacing + Look.e2
    rightPadding: mirrored || !indicator ? 0 : indicator.width + spacing + Look.e2
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo

    delegate: ComboOption {
        required property var model
        required property int index

        width: ListView.view.width
        text: model[control.textRole]
        chosen: control.currentIndex === index
        highlighted: control.highlightedIndex === index
        hoverEnabled: control.hoverEnabled
    }

    indicator: IconImage {
        x: control.mirrored ? Look.e3 : control.width - width - Look.e3
        y: (control.height - height) / 2
        width: Look.icono
        height: Look.icono
        sourceSize: Qt.size(Look.icono, Look.icono)
        name: "caret-down"
        color: control.enabled ? Look.bruma : Look.brumaBaja
    }

    contentItem: T.TextField {
        implicitWidth: (control.editable ? contentWidth : shown.implicitWidth) + leftPadding + rightPadding
        implicitHeight: contentHeight + topPadding + bottomPadding
        leftPadding: Look.e3
        rightPadding: 0
        topPadding: Look.e1
        bottomPadding: Look.e1
        text: control.editable ? control.editText : ""
        enabled: control.editable
        autoScroll: control.editable
        readOnly: control.down
        inputMethodHints: control.inputMethodHints
        validator: control.validator
        selectByMouse: control.selectTextByMouse
        font: control.font
        color: control.enabled ? Look.escarcha : Look.brumaBaja
        selectionColor: Look.luna
        selectedTextColor: Look.noche
        verticalAlignment: Text.AlignVCenter

        Text {
            id: shown

            x: parent.leftPadding
            width: parent.width - parent.leftPadding - parent.rightPadding
            height: parent.height
            visible: !control.editable
            text: control.displayText
            font: control.font
            color: parent.color
            elide: Text.ElideMiddle
            verticalAlignment: Text.AlignVCenter
            T.ToolTip.visible: control.hovered && truncated && !control.down
            T.ToolTip.text: text
            T.ToolTip.delay: Qt.styleHints.mousePressAndHoldInterval
        }
    }

    background: Field {
        implicitWidth: 160
        lit: control.editable ? control.activeFocus : control.visualFocus
        over: control.hovered
        live: control.enabled && !control.flat
        color: !live ? "transparent" : control.down ? Look.noche : over && !control.editable ? Look.nocheLinea : Look.nocheAlta
    }

    popup: T.Popup {
        property real widest: 0
        readonly property FontMetrics metrics: FontMetrics {
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: Font.DemiBold
        }

        y: control.height + Look.e1
        width: Math.min(Math.max(control.width, widest + leftPadding + rightPadding), control.Window.width - leftMargin - rightMargin)
        leftMargin: Look.e2
        rightMargin: Look.e2
        height: Math.min(contentItem.implicitHeight + topPadding + bottomPadding, control.Window.height - topMargin - bottomMargin)
        topMargin: Look.e2
        bottomMargin: Look.e2
        padding: 6
        font: control.font
        onAboutToShow: {
            let wide = 0;
            for (let i = 0; i < control.count; ++i)
                wide = Math.max(wide, metrics.advanceWidth(control.textAt(i)));
            widest = Math.ceil(wide) + Look.e3 * 2 + Look.e2;
        }

        contentItem: ListView {
            clip: true
            implicitHeight: contentHeight
            model: control.delegateModel
            currentIndex: control.highlightedIndex
            highlightMoveDuration: 0

            T.ScrollIndicator.vertical: ScrollIndicator {
            }
        }

        background: Panel {
        }
    }
}
