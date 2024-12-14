#define TINYBVH_IMPLEMENTATION
#include "tinybvh-rs/ffi/include/tinybvh.h"

namespace tinybvh {

/** Utils */

Ray ray_new(const std::array<float, 3>& origin, const std::array<float, 3>& dir) {
    bvhvec3 o{origin[0], origin[1], origin[2]};
    bvhvec3 d{dir[0], dir[1], dir[2]};
    return tinybvh::Ray{o, d};
}

/** Wald BVH */

std::unique_ptr<BVH> new_bvh() { return std::make_unique<BVH>(); }
rust::Slice<const BVHNode> bvh_nodes(const BVH& bvh) {
    return rust::Slice{const_cast<const BVHNode*>(bvh.bvhNode), bvh.usedNodes};
}

/** BVH4 */

std::unique_ptr<BVH4> new_bvh4() { return std::make_unique<BVH4>(); }
rust::Slice<const BVH4::BVHNode> bvh_nodes(const BVH4& bvh) {
    return rust::Slice{const_cast<const BVH4::BVHNode*>(bvh.bvh4Node), bvh.usedNodes};
}

/** BVH8 */

std::unique_ptr<BVH8> new_bvh8() { return std::make_unique<BVH8>(); }
const BVH8::BVHNode* bvh_nodes(const BVH8& bvh) { return bvh.bvh8Node; }
unsigned bvh8_nodes_count(const BVH8& bvh) { return bvh.usedNodes; }

}
