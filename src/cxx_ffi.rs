// Ensure `Intersection` always has a trivial move ctor and no destructor
unsafe impl cxx::ExternType for crate::Intersection {
    type Id = cxx::type_id!("tinybvh::Intersection");
    type Kind = cxx::kind::Trivial;
}
// Ensure `Ray` always has a trivial move ctor and no destructor
unsafe impl cxx::ExternType for crate::Ray {
    type Id = cxx::type_id!("tinybvh::Ray");
    type Kind = cxx::kind::Trivial;
}
// Ensure `BVH4::BVHNode` always has a trivial move ctor and no destructor
unsafe impl cxx::ExternType for crate::Node4 {
    type Id = cxx::type_id!("tinybvh::BVHNode4");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "tinybvh")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("tinybvh-rs/ffi/include/tinybvh.h");

        pub type BVH;
        pub type bvhvec4;
        pub type BVHNode;
        pub type Ray = crate::Ray;
        pub type Intersection = crate::Intersection;

        // pub type Ray;

        // BVH
        pub fn new_bvh() -> UniquePtr<BVH>;
        pub unsafe fn Build(self: Pin<&mut BVH>, vertices: *const bvhvec4, prim_count: u32);
        pub fn Compact(self: Pin<&mut BVH>);
        pub fn NodeCount(self: &BVH) -> i32;
        pub fn SAHCost(self: &BVH, node_idx: u32) -> f32;
        pub fn PrimCount(self: &BVH, node_idx: u32) -> i32;
        pub fn bvh_nodes(bvh: &BVH) -> *const BVHNode;
        pub fn bvh_nodes_count(bvh: &BVH) -> u32;

        // BVH4
        pub type BVH4;
        pub type BVHNode4 = crate::Node4;
        pub fn new_bvh4() -> UniquePtr<BVH4>;
        pub fn bvh4_nodes(bvh: &BVH4) -> *const BVHNode4;
        pub fn bvh4_nodes_count(bvh: &BVH4) -> u32;
        pub fn ConvertFrom(self: Pin<&mut BVH4>, original: &BVH);
        pub fn Intersect(self: &BVH4, original: &mut Ray) -> i32;
    }
}
