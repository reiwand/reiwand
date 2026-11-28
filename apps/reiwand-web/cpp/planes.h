#pragma once
#include <EGL/egl.h>
#include <EGL/eglext.h>

// A DMA-BUF has at most four planes, and eglExportDMABUFImageQueryMESA writes one modifier for each of them (two
// or more with a compressed modifier such as Intel's CCS): the array it writes into must hold them all.
constexpr int kMostPlanes = 4;

struct Queried {
    int fourcc = 0;
    int planes = 0;
    EGLuint64KHR modifier = 0;
};

inline bool queryImage(PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC query, EGLDisplay display, EGLImageKHR image,
                       Queried &out) {
    EGLuint64KHR modifiers[kMostPlanes] = {};
    if (!query(display, image, &out.fourcc, &out.planes, modifiers)) return false;
    out.modifier = modifiers[0];
    return out.planes >= 1 && out.planes <= kMostPlanes;
}
