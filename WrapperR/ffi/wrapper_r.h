#ifndef FFI_TESTS_WRAPPER_R_H
#define FFI_TESTS_WRAPPER_R_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int product;
    int sum;
} both_r_t;

extern both_r_t both_r(int a, int b);

#ifdef __cplusplus
}
#endif

#endif //FFI_TESTS_WRAPPER_R_H
