pub mod cwbvh;
pub mod mbvh;
pub mod verbose;
pub mod wald;

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
            /// Create the BVH from a capture.
            ///
            /// At the opposite of [`$name:new`], this method might not re-allocate
            /// the BVH data, and instead re-use the captured ones.
            pub fn from_capture<S: Into<crate::Positions<'a>>>(
                capture: crate::Capture<cxx::UniquePtr<ffi::$ffi_name>>,
                primitives: S,
            ) -> Self {
                Self {
                    inner: capture.inner,
                    _phantom: PhantomData,
                }
                .build(primitives)
            }

            /// Temporarily move the BVH to loosen the primitives lifetime.
            ///
            /// Useful if editing the primitives is required, without re-allocating
            /// the entire BVH.
            ///
            /// # Examples
            ///
            /// ```
            /// use tinybvh_rs::wald::BVH;
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
