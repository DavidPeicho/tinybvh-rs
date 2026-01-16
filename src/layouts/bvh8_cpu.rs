use crate::{ffi, mbvh};

/// Read-write BVH.
///
/// At the opposite of [`wald::BVH`] and [`mbvh::BVH`], has no lifetime bound
/// because it manages its own primitives,
pub struct BVH {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH8_CPU>,
}

impl BVH {
    /// Create a new BVH converting `original`.
    pub fn new(original: &mbvh::BVH) -> Self {
        Self {
            inner: ffi::BVH8_CPU_new(),
        }
        .convert(original)
    }

    /// Convert (i.e., build) the BVH and bind it to `original`.
    ///
    /// More information on the tinybvh repository (`BVH8_CPU::ConvertFrom()` method).
    pub fn convert(mut self, original: &mbvh::BVH) -> BVH {
        self.inner
            .pin_mut()
            .ConvertFrom(original.inner.as_ref().unwrap());
        self
    }
}

#[cfg(target_feature = "avx2")]
impl crate::Intersector for BVHData {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
