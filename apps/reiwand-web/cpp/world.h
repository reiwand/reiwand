#pragma once
#include "bridge.h"
#include "host.h"
#include <GLES3/gl3.h>
#include <QColor>
#include <QOffscreenSurface>
#include <QOpenGLContext>
#include <QQmlComponent>
#include <QQmlEngine>
#include <vector>

struct World {
    const RwHost *host = nullptr;
    QOpenGLContext *context = nullptr;
    QOffscreenSurface *surface = nullptr;
    QQmlEngine *engine = nullptr;
    QQmlComponent *component = nullptr;
    Bridge *bridge = nullptr;
    std::vector<uint64_t> modifiers;
    bool copy = false;
    QColor background;
};

World &world();
bool openContext(World &w, QString &why);

bool hasDisplay(QString &why);
void prepare();
void openPage(uint32_t id, QSize size, double scale);
void closePage(uint32_t id);
