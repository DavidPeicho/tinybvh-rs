#ifndef TEST
#define TEST

#include <memory>

#include "tinybvh-rs/ffi/tinybvh/tiny_bvh.h"

namespace tinybvh {

std::unique_ptr<BVH> new_bvh();

// void delete_bvh(std::unique_ptr<BVH>& ptr);

}

#endif
