mod cwbvh;
mod wald;
pub use cwbvh::*;
pub use wald::*;

/// Holds BVH data without lifetfime bound.
///
/// This is safe because the BHV canno't be used while captured.
pub struct Capture<T> {
    inner: T,
}

/// Implement shared BVH layout.
///
/// - Temporarily move the BVH to edit the triangles
/// - `update()`
macro_rules! impl_bvh {
    ($name: ident, $ffi_name: ident) => {
        impl<'a> $name<'a> {
            /// Create a new BVH from positions.
            ///
            /// # Notes
            ///
            /// The `primitives` slice must contain 3 positions per primitive.
            pub fn new(primitives: &'a [[f32; 4]]) -> Self {
                Self::new_internal().build(primitives)
            }

            /// Create a new BVH from a strided slice of positions.
            ///
            /// # Notes
            ///
            /// The `primitives` slice must contain 3 positions per primitive.
            #[cfg(feature = "strided")]
            pub fn new_strided(primitives: &pas::Slice<[f32; 4]>) -> Self {
                Self::new_internal().build_strided(primitives)
            }

            /// Create the BVH from a capture.
            ///
            /// At the opposite of [`$name:new`], this method might not re-allocate
            /// the BVH data, and instead re-use the captured ones.
            pub fn from_capture(
                capture: crate::Capture<cxx::UniquePtr<ffi::$ffi_name>>,
                primitives: &'a [[f32; 4]],
            ) -> Self {
                Self {
                    inner: capture.inner,
                    _phantom: PhantomData,
                }
                .build(primitives)
            }

            /// Rebuild the BVH layout.
            ///
            /// For complex BVH types, this can result in multiple builds.
            pub fn build(mut self, primitives: &'a [[f32; 4]]) -> Self {
                if primitives.len() % 3 != 0 {
                    panic!("primitives slice must triangulated (size multiple of 3)")
                }
                self.inner.pin_mut().Build(&primitives.into());
                Self {
                    inner: self.inner,
                    _phantom: PhantomData,
                }
            }

            /// Rebuild the BVH layout.
            ///
            /// At the opposite of [`$name::build`], uses a strided primitives slice.
            #[cfg(feature = "strided")]
            pub fn build_strided(mut self, primitives: &pas::Slice<[f32; 4]>) -> Self {
                if primitives.len() % 3 != 0 {
                    panic!("primitives slice must triangulated (size multiple of 3)")
                }
                self.inner.pin_mut().Build(&primitives.into());
                Self {
                    inner: self.inner,
                    _phantom: PhantomData,
                }
            }

            /// Temporarily move the BVH to loosen the primitives lifetime.
            ///
            /// Useful if editing the primitives is required, without re-allocating
            /// the entire BVH.
            ///
            /// # Examples
            ///
            /// ```
            /// use tinybvh_rs::BVH;
            ///
            /// let mut triangles = vec![
            ///     [-1.0, 1.0, 0.0, 0.0],
            ///     [1.0, 1.0, 0.0, 0.0],
            ///     [-1.0, 0.0, 0.0, 0.0]
            /// ];
            /// let bvh = BVH::new(&triangles);
            /// let capture = bvh.capture();
            /// triangles[0][0] = -10.0;
            /// let bvh = BVH::from_capture(capture, &triangles);
            /// ```
            pub fn capture(self) -> crate::Capture<cxx::UniquePtr<ffi::$ffi_name>> {
                crate::Capture { inner: self.inner }
            }
        }
    };
}
pub(super) use impl_bvh;
