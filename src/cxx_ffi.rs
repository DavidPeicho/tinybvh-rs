#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4Slice {
    data: *const i32,
    count: u32,
    stride: u32,
}

impl From<&[[f32; 4]]> for Vec4Slice {
    fn from(value: &[[f32; 4]]) -> Self {
        Self {
            data: value.as_ptr() as *const i32,
            count: value.len() as u32,
            stride: std::mem::size_of::<[f32; 4]>() as u32,
        }
    }
}

// Ensure `bvhvec4slice` always has a trivial move ctor and no destructor
unsafe impl cxx::ExternType for Vec4Slice {
    type Id = cxx::type_id!("tinybvh::bvhvec4slice");
    type Kind = cxx::kind::Trivial;
}
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
// Ensure `BVH::BVHNode` always has a trivial move ctor and no destructor
unsafe impl cxx::ExternType for crate::NodeWald {
    type Id = cxx::type_id!("tinybvh::BVHNode");
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

        // Utils
        pub type bvhvec4slice = super::Vec4Slice;
        pub type Ray = crate::Ray;
        pub fn ray_new(origin: &[f32; 3], dir: &[f32; 3]) -> Ray;

        // BVH
        pub type BVH;
        pub type BVHNode = crate::NodeWald;
        pub fn new_bvh() -> UniquePtr<BVH>;
        pub fn Build(self: Pin<&mut BVH>, primitives: &bvhvec4slice);
        pub fn Compact(self: Pin<&mut BVH>);
        pub fn SAHCost(self: &BVH, node_idx: u32) -> f32;
        pub fn PrimCount(self: &BVH, node_idx: u32) -> i32;
        pub fn bvh_nodes(bvh: &BVH) -> &[BVHNode];
        pub fn Intersect(self: &BVH, original: &mut Ray) -> i32;

        // BVH4
        pub type BVH4;
        pub type BVHNode4 = crate::Node4;
        pub fn new_bvh4() -> UniquePtr<BVH4>;
        pub fn Build(self: Pin<&mut BVH4>, primitives: &bvhvec4slice);
        pub fn bvh4_nodes(bvh: &BVH4) -> &[BVHNode4];
        pub fn Intersect(self: &BVH4, original: &mut Ray) -> i32;

        // CWBVH
        pub type BVH8_CWBVH;
        pub fn cwbvh_new() -> UniquePtr<BVH8_CWBVH>;
        pub fn cwbvh_nodes(bvh: &BVH8_CWBVH) -> *const u8;
        pub fn cwbvh_nodes_count(bvh: &BVH8_CWBVH) -> u32;
        pub fn Build(self: Pin<&mut BVH8_CWBVH>, primitives: &bvhvec4slice);
        pub fn Intersect(self: &BVH8_CWBVH, original: &mut Ray) -> i32;
    }
}
