#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4Slice {
    data: *const i32,
    count: u32,
    stride: u32,
}

impl From<pas::Slice<'_, [f32; 4]>> for Vec4Slice {
    fn from(value: pas::Slice<[f32; 4]>) -> Self {
        Self {
            data: value.as_ptr() as *const i32,
            count: value.len() as u32,
            stride: value.stride() as u32,
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
unsafe impl cxx::ExternType for crate::wald::Node {
    type Id = cxx::type_id!("tinybvh::BVHNode");
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
        pub type BVHNode = crate::wald::Node;
        pub fn BVH_new() -> UniquePtr<BVH>;
        pub fn BVH_nodes(bvh: &BVH) -> &[BVHNode];
        pub fn BVH_indices(bvh: &BVH) -> &[u32];
        pub fn Build(self: Pin<&mut BVH>, primitives: &bvhvec4slice);
        pub fn BuildHQ(self: Pin<&mut BVH>, primitives: &bvhvec4slice);
        pub fn Compact(self: Pin<&mut BVH>);
        pub fn ConvertFrom(self: Pin<&mut BVH>, original: &BVH_Verbose, compact: bool);
        pub fn SAHCost(self: &BVH, node_idx: u32) -> f32;
        pub fn PrimCount(self: &BVH, node_idx: u32) -> i32;
        pub fn Intersect(self: &BVH, original: &mut Ray) -> i32;

        // CWBVH
        pub type BVH8_CWBVH;
        pub fn CWBVH_new() -> UniquePtr<BVH8_CWBVH>;
        pub fn CWBVH_nodes(bvh: &BVH8_CWBVH) -> *const u8;
        pub fn CWBVH_nodes_count(bvh: &BVH8_CWBVH) -> u32;
        pub fn CWBVH_primitives(bvh: &BVH8_CWBVH) -> *const u8;
        pub fn CWBVH_primitives_count(bvh: &BVH8_CWBVH) -> u32;
        pub fn Build(self: Pin<&mut BVH8_CWBVH>, primitives: &bvhvec4slice);
        pub fn BuildHQ(self: Pin<&mut BVH8_CWBVH>, primitives: &bvhvec4slice);
        pub fn Intersect(self: &BVH8_CWBVH, original: &mut Ray) -> i32;

        // Verbose
        pub type BVH_Verbose;
        pub fn BVH_Verbose_new() -> UniquePtr<BVH_Verbose>;
        pub fn ConvertFrom(self: Pin<&mut BVH_Verbose>, original: &BVH, compact: bool);
        pub fn Refit(self: Pin<&mut BVH_Verbose>, node: u32, skip_leaf: bool);
        pub fn Optimize(self: Pin<&mut BVH_Verbose>, iterations: u32, extreme: bool);
    }
}
