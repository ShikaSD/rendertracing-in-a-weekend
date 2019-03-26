#ifndef objects
#define objects true

typedef struct {
    float3 origin;
    float3 direction;
} ray;

static float3 ray_at(const ray *r, float t) {
    return r->origin + r->direction * t;
}

typedef struct {
    float3 center;
    float radius;
} sphere;

#endif
