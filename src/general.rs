#[cfg(test)]
mod tests;

use std::fmt::Debug;
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
use num_traits::Zero;
#[cfg(feature = "serde_arrays")]
use serde::Deserialize;
#[cfg(feature = "serde_arrays")]
use serde::Serialize;

use crate::array::arr_zip_map;
use crate::Component;
use crate::Vector;

/// General-fixed-size low-dimensional vector.
///
/// Does not any heap allocation.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_arrays", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde_arrays",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct VecND<T, const N: usize> {
    #[cfg_attr(feature = "serde_arrays", serde(with = "serde_arrays"))]
    arr: [T; N],
}

impl<T: Component, const N: usize> Vector for VecND<T, N> {
    type Cmp = T;

    const DIM: usize = N;

    fn norm_sqr(self) -> Self::Cmp {
        self.arr.into_iter().map(|c| c * c).sum::<T>()
    }

    fn dot(self, rhs: Self) -> Self::Cmp {
        self.arr.into_iter().zip(rhs.arr).map(|(a, b)| a * b).sum()
    }

    fn is_finite(self) -> bool {
        self.arr.into_iter().all(|c| c.is_finite())
    }

    fn has_nan(self) -> bool {
        self.arr.into_iter().any(|c| c.is_nan())
    }
}

impl<T: Copy + Debug, const N: usize> Debug for VecND<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.arr.iter()).finish()
    }
}

impl<T: Copy + Zero, const N: usize> Zero for VecND<T, N> {
    fn zero() -> Self {
        Self {
            arr: [T::zero(); N],
        }
    }

    fn is_zero(&self) -> bool {
        self.arr.iter().all(|c| c.is_zero())
    }
}

impl<T: Copy + Neg, const N: usize> Neg for VecND<T, N> {
    type Output = VecND<T::Output, N>;

    #[inline]
    fn neg(self) -> Self::Output {
        VecND {
            arr: self.arr.map(|v| -v),
        }
    }
}

impl<T: Copy + Neg, const N: usize> Neg for &VecND<T, N> {
    type Output = VecND<T::Output, N>;

    #[inline]
    fn neg(self) -> Self::Output {
        VecND {
            arr: self.arr.map(|v| -v),
        }
    }
}

macro_rules! impl_vec_vec_op {
    ($($LB:ident)?; $($RB:ident)?; $Op:ident, $op:ident, $L:ty, $R:ty, $sym:tt) => {
        impl<T: $Op<S>$(+ $LB)? , S$(: $RB)?, const N: usize> $Op<$R> for $L {
            type Output = VecND<T::Output, N>;

            #[inline]
            fn $op(self, rhs: $R) -> Self::Output {
                VecND {
                    arr: arr_zip_map(self.arr, rhs.arr, |a, b| a $sym b),
                }
            }
        }
    };
}

impl_vec_vec_op!(; ; Add, add, VecND<T, N>, VecND<S, N>, +);
impl_vec_vec_op!(; Copy; Add, add, VecND<T, N>, &VecND<S, N>, +);
impl_vec_vec_op!(Copy; ; Add, add, &VecND<T, N>, VecND<S, N>, +);
impl_vec_vec_op!(Copy; Copy; Add, add, &VecND<T, N>, &VecND<S, N>, +);

impl_vec_vec_op!(; ; Sub, sub, VecND<T, N>, VecND<S, N>, -);
impl_vec_vec_op!(; Copy; Sub, sub, VecND<T, N>, &VecND<S, N>, -);
impl_vec_vec_op!(Copy; ; Sub, sub, &VecND<T, N>, VecND<S, N>, -);
impl_vec_vec_op!(Copy; Copy; Sub, sub, &VecND<T, N>, &VecND<S, N>, -);

macro_rules! impl_vec_num_op {
    ($($LB:ident)?; $Op:ident, $op:ident, $L:ty, $R:ty, $sym:tt) => {
        impl<T: $Op<S>$(+ $LB)? , S: Copy, const N: usize> $Op<$R> for $L {
            type Output = VecND<T::Output, N>;

            #[inline]
            fn $op(self, rhs: $R) -> Self::Output {
                VecND {
                    arr: self.arr.map(|l| l $sym rhs)
                }
            }
        }
    };
}

impl_vec_num_op!(; Mul, mul, VecND<T, N>, S, *);
impl_vec_num_op!(Copy; Mul, mul, &VecND<T, N>, S, *);

impl_vec_num_op!(; Div, div, VecND<T, N>, S, /);
impl_vec_num_op!(Copy; Div, div, &VecND<T, N>, S, /);

macro_rules! impl_mul_num_vec {
    ($L:ty, $R:ty, $O:ty) => {
        impl<const N: usize> Mul<$R> for $L {
            type Output = $O;

            #[inline]
            fn mul(self, rhs: $R) -> Self::Output {
                VecND {
                    arr: rhs.arr.map(|r| self * r),
                }
            }
        }
    };
}

impl_mul_num_vec!(f64, VecND<f64, N>, VecND<f64, N>);
impl_mul_num_vec!(f64, &VecND<f64, N>, VecND<f64, N>);
impl_mul_num_vec!(&f64, VecND<f64, N>, VecND<f64, N>);
impl_mul_num_vec!(&f64, &VecND<f64, N>, VecND<f64, N>);

impl_mul_num_vec!(f32, VecND<f32, N>, VecND<f32, N>);
impl_mul_num_vec!(f32, &VecND<f32, N>, VecND<f32, N>);
impl_mul_num_vec!(&f32, VecND<f32, N>, VecND<f32, N>);
impl_mul_num_vec!(&f32, &VecND<f32, N>, VecND<f32, N>);

macro_rules! impl_assign_vec {
    ($($RB:ident)?; $Op:ident, $op:ident, $R:ty, $sym:tt) => {
        impl<T: $Op<S>, S$(: $RB)?, const N: usize> $Op<$R> for VecND<T, N> {
            #[inline]
            fn $op(&mut self, rhs: $R) {
                for (l, r) in self.arr.iter_mut().zip(rhs.arr.into_iter()) {
                    *l $sym r;
                }
            }
        }
    };
}

impl_assign_vec!(; AddAssign, add_assign, VecND<S, N>, +=);
impl_assign_vec!(Copy; AddAssign, add_assign, &VecND<S, N>, +=);
impl_assign_vec!(; SubAssign, sub_assign, VecND<S, N>, -=);
impl_assign_vec!(Copy; SubAssign, sub_assign, &VecND<S, N>, -=);

impl<T: MulAssign<S>, S: Copy, const N: usize> MulAssign<S> for VecND<T, N> {
    #[inline]
    fn mul_assign(&mut self, rhs: S) {
        for c in self.arr.iter_mut() {
            *c *= rhs;
        }
    }
}

impl<T: DivAssign<S>, S: Copy, const N: usize> DivAssign<S> for VecND<T, N> {
    #[inline]
    fn div_assign(&mut self, rhs: S) {
        for c in self.arr.iter_mut() {
            *c /= rhs;
        }
    }
}

impl<T: Float, const N: usize> Sum for VecND<T, N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(VecND::zero(), |a, b| a + b)
    }
}

impl<T, const N: usize> From<[T; N]> for VecND<T, N> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self { arr }
    }
}

impl<T, const N: usize> From<VecND<T, N>> for [T; N] {
    #[inline]
    fn from(vec: VecND<T, N>) -> Self {
        vec.arr
    }
}

impl<T, const N: usize> AsRef<[T; N]> for VecND<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.arr
    }
}

impl<T, const N: usize> AsMut<[T; N]> for VecND<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.arr
    }
}
