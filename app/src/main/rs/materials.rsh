#include<objects.rsh>
#include<random.rsh>
#include<hitables.rsh>

typedef enum {
    LAMBERTIAN,
    METAL
} m_type;

typedef struct {
    float3 albedo;
} lambertian;

typedef struct {
    float3 albedo;
    float fuzz;
} metal;

typedef union {
    lambertian lambertian;
    metal metal;
} m_value;

typedef struct {
    m_type type;
    m_value value;
} material;

static bool scatter_lambertian(
    const lambertian *material,
    const record *rec,
    float3 *attenuation,
    ray *r_out
) {
    float3 target = rec->p + rec->normal + rand_in_sphere();
    r_out->origin = rec->p;
    r_out->direction = target - rec->p;
    *attenuation = material->albedo;
    return true;
}

static float3 _reflect(const float3 v, const float3 n) {
    return v - 2 * dot(v, n) * n;
}

static bool scatter_metal(
    const metal *material,
    const ray *r_in,
    const record *rec,
    float3 *attenuation,
    ray *r_out
) {
    float3 reflected = _reflect(normalize(r_in->direction), rec->normal);
    r_out->origin = rec->p;
    r_out->direction = reflected + material->fuzz * rand_in_sphere();
    *attenuation = material->albedo;
    return (dot(reflected, rec->normal) > 0);
}
