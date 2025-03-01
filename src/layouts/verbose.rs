use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

use super::wald;

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
    /// AABB min position.
    pub min: [f32; 3],
    pub left: u32,
    /// AABB max position.
    pub max: [f32; 3],
    pub right: u32,
    pub tri_count: u32,
    pub first_tri: u32,
    pub parent: u32,
}

impl Node {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

pub struct OptimizeOptions {
    iterations: u32,
    extreme: bool,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            iterations: 25,
            extreme: false,
        }
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
pub struct BVH<'a> {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH_Verbose>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH<'a> {
    pub fn build(original: &wald::BVH) -> Self {
        let mut verbose = Self {
            inner: ffi::BVH_Verbose_new(),
            _phantom: PhantomData,
        };
        verbose.convert_from(original)
    }

    pub fn convert_from(mut self, original: &wald::BVH) -> Self {
        // Second parameter is not used in tinybvh. Defaults to the same value as the header.
        self.inner.pin_mut().ConvertFrom(&original.inner, true);
        Self {
            inner: self.inner,
            _phantom: PhantomData,
        }
    }

    pub fn refit(&mut self, skip_leaf: bool) {
        self.inner.pin_mut().Refit(0 as u32, skip_leaf);
    }

    pub fn refit_node(&mut self, node: u32, skip_leaf: bool) {
        self.inner.pin_mut().Refit(node, skip_leaf);
    }

    pub fn optimize(&mut self, opts: &OptimizeOptions) {
        self.inner.pin_mut().Optimize(opts.iterations, opts.extreme);
    }
}
