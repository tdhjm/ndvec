//!
//! # ndvec
//!
//! Simple low-dimensional vectors.
//!
//! Supports numerical operators, even with references.
//!
//! This crate intentionally does not support operators for element-wise product and so on.
//!
mod array;
mod fixed;
mod general;
mod ops;

use std::fmt::Debug;
use std::iter::Sum;

use num_traits::Float;
use num_traits::Zero;

pub use crate::fixed::*;
pub use crate::general::*;
pub use crate::ops::*;

/// 1-dimensional vector with 64bit component
pub type V1D64 = Vec1D<f64>;
/// 1-dimensional vector with 32bit component
pub type V1D32 = Vec1D<f32>;
/// 2-dimensional vector with 64bit component
pub type V2D64 = Vec2D<f64>;
/// 2-dimensional vector with 32bit component
pub type V2D32 = Vec2D<f32>;
/// 3-dimensional vector with 64bit component
pub type V3D64 = Vec3D<f64>;
/// 3-dimensional vector with 32bit component
pub type V3D32 = Vec3D<f32>;

/// Fixed-size vector.
pub trait Vector:
    Clone
    + Copy
    + Debug
    + PartialEq
    + Zero
    + Sum
    + VectorRefOps<Self::Cmp, Self>
    + VectorRefAssignOps<Self::Cmp, Self>
where
    for<'l> &'l Self: VectorRefOps<Self::Cmp, Self>,
{
    /// Component scalar type of the vector.
    type Cmp: Component;

    /// Dimension of the vector.
    const DIM: usize;

    /// The square of the L2-norm.
    fn norm_sqr(self) -> Self::Cmp;

    /// The dot product with another vector.
    fn dot(self, rhs: Self) -> Self::Cmp;

    /// Checks if all components are finite.
    fn is_finite(self) -> bool;

    /// Checks if any of the components is NaN.
    fn has_nan(self) -> bool;

    /// The L2-norm.
    #[inline]
    fn norm(self) -> Self::Cmp {
        self.norm_sqr().sqrt()
    }

    /// The euclid distance from the another vector.
    #[inline]
    fn distance(self, rhs: Self) -> Self::Cmp {
        (self - rhs).norm()
    }

    /// The square of the euclid distance from the another vector.
    #[inline]
    fn distance_sqr(self, rhs: Self) -> Self::Cmp {
        (self - rhs).norm_sqr()
    }
}
