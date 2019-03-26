#include<objects.rsh>

typedef struct {
    float3 start;
    float3 horizontal;
    float3 vertical;
    float3 origin;
} camera;

static ray camera_ray(const camera *c, float u, float v) {
    ray r = { c->origin, c->start + u * c->horizontal + v * c->vertical };
    return r;
}
