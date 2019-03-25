#pragma version(1)
#pragma rs java_package_name(com.shika.rayrender)

uchar4 RS_KERNEL hello_world(uchar4 in, rs_kernel_context context, int32_t x, int32_t y) {
    float4 input = rsUnpackColor8888(in);
    int32_t width = rsGetDimX(context);
    int32_t height = rsGetDimY(context);

    input.r = ((float) x) / width;
    input.g = ((float) y) / height;
    input.b = 0.2;
    input.a = 1;

    return rsPackColorTo8888(input);
}
