#pragma version(1)
#pragma rs java_package_name(com.shika.rayrender)
#include<hitables.rsh>
#include<random.rsh>
#include<camera.rsh>

#define FLT_MAX 3.402823466e+38F
#define FLT_MIN 1.175494351e-38F

const static camera cam = {
    .start = {-2, -1, -1},
    .horizontal = {4, 0, 0},
    .vertical = {0, 2, 0},
    .origin = {0, 0, 0}
};

static float3 color(const ray *r, const hitable_t *world, const half depth) {
    record* rec = NULL;

    bool got_hit = depth < 25 && hit_list(world, 2, r, &rec, 0.001, FLT_MAX);
    if (got_hit) {
        float3 target = rec->p + rec->normal + rand_in_sphere();
        return 0.5 * color(&(ray) { rec->p, target - rec->p }, world, depth + 1);
    }

    float3 norm_direction = normalize(r->direction);
    float t = 0.5 * (norm_direction.y + 1);
    return (1 - t) * (float3) { 1, 1, 1 } + t * (float3) { 0.5, 0.7, 1.0 };
}

uchar4 RS_KERNEL raytrace(uchar4 in, rs_kernel_context context, int32_t x, int32_t y) {
    float4 input = rsUnpackColor8888(in);
    int32_t width = rsGetDimX(context);
    int32_t height = rsGetDimY(context);

    hitable s_h = { { { 0, 0.1, -1 }, 0.5 } };
    hitable g_h = { { { 0, -100.5, -1 }, 100 } };
    
    hitable_t world[2] = {
        { .type = obj_sphere, .value = s_h },
        { .type = obj_sphere, .value = g_h }
    };

    float aa_factor = 100;
    float3 col = {0, 0, 0};
    for (int i = 0; i < aa_factor; i++) {
        float x_f = (float) x;
        float y_f = (float) y;
        float u = (x_f + rand()) / width;
        float v = 1 - (y_f + rand()) / height;

        ray r = camera_ray(&cam, u, v);
        col += color(&r, world, 0);
    }

    col /= aa_factor;

    input.r = col.r;
    input.g = col.g;
    input.b = col.b;
    input.a = 1;

    return rsPackColorTo8888(input);
}
