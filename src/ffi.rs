use ffi::BVH;

#[cxx::bridge(namespace = "tinybvh")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("tinybvh-rs/ffi/include/tinybvh.h");

        pub type BVH;
        pub type bvhvec4;

        pub fn new_bvh() -> UniquePtr<BVH>;

        pub unsafe fn Build(self: Pin<&mut BVH>, vertices: *const bvhvec4, prim_count: u32);

        pub fn SAHCost(self: &BVH, node_idx: u32) -> f32;
        // pub fn node_count(const BVHLayout layout) -> i32;
	    pub fn PrimCount(self: &BVH, node_idx: u32) -> i32;
    }
}
