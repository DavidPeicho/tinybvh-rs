#ifndef TEST
#define TEST

#include <memory>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wall"
#include "tinybvh-rs/ffi/tinybvh/tiny_bvh.h"
#pragma GCC diagnostic pop

namespace tinybvh {

using BVHLayout = BVH::BVHLayout;
using BVHNode = BVH::BVHNode;

std::unique_ptr<BVH> new_bvh();

const BVHNode* bvh_nodes(const BVH&);
unsigned bvh_nodes_count(const BVH&);

}

#endif
