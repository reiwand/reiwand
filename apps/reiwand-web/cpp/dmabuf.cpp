#include "dmabuf.h"
#include "planes.h"
#include <QtGui/qguiapplication_platform.h>
#include <unistd.h>

static PFNEGLCREATEIMAGEKHRPROC createImage;
static PFNEGLDESTROYIMAGEKHRPROC destroyImage;
static PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC query;
static PFNEGLEXPORTDMABUFIMAGEMESAPROC exportImage;
static bool resolved;

static void resolve() {
    if (resolved) return;
    resolved = true;
    createImage = reinterpret_cast<PFNEGLCREATEIMAGEKHRPROC>(eglGetProcAddress("eglCreateImageKHR"));
    destroyImage = reinterpret_cast<PFNEGLDESTROYIMAGEKHRPROC>(eglGetProcAddress("eglDestroyImageKHR"));
    query = reinterpret_cast<PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC>(eglGetProcAddress("eglExportDMABUFImageQueryMESA"));
    exportImage = reinterpret_cast<PFNEGLEXPORTDMABUFIMAGEMESAPROC>(eglGetProcAddress("eglExportDMABUFImageMESA"));
}

bool dmabufExport(QOpenGLContext *context, GLuint texture, Exported &out) {
    auto *egl = context->nativeInterface<QNativeInterface::QEGLContext>();
    if (!egl) return false;
    resolve();
    if (!createImage || !destroyImage || !query || !exportImage) return false;
    EGLDisplay display = egl->display();
    if (display == EGL_NO_DISPLAY) return false;
    EGLImageKHR image = createImage(display, egl->nativeContext(), EGL_GL_TEXTURE_2D_KHR,
                                    reinterpret_cast<EGLClientBuffer>(static_cast<uintptr_t>(texture)), nullptr);
    if (!image) return false;
    Queried found;
    bool done = false;
    if (queryImage(query, display, image, found) && found.planes == 1) {
        int fd = -1;
        EGLint stride = 0, offset = 0;
        if (exportImage(display, image, &fd, &stride, &offset) && fd >= 0) {
            out = {fd, static_cast<uint32_t>(found.fourcc), found.modifier, static_cast<uint32_t>(stride),
                   static_cast<uint32_t>(offset), found.planes};
            done = true;
        }
    }
    destroyImage(display, image);
    return done;
}
