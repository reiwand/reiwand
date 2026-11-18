#pragma once
#include <cstddef>
#include <cstdint>

extern "C" {
struct RwHost {
    void (*ready)(const char *version);
    void (*readable)();
    void (*loaded)(uint32_t view);
    void (*crashed)(uint32_t view, int status);
    void (*message)(uint32_t view, const char *json);
    void (*answered)(uint32_t view);
    void (*buffer)(uint32_t view, uint32_t id, uint32_t width, uint32_t height, uint32_t fourcc, uint64_t modifier,
                   uint32_t offset, uint32_t stride, int fd);
    void (*frame)(uint32_t view, uint32_t id);
    void (*forget)(uint32_t view);
    uint8_t *(*slot)(uint32_t view, uint32_t width, uint32_t height, uint32_t stride, uint32_t *index);
    void (*pixels)(uint32_t view, uint32_t index, uint32_t width, uint32_t height, uint32_t stride);
    int (*serve)(const char *uri, uint64_t *size, char *mime, uint32_t room);
    void (*said)(const char *text);
};

int rw_check(char *message, size_t room);
int rw_run(const char *config, const RwHost *host, int socket);
void rw_quit();
void rw_open(uint32_t view, uint32_t width, uint32_t height, double scale);
void rw_close(uint32_t view);
void rw_resize(uint32_t view, uint32_t width, uint32_t height, double scale);
void rw_visible(uint32_t view, int visible);
void rw_fps(uint32_t view, uint32_t fps);
void rw_scripts(uint32_t view, const char *json);
void rw_load(uint32_t view, const char *url);
void rw_eval(uint32_t view, const char *code);
void rw_ping(uint32_t view);
int64_t rw_renderer(uint32_t view);
void rw_muted(int muted);
void rw_copy();
void rw_released(uint32_t view, uint32_t id);
void rw_shot(uint32_t view);
void rw_pointer(uint32_t view, int present, double x, double y);
void rw_press(uint32_t view, double x, double y, uint32_t button);
void rw_later(uint32_t millis, void (*run)(void *), void *data);
}
