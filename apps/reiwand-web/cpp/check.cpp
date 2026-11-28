#include "world.h"
#include <QGuiApplication>
#include <QtGlobal>
#include <cstdio>

int rw_check(char *message, size_t room) {
    QString why;
    if (!hasDisplay(why)) {
        snprintf(message, room, "%s", why.toUtf8().constData());
        return 2;
    }
    prepare();
    int argc = 1;
    char name[] = "reiwand-web";
    char *argv[] = {name, nullptr};
    QGuiApplication app(argc, argv);
    World probe;
    if (!openContext(probe, why)) {
        snprintf(message, room, "%s", why.toUtf8().constData());
        return 2;
    }
    snprintf(message, room, "%s", qVersion());
    return 0;
}
