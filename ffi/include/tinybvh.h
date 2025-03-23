#ifndef TINYBVH_RUST
#define TINYBVH_RUST

#include <array>
#include <memory>

#include "rust/cxx.h"
#include "tinybvh-rs/ffi/tinybvh/tiny_bvh.h"

namespace tinybvh {

/* Math */
Ray ray_new(const std::array<float, 3>& origin, const std::array<float, 3>& dir);

/* BVH Wald 32 */

using BVHNode = BVH::BVHNode;
std::unique_ptr<BVH> BVH_new();
rust::Slice<const BVHNode> BVH_nodes(const BVH&);
rust::Slice<const uint32_t> BVH_indices(const BVH&);

/* MBVH<8> */
template class MBVH<8>;
using MBVH8 = MBVH<8>;
std::unique_ptr<MBVH8> MBVH8_new();

/* CWBVH */

struct NodeCWBVH; // TODO: Remove once tinybvh provides a struct for CWBVH node.

std::unique_ptr<BVH8_CWBVH> CWBVH_new();
const uint8_t* CWBVH_nodes(const BVH8_CWBVH&);
uint32_t CWBVH_nodes_count(const BVH8_CWBVH&);
const uint8_t* CWBVH_primitives(const BVH8_CWBVH&);
uint32_t CWBVH_primitives_count(const BVH8_CWBVH&);

/* Verbose */

using BVHNode = BVH::BVHNode;
std::unique_ptr<BVH_Verbose> BVH_Verbose_new();


}

#endif
