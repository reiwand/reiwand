// Built with AddressSanitizer by tests/native.rs: the query of a DMA-BUF writes one modifier per plane, so a
// compressed modifier with an auxiliary plane must not write past what the helper gives it.
#include "planes.h"
#include <cstdio>

static int planesToReport = 1;

static EGLBoolean fakeQuery(EGLDisplay, EGLImageKHR, int *fourcc, int *planes, EGLuint64KHR *modifiers) {
    *fourcc = 0x34325241;
    *planes = planesToReport;
    for (int i = 0; i < planesToReport; i++) modifiers[i] = 0x0100000000000004ull + i;
    return EGL_TRUE;
}

static int check(bool ok, const char *what) {
    if (!ok) std::fprintf(stderr, "failed: %s\n", what);
    return ok ? 0 : 1;
}

int main() {
    int failed = 0;
    Queried one;
    failed += check(queryImage(fakeQuery, nullptr, nullptr, one), "a single plane is accepted");
    failed += check(one.planes == 1 && one.modifier == 0x0100000000000004ull, "the single plane's modifier is read");
    for (int planes = 2; planes <= kMostPlanes; planes++) {
        planesToReport = planes;
        Queried many;
        failed += check(queryImage(fakeQuery, nullptr, nullptr, many), "a compressed modifier is queried");
        failed += check(many.planes == planes, "every plane is counted");
        failed += check(many.modifier == 0x0100000000000004ull, "the first plane's modifier is kept");
    }
    return failed == 0 ? 0 : 1;
}
