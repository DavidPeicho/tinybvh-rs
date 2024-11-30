#define TINYBVH_IMPLEMENTATION
#include "tinybvh-rs/ffi/include/tinybvh.h"

namespace tinybvh {

std::unique_ptr<BVH> new_bvh() {
    return std::make_unique<BVH>();
}

// void delete_bvh(std::unique_ptr<BVH>& ptr) {
//     ptr = nullptr;
// }

}
