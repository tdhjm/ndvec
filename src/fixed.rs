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
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::Component;
use crate::RefComponent;
use crate::VecND;
use crate::Vector;

macro_rules! declare_vector {
    ($V:ident{$first:ident $(, $rest:ident )*}; $N:literal) => {
        #[doc = concat!($N, "-dimensional vector.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $V<T> {
            pub $first: T,
            $(pub $rest: T,)*
        }

        impl<T> $V<T> {
            #[inline]
            pub const fn new($first: T $(, $rest: T)*) -> Self {
                Self { $first $(, $rest)* }
            }
        }
    };
}

declare_vector!(Vec1D { x }; 1);
declare_vector!(Vec2D { x, y }; 2);
declare_vector!(Vec3D { x, y, z }; 3);

impl<T: Component> Vector for Vec1D<T>
where
    for<'a> &'a T: RefComponent<T>,
{
    type Cmp = T;

    const DIM: usize = 1;

    #[inline]
    fn norm_sqr(self) -> Self::Cmp {
        self.x * self.x
    }

    #[inline]
    fn dot(self, rhs: Self) -> Self::Cmp {
        self.x * rhs.x
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite()
    }

    #[inline]
    fn has_nan(self) -> bool {
        self.x.is_nan()
    }

    #[inline]
    fn norm(self) -> Self::Cmp {
        self.x.abs()
    }
}

macro_rules! impl_vector {
    ($V:ident{$first:ident $(, $rest:ident )*}; $N:literal) => {

        impl<T: Component> Vector for $V<T>
        where
            for<'a> &'a T: RefComponent<T>,
        {
            type Cmp = T;

            const DIM: usize = $N;

            #[inline]
            fn norm_sqr(self) -> Self::Cmp {
                self.$first * self.$first $( + self.$rest * self.$rest )*
            }

            #[inline]
            fn dot(self, rhs: Self) -> Self::Cmp {
                self.$first * rhs.$first $( + self.$rest * rhs.$rest )*
            }

            #[inline]
            fn is_finite(self) -> bool {
                self.$first.is_finite() $( && self.$rest.is_finite() )*
            }

            #[inline]
            fn has_nan(self) -> bool {
                self.$first.is_nan() $( || self.$rest.is_nan() )*
            }
        }
    };
}

impl_vector!(Vec2D { x, y }; 2);
impl_vector!(Vec3D { x, y, z }; 3);

macro_rules! impl_vec_vec_op {
    ($V:ident{$( $cmp:ident ),+}; $($LB:ident)?; $($RB:ident)?; $Op:ident, $op:ident, $L:ty, $R:ty, $sym:tt) => {
        impl<T: $Op<S>$(+ $LB)? , S$(: $RB)?> $Op<$R> for $L {
            type Output = $V<T::Output>;

            #[inline]
            fn $op(self, rhs: $R) -> Self::Output {
                $V {
                    $($cmp: self.$cmp $sym rhs.$cmp,)+
                }
            }
        }
    };
}

macro_rules! impl_vec_num_op {
    ($V:ident{$( $cmp:ident ),+}; $($LB:ident)?; $Op:ident, $op:ident, $L:ty, $R:ty, $sym:tt) => {
        impl<T: $Op<S>$(+ $LB)? , S: Copy> $Op<$R> for $L {
            type Output = $V<T::Output>;

            #[inline]
            fn $op(self, rhs: $R) -> Self::Output {
                $V {
                    $($cmp: self.$cmp $sym rhs,)+
                }
            }
        }
    };
}

macro_rules! impl_mul_num_vec {
    ($V:ident{$( $cmp:ident ),+}; $L:ty, $R:ty, $O:ty) => {
        impl Mul<$R> for $L {
            type Output = $O;

            #[inline]
            fn mul(self, rhs: $R) -> Self::Output {
                $V {
                    $($cmp: self * rhs.$cmp,)+
                }
            }
        }
    };
}

macro_rules! impl_assign_vec {
    ($V:ident{$( $cmp:ident ),+}; $($RB:ident)?; $Op:ident, $op:ident, $R:ty, $sym:tt) => {
        impl<T: $Op<S>, S$(: $RB)?> $Op<$R> for $V<T> {
            #[inline]
            fn $op(&mut self, rhs: $R) {
                $(self.$cmp $sym rhs.$cmp;)+
            }
        }
    };
}

macro_rules! pseudo_designator {
    ($pseudo:tt => $actual:tt) => {
        $actual
    };
}

