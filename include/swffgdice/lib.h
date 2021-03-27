#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace swffgdice {

struct Face {
    int32_t success;
    int32_t advantage;
    int32_t triumph;
    int32_t failure;
    int32_t threat;
    int32_t despair;
    int32_t light;
    int32_t dark;
};


extern "C" {

Face roll(const char *arg);

} // extern "C"

} // namespace swffgdice
