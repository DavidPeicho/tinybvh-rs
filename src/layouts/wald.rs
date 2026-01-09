use crate::ffi;
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
    /// AABB min position.
    pub min: [f32; 3],
    /// If the node is a leaf, this is the start index of the primitive.
    /// Otherwise, this is the start index of the first child node.
    pub left_first: u32,
    /// AABB max position.
    pub max: [f32; 3],
    /// If the node is a leaf, number of triangles in the node.
    /// `0` otherwise.
    pub tri_count: u32,
}

impl Node {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/// BVH with node layout [`Node`].
///
/// # Examples
///
/// ```
/// use tinybvh_rs::wald;
///
/// let triangles = vec![
///     [-1.0, 1.0, 0.0, 0.0],
///     [1.0, 1.0, 0.0, 0.0],
///     [-1.0, 0.0, 0.0, 0.0]
/// ];
/// let bvh = wald::BVH::new(&triangles);
/// ```
pub struct BVHData {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH>,
    pub(crate) max_primitives_per_leaf: Option<u32>,
}

impl BVHData {
    pub fn builder<'a>(mut self, primitives: crate::Positions<'a>) -> BVH<'a> {
        let slice = primitives.into();
        ffi::BVH_setPrimitives(self.inner.pin_mut(), &slice);
        BVH {
            bvh: self,
            _phantom: PhantomData,
        }
    }

    /// Number of primitives for a given node.
    pub fn primitive_count(&self, id: u32) -> u32 {
        self.inner.PrimCount(id) as u32
    }

    /// SAH cost for a subtree.
    pub fn sah_cost(&self, id: u32) -> f32 {
        self.inner.SAHCost(id)
    }

    /// BVH nodes.
    ///
    /// Useful to upload to the BVH to the GPU.
    pub fn nodes(&self) -> &[Node] {
        ffi::BVH_nodes(&self.inner)
    }

    /// BVH indices.
    ///
    /// Map from primitive index to first vertex index.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for i in 0..node.tri_count {
    ///     let vertex_start = bvh.indices()[node.left_first + i] * 3;
    ///     let vertex = [
    ///         primitives[vertex_start],
    ///         primitives[vertex_start + 1],
    ///         primitives[vertex_start + 2]
    ///     ];
    ///     println!("Vertex {:?}", vertex);
    /// }
    /// ```
    pub fn indices(&self) -> &[u32] {
        ffi::BVH_indices(&self.inner)
    }
}

pub struct BVH<'a> {
    pub(crate) bvh: BVHData,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> Deref for BVH<'a> {
    type Target = BVHData;

    fn deref(&self) -> &Self::Target {
        &self.bvh
    }
}

impl<'a> BVH<'a> {
    pub fn new(primitives: crate::Positions<'a>) -> Self {
        let bvh = BVHData {
            inner: ffi::BVH_new(),
            max_primitives_per_leaf: None,
        };
        bvh.builder(primitives).build(primitives)
    }

    pub fn new_hq(primitives: crate::Positions<'a>) -> Self {
        let data = BVHData {
            inner: ffi::BVH_new(),
            max_primitives_per_leaf: None,
        };
        data.builder(primitives).build_hq(primitives)
    }

    pub fn build(mut self, primitives: crate::Positions<'a>) -> Self {
        if primitives.len() % 3 != 0 {
            panic!("primitives slice must triangulated (size multiple of 3)")
        }
        let slice = primitives.into();
        self.bvh.inner.pin_mut().Build(&slice);
        self.bvh.max_primitives_per_leaf = None;
        self
    }

    pub fn build_hq(mut self, primitives: crate::Positions<'a>) -> Self {
        if primitives.len() % 3 != 0 {
            panic!("primitives slice must triangulated (size multiple of 3)")
        }
        let slice = primitives.into();
        self.bvh.inner.pin_mut().BuildHQ(&slice);
        self.bvh.max_primitives_per_leaf = None;
        self
    }

    // Remove unused nodes and reduce the size of the BVH.
    pub fn compact(&mut self) {
        self.bvh.inner.pin_mut().Compact();
    }

    pub fn split_leaves(&mut self, max_primitives: u32) {
        self.bvh.inner.pin_mut().SplitLeafs(max_primitives);

        let max_prim = self.bvh.max_primitives_per_leaf.unwrap_or(u32::MAX);
        self.bvh.max_primitives_per_leaf = Some(u32::min(max_prim, max_primitives));
    }

    pub fn data(self) -> BVHData {
        self.bvh
    }
}

impl crate::Intersector for BVH<'_> {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.bvh.inner.Intersect(ray) as u32
    }
}
