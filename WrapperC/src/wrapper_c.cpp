#include "wrapper_c.h"
#include "core_c.h"
#include "core_r.h"

both_c_t both_c(int a, int b) {
    return {
        add(a, b),
        product(a, b)
    };
}