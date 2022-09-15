#ifndef FFI_TESTS_WRAPPER_C_H
#define FFI_TESTS_WRAPPER_C_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int sum;
    int product;
} both_c_t;

both_c_t both_c(int a, int b);

#ifdef __cplusplus
}
#endif


#endif //FFI_TESTS_WRAPPER_C_H
