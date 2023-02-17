//! Numeric traits for vectors and their components.
//!
//! # Examples
//!
//! ## Components
//!
//! ```
//! use ::ndvec::*;
//!
//! pub fn check_add<T: Component>(c1: T, c2: T, c3: T, c4: T)
//! where
//!     for<'a> &'a T: RefComponent<T>,
//! {
//!     let v1 = Vec2D::new(c1, c2);
//!     let v2 = Vec2D::new(c3, c4);
//!     let sum = v1 + &v2;
//!     assert_eq!(sum.x, c1 + c3);
//!     assert_eq!(sum.y, &c2 + c4);
//! }
//!
//! check_add(1f64, 2f64, 3f64, 4f64);
//! check_add(2f32, -4f32, 6f32, -8f32);
//! ```
//!
//! ## Vectors
//!
//! ```
//! use ::ndvec::*;
//!
//! pub fn check_sub<V: Vector>(v1: V, v2: V, v3: V)
//! where
//!     for<'l> &'l V: VectorRefOps<V::Cmp, V>,
//! {
//!     assert_eq!(v1 - v2, v3);
//!     assert_eq!(v1 - &v2, v3);
//!     assert_eq!(&v1 - v2, v3);
//!     assert_eq!(&v1 - &v2, v3);
//! }
//!
//! check_sub(
//!     Vec2D::new(5f64, 2f64),
//!     Vec2D::new(4f64, 4f64),
//!     Vec2D::new(1f64, -2f64)
//! );
//! check_sub(
//!     Vec3D::new(5f32, 2f32, -10f32),
//!     Vec3D::new(4f32, 4f32, -2f32),
//!     Vec3D::new(1f32, -2f32, -8f32)
//! );
//! ```
//!

use std::fmt::Debug;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use num_traits::Float;
use num_traits::NumAssignRef;
use num_traits::RefNum;

/// Generic trait for floating point numbers as components of vectors.
pub trait Component: Debug + Display + Sum + Float + RefNum<Self> + NumAssignRef {}

impl<T: Debug + Display + Sum + Float + RefNum<Self> + NumAssignRef> Component for T {}

/// Means supports mathematical operators.
///
/// This trait is designed for references of [`Component`].
pub trait RefComponent<Base>: Neg<Output = Base> + RefNum<Base> {}

impl<T, Base> RefComponent<Base> for T where T: Neg<Output = Base> + RefNum<Base> {}

/// Basic operators for vectors.
pub trait VectorOps<Cmp, Base, Rhs>:
    Neg<Output = Base>
    + Add<Rhs, Output = Base>
    + Sub<Rhs, Output = Base>
    + Mul<Cmp, Output = Base>
    + Div<Cmp, Output = Base>
{
}

impl<T, Cmp, Base, Rhs> VectorOps<Cmp, Base, Rhs> for T where
    T: Neg<Output = Base>
        + Add<Rhs, Output = Base>
        + Sub<Rhs, Output = Base>
        + Mul<Cmp, Output = Base>
        + Div<Cmp, Output = Base>
{
}

/// Basic operators with vectors and their references.
///
/// This trait is designed for references of [`Vector`](crate::Vector).
pub trait VectorRefOps<Cmp, Base>:
    VectorOps<Cmp, Base, Base> + for<'r> VectorOps<&'r Cmp, Base, &'r Base>
{
}

impl<T, Cmp, Base> VectorRefOps<Cmp, Base> for T where
    T: VectorOps<Cmp, Base, Base> + for<'r> VectorOps<&'r Cmp, Base, &'r Base>
{
}

/// Assigning operators for vectors.
pub trait VectorAssignOps<Cmp, Rhs>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Cmp> + DivAssign<Cmp>
{
}

impl<T, Cmp, Rhs> VectorAssignOps<Cmp, Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Cmp> + DivAssign<Cmp>
{
}

/// Assigning operators for vectors with references.
pub trait VectorRefAssignOps<Cmp, Base>:
    VectorAssignOps<Cmp, Base> + for<'r> VectorAssignOps<&'r Cmp, &'r Base>
{
}

impl<T, Cmp, Base> VectorRefAssignOps<Cmp, Base> for T where
    T: VectorAssignOps<Cmp, Base> + for<'r> VectorAssignOps<&'r Cmp, &'r Base>
{
}
