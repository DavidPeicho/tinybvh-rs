#ifndef TEST
#define TEST

#include <memory>

#include "tinybvh-rs/ffi/tinybvh/tiny_bvh.h"

namespace tinybvh {

using BVHLayout = BVH::BVHLayout;
using BVHNode = BVH::BVHNode;

std::unique_ptr<BVH> new_bvh();

const BVHNode* bvh_nodes(const BVH&);
unsigned bvh_nodes_count(const BVH&);

}

#endif
