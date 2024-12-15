mod bvh4;
mod wald;
pub use bvh4::*;
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
macro_rules! impl_bvh_layout {
    ($name: ident) => {
        impl<'a> $name<'a> {
            pub fn from_capture(
                capture: crate::Capture<cxx::UniquePtr<ffi::$name>>,
                primitives: &'a [[f32; 4]],
            ) -> Self {
                Self {
                    inner: capture.inner,
                    _phantom: PhantomData,
                }
                .build(primitives)
            }

            /// Build the BVH layout.
            ///
            /// For complex BVH types, this can result in multiple builds:
            /// - [`crate::BVH4`]: Requires building a [`crate::BVH`] first
            pub fn build(mut self, primitives: &'a [[f32; 4]]) -> Self {
                let primitives = primitives.into();
                self.inner.pin_mut().Build(&primitives);
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
            pub fn capture(self) -> crate::Capture<cxx::UniquePtr<ffi::$name>> {
                crate::Capture { inner: self.inner }
            }
        }
    };
}
pub(super) use impl_bvh_layout;
