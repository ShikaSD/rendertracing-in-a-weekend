#include<objects.rsh>

typedef struct {
    float dist;
    float3 p;
    float3 normal;
} record;

typedef enum {
    obj_sphere
} obj;

typedef union {
    sphere sphere;
} hitable;

typedef struct {
    obj type;
    hitable value;
} hitable_t;

static bool hit_sphere(const sphere *s, const ray *r, record *rec, float dist_min, float dist_max) {
    float3 d_oc = r->origin - s->center;
    float a = dot(r->direction, r->direction);
    float b = 2 * dot(d_oc, r->direction);
    float c = dot(d_oc, d_oc) - s->radius * s->radius;
    float d = b * b - 4 * a * c;

    if (d >= 0) {
        float dist = (-b - sqrt(b * b - a * c)) / a;
        if (dist > dist_max || dist < dist_min) {
            dist = (-b + sqrt(b * b - a * c)) / a;
        }
        if (dist < dist_max && dist > dist_min) {
            rec->dist = dist;
            rec->p = ray_at(r, dist);
            rec->normal = (rec->p - s->center) / s->radius; // normalize
            return true;
        }
    }
    return false;
}

static bool hit_list(const hitable_t *h, int hsize, const ray *r, record **rec, float dist_min, float dist_max) {
    float closest = dist_max;
    record t_rec;
    bool hit_result = false;

    for (int i = 0; i < hsize; i++) {
        hitable_t current = *(h + i);

        bool hit = false;
        switch (current.type) {
            case obj_sphere:
                hit = hit_sphere(&current.value.sphere, r, &t_rec, dist_min, closest);
                break;
        }

        if (hit) {
            closest = t_rec.dist;
            hit_result = true;
            *rec = &t_rec;
        }
    }

    return hit_result;
}
