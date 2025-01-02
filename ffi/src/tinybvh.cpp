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

std::unique_ptr<BVH> BVH_new() { return std::make_unique<BVH>(); }
rust::Slice<const BVHNode> BVH_nodes(const BVH& bvh) {
    return rust::Slice{const_cast<const BVHNode*>(bvh.bvhNode), bvh.usedNodes};
}
rust::Slice<const uint32_t> BVH_indices(const BVH& bvh) {
    return rust::Slice{const_cast<const uint32_t*>(bvh.triIdx), bvh.idxCount};
}

/** CWBVH */

std::unique_ptr<BVH8_CWBVH> CWBVH_new() { return std::make_unique<BVH8_CWBVH>(); }
const uint8_t* CWBVH_nodes(const BVH8_CWBVH& bvh) { return reinterpret_cast<const uint8_t*>(bvh.bvh8Data); }
uint32_t CWBVH_nodes_count(const BVH8_CWBVH& bvh) {
    /* tinybvh `usedBlocks` is the number of `vec4`, **not** the number of nodes. */
    return bvh.usedBlocks / 5;
}
const uint8_t* CWBVH_primitives(const BVH8_CWBVH& bvh) { return reinterpret_cast<const uint8_t*>(bvh.bvh8Tris); }
uint32_t CWBVH_primitives_count(const BVH8_CWBVH& bvh) { return bvh.idxCount; }

}
