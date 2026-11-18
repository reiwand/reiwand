#pragma once
#include <GLES3/gl3.h>
#include <QOpenGLContext>
#include <cstdint>

struct Exported {
    int fd = -1;
    uint32_t fourcc = 0;
    uint64_t modifier = 0;
    uint32_t stride = 0;
    uint32_t offset = 0;
    int planes = 0;
};

bool dmabufExport(QOpenGLContext *context, GLuint texture, Exported &out);
