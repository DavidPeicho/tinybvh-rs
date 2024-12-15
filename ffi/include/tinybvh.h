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
std::unique_ptr<BVH> new_bvh();
rust::Slice<const BVHNode> bvh_nodes(const BVH&);

/* BVH4 */

using BVHNode4 = BVH4::BVHNode;
std::unique_ptr<BVH4> new_bvh4();
rust::Slice<const BVHNode4> bvh4_nodes(const BVH4&);

}

#endif
