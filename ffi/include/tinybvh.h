#ifndef TEST
#define TEST

#include <memory>

#include "tinybvh-rs/ffi/tinybvh/tiny_bvh.h"

namespace tinybvh {

using BVHLayout = BVH::BVHLayout;

std::unique_ptr<BVH> new_bvh();

}

#endif
