#define TINYBVH_IMPLEMENTATION
#include "tinybvh-rs/ffi/include/tinybvh.h"

namespace tinybvh {

std::unique_ptr<BVH> new_bvh() {
    return std::make_unique<BVH>();
}
const BVH::BVHNode* bvh_nodes(const BVH& bvh) { return bvh.bvhNode; }
unsigned bvh_nodes_count(const BVH& bvh) { return bvh.usedBVHNodes; }

}