macro_rules! impl_vec_traits {
    ($V:ident{$( $cmp:ident ),+}; $N:literal) => {
        impl<T: Zero> Zero for $V<T> {
            #[inline]
            fn zero() -> Self {
                Self {
                    $($cmp: T::zero(),)+
                }
            }

            #[inline]
            fn is_zero(&self) -> bool {
                $(self.$cmp.is_zero())&&+
            }
        }

        impl<T: Neg> Neg for $V<T> {
            type Output = $V<T::Output>;

            #[inline]
            fn neg(self) -> Self::Output {
                $V {
                    $($cmp: -self.$cmp,)+
                }
            }
        }

        impl<T: Copy + Neg> Neg for &$V<T> {
            type Output = $V<T::Output>;

            #[inline]
            fn neg(self) -> Self::Output {
                $V {
                    $($cmp: -self.$cmp,)+
                }
            }
        }

        impl_vec_vec_op!($V{$($cmp),+}; ;; Add, add, $V<T>, $V<S>, +);
        impl_vec_vec_op!($V{$($cmp),+}; ; Copy; Add, add, $V<T>, &$V<S>, +);
        impl_vec_vec_op!($V{$($cmp),+}; Copy; ; Add, add, &$V<T>, $V<S>, +);
        impl_vec_vec_op!($V{$($cmp),+}; Copy; Copy; Add, add, &$V<T>, &$V<S>, +);

        impl_vec_vec_op!($V{$($cmp),+}; ;; Sub, sub, $V<T>, $V<S>, -);
        impl_vec_vec_op!($V{$($cmp),+}; ; Copy; Sub, sub, $V<T>, &$V<S>, -);
        impl_vec_vec_op!($V{$($cmp),+}; Copy; ; Sub, sub, &$V<T>, $V<S>, -);
        impl_vec_vec_op!($V{$($cmp),+}; Copy; Copy; Sub, sub, &$V<T>, &$V<S>, -);

        impl_vec_num_op!($V{$($cmp),+}; ; Mul, mul, $V<T>, S, *);
        impl_vec_num_op!($V{$($cmp),+}; Copy; Mul, mul, &$V<T>, S, *);

        impl_vec_num_op!($V{$($cmp),+}; ; Div, div, $V<T>, S, /);
        impl_vec_num_op!($V{$($cmp),+}; Copy; Div, div, &$V<T>, S, /);

        impl_mul_num_vec!($V{$($cmp),+}; f64, $V<f64>, $V<f64>);
        impl_mul_num_vec!($V{$($cmp),+}; f64, &$V<f64>, $V<f64>);
        impl_mul_num_vec!($V{$($cmp),+}; &f64, $V<f64>, $V<f64>);
        impl_mul_num_vec!($V{$($cmp),+}; &f64, &$V<f64>, $V<f64>);

        impl_mul_num_vec!($V{$($cmp),+}; f32, $V<f32>, $V<f32>);
        impl_mul_num_vec!($V{$($cmp),+}; f32, &$V<f32>, $V<f32>);
        impl_mul_num_vec!($V{$($cmp),+}; &f32, $V<f32>, $V<f32>);
        impl_mul_num_vec!($V{$($cmp),+}; &f32, &$V<f32>, $V<f32>);

        impl_assign_vec!($V{$($cmp),+}; ; AddAssign, add_assign, $V<S>, +=);
        impl_assign_vec!($V{$($cmp),+}; Copy; AddAssign, add_assign, &$V<S>, +=);
        impl_assign_vec!($V{$($cmp),+}; ; SubAssign, sub_assign, $V<S>, -=);
        impl_assign_vec!($V{$($cmp),+}; Copy; SubAssign, sub_assign, &$V<S>, -=);

        impl<T: MulAssign<S>, S: Copy> MulAssign<S> for $V<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: S) {
                $(self.$cmp *= rhs;)+
            }
        }

        impl<T: DivAssign<S>, S: Copy> DivAssign<S> for $V<T> {
            #[inline]
            fn div_assign(&mut self, rhs: S) {
                $(self.$cmp /= rhs;)+
            }
        }

        impl<T: Float> Sum for $V<T> {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold($V::zero(), |a, b| a + b)
            }
        }

        impl<T> From<[T; $N]> for $V<T> {
            #[inline]
            fn from([$($cmp),+]: [T; $N]) -> Self {
                Self { $($cmp),+ }
            }
        }
        impl<T> From<$V<T>> for [T; $N] {
            #[inline]
            fn from(vec: $V<T>) -> Self {
                [$(vec.$cmp),+]
            }
        }

        impl<T> From<($( pseudo_designator!($cmp => T), )+)> for $V<T> {
            #[inline]
            fn from(($($cmp,)+): ($( pseudo_designator!($cmp => T), )+)) -> Self {
                Self { $($cmp),+ }
            }
        }

        impl<T> From<$V<T>> for ($( pseudo_designator!($cmp => T), )+) {
            #[inline]
            fn from(vec: $V<T>) -> Self {
                ($(vec.$cmp,)+)
            }
        }

        impl<T> From<VecND<T, $N>> for $V<T> {
            #[inline]
            fn from(g: VecND<T, $N>) -> Self {
                let [$($cmp),+]: [T; $N] = g.into();
                Self { $($cmp),+ }
            }
        }

        impl<T> From<$V<T>> for VecND<T, $N> {
            #[inline]
            fn from(f: $V<T>) -> Self {
                [$(f.$cmp),+].into()
            }
        }

    };
}

impl_vec_traits!(Vec1D { x }; 1);
impl_vec_traits!(Vec2D { x, y }; 2);
impl_vec_traits!(Vec3D { x, y, z }; 3);
