#ifndef objects_incl_guard
#define objects_incl_guard true

typedef struct {
    float dist;
    float3 p;
    float3 normal;
} record;

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
