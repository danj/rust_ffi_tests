#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct both_r {
  int32_t product;
  int32_t sum;
};

extern "C" {

both_r both_r(int32_t a, int32_t b);

} // extern "C"
