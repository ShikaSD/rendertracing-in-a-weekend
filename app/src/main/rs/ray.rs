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

static float3 color(const ray *r, const hitable_t *world, const half world_size, const half depth) {
    record rec;
    material mat;
    bool got_hit = hit_list(world, world_size, r, &rec, &mat, 0.001, FLT_MAX);

    if (got_hit) {
        ray result;
        float3 att;

        bool scattered = false;
        switch (mat.type) {
            case LAMBERTIAN:
                scattered = scatter_lambertian(&mat.value.lambertian, &rec, &att, &result);
                break;
            case METAL:
                scattered = scatter_metal(&mat.value.metal, r, &rec, &att, &result);
                break;
        }

        if (depth < 50 && scattered) {
            return att * color(&result, world, world_size, depth + 1);
        } else {
            return (float3){0, 0, 0};
        }
    }

    float3 norm_direction = normalize(r->direction);
    float t = 0.5 * (norm_direction.y + 1);
    return (1 - t) * (float3) { 1, 1, 1 } + t * (float3) { 0.5, 0.7, 1.0 };
}

uchar4 RS_KERNEL raytrace(uchar4 in, rs_kernel_context context, int32_t x, int32_t y) {
    float4 input = rsUnpackColor8888(in);
    int32_t width = rsGetDimX(context);
    int32_t height = rsGetDimY(context);
    
    hitable_t world[] = {
        {
            .type = SPHERE,
            .value = { .sphere = { .center = { 0, 0.3, -1 }, .radius = 0.5 } },
            .material = {
                .type = LAMBERTIAN,
                .value = { .lambertian = { .albedo = { 0.8, 0.3, 0.3 } } }
            }
        },
        {
            .type = SPHERE,
            .value = { .sphere = { .center = { 0, -100.5, -1 }, .radius = 100 } },
            .material = {
                .type = LAMBERTIAN,
                .value = { .lambertian = { .albedo = { 0.8, 0.8, 0 } } }
            }
        },
        {
            .type = SPHERE,
            .value = { .sphere = { .center = { 1, 0.3, -0.9 }, .radius = 0.5 } },
            .material = {
                .type = METAL,
                .value = { .metal = { .albedo = { 0.8, 0.6, 0.2 }, .fuzz = 0 } }
            }
        },
        {
            .type = SPHERE,
            .value = { .sphere = { .center = { -1, 0.3, -0.9}, .radius = 0.5 } },
            .material = {
                .type = METAL,
                .value = { .metal = { .albedo = { 0.8, 0.8, 0.8 }, .fuzz = 0 } }
            }
        }
    };

    float aa_factor = 100;
    float3 col = {0, 0, 0};
    for (int i = 0; i < aa_factor; i++) {
        float x_f = (float) x;
        float y_f = (float) y;
        float u = (x_f + rand()) / width;
        float v = 1 - (y_f + rand()) / height;

        ray r = camera_ray(&cam, u, v);
        col += color(&r, world, 4, 0);
    }

    col /= aa_factor;
    col = half_sqrt(col);

    input.r = col.r;
    input.g = col.g;
    input.b = col.b;
    input.a = 1;

    return rsPackColorTo8888(input);
}
