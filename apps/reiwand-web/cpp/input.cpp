#include "page.h"
#include <QHoverEvent>
#include <QMouseEvent>

static QQuickItem *inputOf(QQuickItem *item) {
    for (QQuickItem *child : item->childItems()) {
        if (QByteArray(child->metaObject()->className()).endsWith("DelegateItem")) return child;
        if (QQuickItem *found = inputOf(child)) return found;
    }
    return nullptr;
}

void Page::hover(QPointF at) {
    if (!input) input = inputOf(window->contentItem());
    if (input) {
        QHoverEvent event(QEvent::HoverMove, at, input->mapToGlobal(at), last);
        QCoreApplication::sendEvent(input, &event);
    } else {
        QMouseEvent move(QEvent::MouseMove, at, at, at, Qt::NoButton, Qt::NoButton, Qt::NoModifier);
        QCoreApplication::sendEvent(window, &move);
    }
    last = at;
}

void Page::leave() {
    QEvent event(QEvent::Leave);
    QCoreApplication::sendEvent(window, &event);
}
