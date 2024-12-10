#define TINYBVH_IMPLEMENTATION
#include "tinybvh-rs/ffi/include/tinybvh.h"

namespace tinybvh {

/** Wald BVH */

std::unique_ptr<BVH> new_bvh() { return std::make_unique<BVH>(); }
const BVH::BVHNode* bvh_nodes(const BVH& bvh) { return bvh.bvhNode; }
unsigned bvh_nodes_count(const BVH& bvh) { return bvh.usedNodes; }

/** BVH4 */

std::unique_ptr<BVH4> new_bvh4() { return std::make_unique<BVH4>(); }
const BVHNode4* bvh4_nodes(const BVH4& bvh) { return bvh.bvh4Node; }
unsigned bvh4_nodes_count(const BVH4& bvh) { return bvh.usedNodes; }

/** BVH8 */

std::unique_ptr<BVH8> new_bvh8() { return std::make_unique<BVH8>(); }
const BVH8::BVHNode* bvh_nodes(const BVH8& bvh) { return bvh.bvh8Node; }
unsigned bvh8_nodes_count(const BVH8& bvh) { return bvh.usedNodes; }

}
