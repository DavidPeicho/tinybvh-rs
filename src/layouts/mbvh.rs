use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

use super::wald;

/// BVH8 with node layout [`Node`].
pub struct BVH8<'a> {
    pub(crate) inner: cxx::UniquePtr<ffi::MBVH8>,
    _phantom: PhantomData<&'a wald::BVH<'a>>,
}

impl<'a> BVH8<'a> {
    pub fn from(original: &wald::BVH) -> Self {
        Self {
            inner: ffi::MBVH8_new(),
            _phantom: PhantomData,
        }
        .convert_from(original)
    }

    pub fn convert_from(mut self, original: &wald::BVH) -> Self {
        self.inner.pin_mut().ConvertFrom(&original.inner, true);
        self
    }
}
// super::impl_bvh!(BVH8, MBVH8);
