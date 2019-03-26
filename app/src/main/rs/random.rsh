uint32_t rand_state[5] = {
    0x6635e5ce, 0x13bf026f, 0x43225b59, 0x3b0314d0, 0x670f485a
};

/* The state array must be initialized to not be all zero in the first four words */
static uint32_t xorwow(uint32_t state[static 5]) {
	/* Algorithm "xorwow" from p. 5 of Marsaglia, "Xorshift RNGs" */
	uint32_t s, t = state[3];
	state[3] = state[2];
	state[2] = state[1];
	state[1] = s = state[0];
	t ^= t >> 2;
	t ^= t << 1;
	state[0] = t ^= s ^ (s << 4);
	return t + (state[4] += 362437);
}

static float rand() {
    return (float) xorwow(rand_state) / 0xffffffff;
}

static float3 rand_in_sphere() {
    float num = rand();
    float x1 = rand() * 2 - 1;
    float x2 = rand() * 2 - 1;
    float x3 = rand() * 2 - 1;
    float multiplier = native_rsqrt(x1 * x1 + x2 * x2 + x3 * x3) * num;

    return multiplier * (float3) { x1, x2, x3 };
}
