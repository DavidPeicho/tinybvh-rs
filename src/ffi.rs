#[cxx::bridge(namespace = "tinybvh")]
pub(crate) mod ffi {

    #[repr(u8)]
	enum BVHLayout{
		WALD_32BYTE = 1,
		AILA_LAINE,
		ALT_SOA,
		VERBOSE,
		BASIC_BVH4,
		BVH4_GPU,
		BVH4_AFRA,
		BASIC_BVH8,
		CWBVH
	}

    unsafe extern "C++" {
        include!("tinybvh-rs/ffi/include/tinybvh.h");

        pub type BVH;
        pub type bvhvec4;
        pub type BVHLayout;
		pub type BVHNode;

        pub fn new_bvh() -> UniquePtr<BVH>;

        pub unsafe fn Build(self: Pin<&mut BVH>, vertices: *const bvhvec4, prim_count: u32);
    	pub fn Compact(self: Pin<&mut BVH>, layout: BVHLayout);
    	pub fn Convert(self: Pin<&mut BVH>, from: BVHLayout, to: BVHLayout, deleteOriginal: bool);

        pub fn NodeCount(self: &BVH, layout: BVHLayout) -> i32;

        pub fn SAHCost(self: &BVH, node_idx: u32) -> f32;
        // pub fn node_count(const BVHLayout layout) -> i32;
	    pub fn PrimCount(self: &BVH, node_idx: u32) -> i32;

		pub fn bvh_nodes(bvh: &BVH) -> *const BVHNode;
		pub fn bvh_nodes_count(bvh: &BVH) -> u32;
    }

}
