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
rust::Slice<const uint32_t> bvh_indices(const BVH& bvh) {
    return rust::Slice{const_cast<const uint32_t*>(bvh.triIdx), bvh.idxCount};
}

/** BVH4 */

std::unique_ptr<BVH4> new_bvh4() { return std::make_unique<BVH4>(); }
rust::Slice<const BVH4::BVHNode> bvh4_nodes(const BVH4& bvh) {
    return rust::Slice{const_cast<const BVH4::BVHNode*>(bvh.bvh4Node), bvh.usedNodes};
}
rust::Slice<const uint32_t> bvh4_indices(const BVH4& bvh) {
    return rust::Slice{const_cast<const uint32_t*>(bvh.bvh.triIdx), bvh.bvh.idxCount};
}

/** CWBVH */

std::unique_ptr<BVH8_CWBVH> cwbvh_new() { return std::make_unique<BVH8_CWBVH>(); }
const uint8_t* cwbvh_nodes(const BVH8_CWBVH& bvh) { return reinterpret_cast<const uint8_t*>(bvh.bvh8Data); }
uint32_t cwbvh_nodes_count(const BVH8_CWBVH& bvh) { return bvh.usedBlocks; }
const uint8_t* cwbvh_primitives(const BVH8_CWBVH& bvh) { return reinterpret_cast<const uint8_t*>(bvh.bvh8Tris); }
uint32_t cwbvh_primitives_count(const BVH8_CWBVH& bvh) { return bvh.idxCount; }

}
