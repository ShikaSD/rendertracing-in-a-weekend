#pragma version(1)
#pragma rs java_package_name(com.shika.rayrender)

#define FLT_MAX 3.402823466e+38F
#define FLT_MIN 1.175494351e-38F

const float3 lower_left_corner = {-2, -1, -1};
const float3 horizontal = {4, 0, 0};
const float3 vertical = {0, 2, 0};
const float3 origin = {0, 0, 0};

typedef struct {
    float3 origin;
    float3 direction;
} ray;

typedef struct {
    float3 center;
    float radius;
} sphere;

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

static float3 ray_at(const ray *r, float t) {
    return r->origin + r->direction * t;
}

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

static float3 color(const ray *r, const hitable_t *world) {
    record* rec = NULL;
    bool got_hit = hit_list(world, 2, r, &rec, 0, FLT_MAX);

    if (got_hit) {
        float3 normal = rec->normal;
        return 0.5 * (float3) { normal.x + 1, normal.y + 1, normal.z + 1 };
    }

    float3 norm_direction = normalize(r->direction);
    float t = 0.5 * (norm_direction.y + 1);
    return (1 - t) * (float3) { 1, 1, 1 } + t * (float3) { 0.5, 0.7, 1.0 };
}

uchar4 RS_KERNEL raytrace(uchar4 in, rs_kernel_context context, int32_t x, int32_t y) {
    float4 input = rsUnpackColor8888(in);
    int32_t width = rsGetDimX(context);
    int32_t height = rsGetDimY(context);

    hitable s_h = { { { 0, 0, -1 }, 0.5 } };
    hitable g_h = { { { 0, -100.5, -1 }, 100 } };
    
    hitable_t world[2] = {
        { .type = obj_sphere, .value = s_h },
        { .type = obj_sphere, .value = g_h }
    };

    float u = ((float) x) / width;
    float v = 1 - ((float) y) / height;

    float3 direction = lower_left_corner + u * horizontal + v * vertical;
    ray r = { origin, direction };
    float3 col = color(&r, world);

    input.r = col.r;
    input.g = col.g;
    input.b = col.b;
    input.a = 1;

    return rsPackColorTo8888(input);
}
