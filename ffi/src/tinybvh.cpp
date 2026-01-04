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

std::unique_ptr<BVH> BVH_new() {
    return std::make_unique<BVH>();
}
void BVH_setPrimitives(BVH& out, const bvhvec4slice& primitives) {
    out.verts = primitives;
}
rust::Slice<const BVHNode> BVH_nodes(const BVH& bvh) {
    return rust::Slice{const_cast<const BVHNode*>(bvh.bvhNode), bvh.usedNodes};
}
rust::Slice<const uint32_t> BVH_indices(const BVH& bvh) {
    return rust::Slice{const_cast<const uint32_t*>(bvh.primIdx), bvh.triCount};
}

/** MBVH8 */

std::unique_ptr<MBVH8> MBVH8_new() {
    return std::make_unique<MBVH8>();
}
void MBVH8_setBVH(MBVH8& out, const BVH& bvh) {
    out.bvh = bvh;
}
const uint8_t* MBVH8_nodes(const MBVH8& bvh) {
    return reinterpret_cast<const uint8_t*>(bvh.mbvhNode);
}
uint32_t MBVH8_nodes_count(const MBVH8& bvh) {
    return bvh.usedNodes;
}

/** BVH8_CPU */

std::unique_ptr<BVH8_CPU> BVH8_CPU_new() {
    return std::make_unique<BVH8_CPU>();
}
void BVH8_CPU_setBVH(BVH8_CPU& out, const MBVH8& bvh) {
    out.bvh8 = bvh;
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
