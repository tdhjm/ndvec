use crate::*;

#[test]
fn norm_sqr() {
    fn test<V: Vector>(v: V, a: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(v.norm_sqr(), a, "sqrt({v:?}) != {a}");
    }
    test(Vec1D::new(2f32), 4f32);
    test(Vec1D::new(-3f64), 9f64);
    test(Vec2D::new(2f32, 3f32), 13f32);
    test(Vec2D::new(-4f64, 3f64), 25f64);
    test(Vec3D::new(3f32, 4f32, 5f32), 50f32);
    test(Vec3D::new(-4f64, 3f64, 2f64), 29f64);
}

#[test]
fn dot() {
    fn test<V: Vector>(a: V, b: V, p: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(a.dot(b), p, "{a:?} dot {b:?} != {p}");
    }
    test(Vec1D::new(2f32), Vec1D::new(5f32), 10f32);
    test(Vec1D::new(2f64), Vec1D::new(-4f64), -8f64);
    test(Vec2D::new(2f32, 3f32), Vec2D::new(3f32, 5f32), 21f32);
    test(Vec2D::new(-5f64, 8f64), Vec2D::new(6f64, 9f64), 42f64);
    test(
        Vec3D::new(10f32, -8f32, 6f32),
        Vec3D::new(3f32, -5f32, 7f32),
        112f32,
    );
    test(
        Vec3D::new(2f64, 3f64, 4f64),
        Vec3D::new(3f64, 4f64, -5f64),
        -2f64,
    );
}

#[test]
fn is_finite() {
    assert!(!Vec1D::new(f64::NAN).is_finite());
    assert!(Vec1D::new(0f64).is_finite());
    assert!(!Vec2D::new(f64::NAN, f64::NAN).is_finite());
    assert!(!Vec2D::new(f64::INFINITY, 0f64).is_finite());
    assert!(!Vec2D::new(0f64, f64::INFINITY).is_finite());
    assert!(Vec2D::new(0f64, 0f64).is_finite());
    assert!(!Vec3D::new(f64::NAN, f64::NAN, f64::NAN).is_finite());
    assert!(!Vec3D::new(f64::NAN, f64::INFINITY, 0f64).is_finite());
    assert!(!Vec3D::new(f64::NAN, 0f64, f64::INFINITY).is_finite());
    assert!(!Vec3D::new(f64::INFINITY, 0f64, 0f64).is_finite());
    assert!(!Vec3D::new(0f64, f64::INFINITY, f64::INFINITY).is_finite());
    assert!(!Vec3D::new(0f64, f64::NAN, 0f64).is_finite());
    assert!(!Vec3D::new(0f64, 0f64, f64::NAN).is_finite());
    assert!(Vec3D::new(0f64, 0f64, 0f64).is_finite());
}

#[test]
fn has_nan() {
    assert!(Vec1D::new(f64::NAN).has_nan());
    assert!(!Vec1D::new(0f64).has_nan());
    assert!(Vec2D::new(f64::NAN, f64::NAN).has_nan());
    assert!(Vec2D::new(f64::NAN, 0f64).has_nan());
    assert!(Vec2D::new(0f64, f64::NAN).has_nan());
    assert!(!Vec2D::new(0f64, 0f64).has_nan());
    assert!(Vec3D::new(f64::NAN, f64::NAN, f64::NAN).has_nan());
    assert!(Vec3D::new(f64::NAN, f64::NAN, 0f64).has_nan());
    assert!(Vec3D::new(f64::NAN, 0f64, f64::NAN).has_nan());
    assert!(Vec3D::new(f64::NAN, 0f64, 0f64).has_nan());
    assert!(Vec3D::new(0f64, f64::NAN, f64::NAN).has_nan());
    assert!(Vec3D::new(0f64, f64::NAN, 0f64).has_nan());
    assert!(Vec3D::new(0f64, 0f64, f64::NAN).has_nan());
    assert!(!Vec3D::new(0f64, 0f64, 0f64).has_nan());
}

#[test]
fn norm() {
    fn test<V: Vector>(v: V, a: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(v.norm(), a, "sqrt({v:?}) != {a}");
    }
    test(Vec1D::new(2f32), 2f32);
    test(Vec1D::new(-3f64), 3f64);
    test(Vec2D::new(5f32, 12f32), 13f32);
    test(Vec2D::new(-4f64, 3f64), 5f64);
    test(Vec3D::new(2f32, 3f32, 6f32), 7f32);
    test(Vec3D::new(-9f64, 6f64, 2f64), 11f64);
}

#[test]
fn distance() {
    assert_eq!(Vec1D::new(1f64).distance(Vec1D::new(3f64)), 2f64);
    assert_eq!(
        Vec2D::new(1f64, 2f64).distance(Vec2D::new(5f64, -1f64)),
        5f64
    );
    assert_eq!(
        Vec3D::new(3f32, 2f32, 1f32).distance(Vec3D::new(4f32, -2f32, 9f32)),
        9f32
    );
}

#[test]
fn distance_sqr() {
    assert_eq!(Vec1D::new(1f32).distance_sqr(Vec1D::new(3f32)), 4f32);
    assert_eq!(
        Vec2D::new(1f32, 2f32).distance_sqr(Vec2D::new(5f32, -1f32)),
        25f32
    );
    assert_eq!(
        Vec3D::new(3f64, 2f64, 1f64).distance_sqr(Vec3D::new(4f64, -2f64, 9f64)),
        81f64
    );
}

#[test]
fn zero() {
    assert_eq!(Vec1D::zero(), Vec1D::new(0f32));
    assert_eq!(Vec2D::zero(), Vec2D::new(0f64, 0f64));
    assert_eq!(Vec3D::zero(), Vec3D::new(0f32, 0f32, 0f32));
}

#[test]
fn is_zero() {
    assert!(Vec1D::new(0.0).is_zero());
    assert!(!Vec1D::new(1.0).is_zero());
    assert!(Vec2D::new(0.0, 0.0).is_zero());
    assert!(!Vec2D::new(0.0, 1.0).is_zero());
    assert!(!Vec2D::new(1.0, 0.0).is_zero());
    assert!(!Vec2D::new(1.0, 1.0).is_zero());
    assert!(Vec3D::new(0.0, 0.0, 0.0).is_zero());
    assert!(!Vec3D::new(0.0, 0.0, 1.0).is_zero());
    assert!(!Vec3D::new(0.0, 1.0, 0.0).is_zero());
    assert!(!Vec3D::new(0.0, 1.0, 1.0).is_zero());
    assert!(!Vec3D::new(1.0, 0.0, 0.0).is_zero());
    assert!(!Vec3D::new(1.0, 0.0, 1.0).is_zero());
    assert!(!Vec3D::new(1.0, 1.0, 0.0).is_zero());
    assert!(!Vec3D::new(1.0, 1.0, 1.0).is_zero());
}

#[test]
fn neg() {
    assert_eq!(-Vec1D::new(2f32), Vec1D::new(-2f32));
    assert_eq!(-&Vec1D::new(-3f64), Vec1D::new(3f64));
    assert_eq!(-Vec2D::new(2f32, -5f32), Vec2D::new(-2f32, 5f32));
    assert_eq!(-&Vec2D::new(-3f64, -12f64), Vec2D::new(3f64, 12f64));
    assert_eq!(
        -Vec3D::new(2f32, -5f32, 1f32),
        Vec3D::new(-2f32, 5f32, -1f32)
    );
    assert_eq!(
        -&Vec3D::new(-3f64, -12f64, 5f64),
        Vec3D::new(3f64, 12f64, -5f64)
    );
}

#[test]
fn add() {
    assert_eq!(Vec1D::new(1f32) + Vec1D::new(4f32), Vec1D::new(5f32));
    assert_eq!(Vec1D::new(1f32) + &Vec1D::new(4f32), Vec1D::new(5f32));
    assert_eq!(&Vec1D::new(1f32) + Vec1D::new(4f32), Vec1D::new(5f32));
    assert_eq!(&Vec1D::new(1f32) + &Vec1D::new(4f32), Vec1D::new(5f32));

    assert_eq!(
        Vec2D::new(1f64, 5f64) + Vec2D::new(4f64, 2f64),
        Vec2D::new(5f64, 7f64)
    );
    assert_eq!(
        Vec2D::new(1f64, 5f64) + &Vec2D::new(4f64, 2f64),
        Vec2D::new(5f64, 7f64)
    );
    assert_eq!(
        &Vec2D::new(1f64, 5f64) + Vec2D::new(4f64, 2f64),
        Vec2D::new(5f64, 7f64)
    );
    assert_eq!(
        &Vec2D::new(1f64, 5f64) + &Vec2D::new(4f64, 2f64),
        Vec2D::new(5f64, 7f64)
    );

    assert_eq!(
        Vec3D::new(1f64, 5f64, -2f64) + Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(5f64, 7f64, -3f64)
    );
    assert_eq!(
        Vec3D::new(1f64, 5f64, -2f64) + &Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(5f64, 7f64, -3f64)
    );
    assert_eq!(
        &Vec3D::new(1f64, 5f64, -2f64) + Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(5f64, 7f64, -3f64)
    );
    assert_eq!(
        &Vec3D::new(1f64, 5f64, -2f64) + &Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(5f64, 7f64, -3f64)
    );
}

#[test]
fn sub() {
    assert_eq!(Vec1D::new(1f32) - Vec1D::new(4f32), Vec1D::new(-3f32));
    assert_eq!(Vec1D::new(1f32) - &Vec1D::new(4f32), Vec1D::new(-3f32));
    assert_eq!(&Vec1D::new(1f32) - Vec1D::new(4f32), Vec1D::new(-3f32));
    assert_eq!(&Vec1D::new(1f32) - &Vec1D::new(4f32), Vec1D::new(-3f32));

    assert_eq!(
        Vec2D::new(1f64, 5f64) - Vec2D::new(4f64, 2f64),
        Vec2D::new(-3f64, 3f64)
    );
    assert_eq!(
        Vec2D::new(1f64, 5f64) - &Vec2D::new(4f64, 2f64),
        Vec2D::new(-3f64, 3f64)
    );
    assert_eq!(
        &Vec2D::new(1f64, 5f64) - Vec2D::new(4f64, 2f64),
        Vec2D::new(-3f64, 3f64)
    );
    assert_eq!(
        &Vec2D::new(1f64, 5f64) - &Vec2D::new(4f64, 2f64),
        Vec2D::new(-3f64, 3f64)
    );

    assert_eq!(
        Vec3D::new(1f64, 5f64, -2f64) - Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(-3f64, 3f64, -1f64)
    );
    assert_eq!(
        Vec3D::new(1f64, 5f64, -2f64) - &Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(-3f64, 3f64, -1f64)
    );
    assert_eq!(
        &Vec3D::new(1f64, 5f64, -2f64) - Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(-3f64, 3f64, -1f64)
    );
    assert_eq!(
        &Vec3D::new(1f64, 5f64, -2f64) - &Vec3D::new(4f64, 2f64, -1f64),
        Vec3D::new(-3f64, 3f64, -1f64)
    );
}

#[test]
fn mul() {
    assert_eq!(Vec1D::new(4f32) * 3f32, Vec1D::new(12f32));
    assert_eq!(Vec1D::new(4f32) * &3f32, Vec1D::new(12f32));
    assert_eq!(&Vec1D::new(4f32) * 3f32, Vec1D::new(12f32));
    assert_eq!(&Vec1D::new(4f32) * &3f32, Vec1D::new(12f32));

    assert_eq!(Vec2D::new(2f64, 3f64) * -5f64, Vec2D::new(-10f64, -15f64));
    assert_eq!(Vec2D::new(2f64, 3f64) * &-5f64, Vec2D::new(-10f64, -15f64));
    assert_eq!(&Vec2D::new(2f64, 3f64) * -5f64, Vec2D::new(-10f64, -15f64));
    assert_eq!(&Vec2D::new(2f64, 3f64) * &-5f64, Vec2D::new(-10f64, -15f64));

    assert_eq!(
        Vec3D::new(-1f64, 7f64, 5f64) * -10f64,
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        Vec3D::new(-1f64, 7f64, 5f64) * &-10f64,
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        &Vec3D::new(-1f64, 7f64, 5f64) * -10f64,
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        &Vec3D::new(-1f64, 7f64, 5f64) * &-10f64,
        Vec3D::new(10f64, -70f64, -50f64)
    );
}

#[test]
fn div() {
    assert_eq!(Vec1D::new(12f64) / 3f64, Vec1D::new(4f64));
    assert_eq!(Vec1D::new(12f64) / &3f64, Vec1D::new(4f64));
    assert_eq!(&Vec1D::new(12f64) / 3f64, Vec1D::new(4f64));
    assert_eq!(&Vec1D::new(12f64) / &3f64, Vec1D::new(4f64));

    assert_eq!(Vec2D::new(-10f32, -15f32) / -5f32, Vec2D::new(2f32, 3f32));
    assert_eq!(Vec2D::new(-10f32, -15f32) / &-5f32, Vec2D::new(2f32, 3f32));
    assert_eq!(&Vec2D::new(-10f32, -15f32) / -5f32, Vec2D::new(2f32, 3f32));
    assert_eq!(&Vec2D::new(-10f32, -15f32) / &-5f32, Vec2D::new(2f32, 3f32));

    assert_eq!(
        Vec3D::new(10f32, -70f32, -50f32) / -10f32,
        Vec3D::new(-1f32, 7f32, 5f32)
    );
    assert_eq!(
        Vec3D::new(10f32, -70f32, -50f32) / &-10f32,
        Vec3D::new(-1f32, 7f32, 5f32)
    );
    assert_eq!(
        &Vec3D::new(10f32, -70f32, -50f32) / -10f32,
        Vec3D::new(-1f32, 7f32, 5f32)
    );
    assert_eq!(
        &Vec3D::new(10f32, -70f32, -50f32) / &-10f32,
        Vec3D::new(-1f32, 7f32, 5f32)
    );
}

#[test]
fn num_mul() {
    assert_eq!(3f32 * Vec1D::new(4f32), Vec1D::new(12f32));
    assert_eq!(&3f32 * Vec1D::new(4f32), Vec1D::new(12f32));
    assert_eq!(3f32 * &Vec1D::new(4f32), Vec1D::new(12f32));
    assert_eq!(&3f32 * &Vec1D::new(4f32), Vec1D::new(12f32));

    assert_eq!(-5f64 * Vec2D::new(2f64, 3f64), Vec2D::new(-10f64, -15f64));
    assert_eq!(&-5f64 * Vec2D::new(2f64, 3f64), Vec2D::new(-10f64, -15f64));
    assert_eq!(-5f64 * &Vec2D::new(2f64, 3f64), Vec2D::new(-10f64, -15f64));
    assert_eq!(&-5f64 * &Vec2D::new(2f64, 3f64), Vec2D::new(-10f64, -15f64));

    assert_eq!(
        -10f64 * Vec3D::new(-1f64, 7f64, 5f64),
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        &-10f64 * Vec3D::new(-1f64, 7f64, 5f64),
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        -10f64 * &Vec3D::new(-1f64, 7f64, 5f64),
        Vec3D::new(10f64, -70f64, -50f64)
    );
    assert_eq!(
        &-10f64 * &Vec3D::new(-1f64, 7f64, 5f64),
        Vec3D::new(10f64, -70f64, -50f64)
    );
}

#[test]
fn add_assign() {
    let mut x = Vec1D::new(1f32);
    x += Vec1D::new(4f32);
    assert_eq!(x, Vec1D::new(5f32));
    let mut x = Vec1D::new(1f32);
    x += &Vec1D::new(4f32);
    assert_eq!(x, Vec1D::new(5f32));

    let mut x = Vec2D::new(1f64, 5f64);
    x += Vec2D::new(4f64, 2f64);
    assert_eq!(x, Vec2D::new(5f64, 7f64));
    let mut x = Vec2D::new(1f64, 5f64);
    x += &Vec2D::new(4f64, 2f64);
    assert_eq!(x, Vec2D::new(5f64, 7f64));

    let mut x = Vec3D::new(1f64, 5f64, -2f64);
    x += Vec3D::new(4f64, 2f64, -1f64);
    assert_eq!(x, Vec3D::new(5f64, 7f64, -3f64));
    let mut x = Vec3D::new(1f64, 5f64, -2f64);
    x += &Vec3D::new(4f64, 2f64, -1f64);
    assert_eq!(x, Vec3D::new(5f64, 7f64, -3f64));
}

#[test]
fn sub_assign() {
    let mut x = Vec1D::new(1f32);
    x -= Vec1D::new(4f32);
    assert_eq!(x, Vec1D::new(-3f32));
    let mut x = Vec1D::new(1f32);
    x -= &Vec1D::new(4f32);
    assert_eq!(x, Vec1D::new(-3f32));

    let mut x = Vec2D::new(1f64, 5f64);
    x -= Vec2D::new(4f64, 2f64);
    assert_eq!(x, Vec2D::new(-3f64, 3f64));

    let mut x = Vec2D::new(1f64, 5f64);
    x -= &Vec2D::new(4f64, 2f64);
    assert_eq!(x, Vec2D::new(-3f64, 3f64));

    let mut x = Vec3D::new(1f64, 5f64, -2f64);
    x -= Vec3D::new(4f64, 2f64, -1f64);
    assert_eq!(x, Vec3D::new(-3f64, 3f64, -1f64));

    let mut x = Vec3D::new(1f64, 5f64, -2f64);
    x -= &Vec3D::new(4f64, 2f64, -1f64);
    assert_eq!(x, Vec3D::new(-3f64, 3f64, -1f64));
}

#[test]
fn mul_assign() {
    let mut x = Vec1D::new(4f32);
    x *= 3f32;
    assert_eq!(x, Vec1D::new(12f32));
    let mut x = Vec1D::new(4f32);
    x *= &3f32;
    assert_eq!(x, Vec1D::new(12f32));

    let mut x = Vec2D::new(2f64, 3f64);
    x *= -5f64;
    assert_eq!(x, Vec2D::new(-10f64, -15f64));
    let mut x = Vec2D::new(2f64, 3f64);
    x *= &-5f64;
    assert_eq!(x, Vec2D::new(-10f64, -15f64));

    let mut x = Vec3D::new(-1f64, 7f64, 5f64);
    x *= -10f64;
    assert_eq!(x, Vec3D::new(10f64, -70f64, -50f64));
    let mut x = Vec3D::new(-1f64, 7f64, 5f64);
    x *= &-10f64;
    assert_eq!(x, Vec3D::new(10f64, -70f64, -50f64));
}

#[test]
fn div_assign() {
    let mut x = Vec1D::new(12f64);
    x /= 3f64;
    assert_eq!(x, Vec1D::new(4f64));
    let mut x = Vec1D::new(12f64);
    x /= &3f64;
    assert_eq!(x, Vec1D::new(4f64));

    let mut x = Vec2D::new(-10f32, -15f32);
    x /= -5f32;
    assert_eq!(x, Vec2D::new(2f32, 3f32));
    let mut x = Vec2D::new(-10f32, -15f32);
    x /= &-5f32;
    assert_eq!(x, Vec2D::new(2f32, 3f32));

    let mut x = Vec3D::new(10f32, -70f32, -50f32);
    x /= -10f32;
    assert_eq!(x, Vec3D::new(-1f32, 7f32, 5f32));
    let mut x = Vec3D::new(10f32, -70f32, -50f32);
    x /= &-10f32;
    assert_eq!(x, Vec3D::new(-1f32, 7f32, 5f32));
}

#[test]
fn sum() {
    assert_eq!(
        [Vec1D::new(1f64), Vec1D::new(3f64), Vec1D::new(-2f64)]
            .into_iter()
            .sum::<Vec1D<_>>(),
        Vec1D::new(2f64)
    );

    assert_eq!(
        [
            Vec2D::new(1f64, 5f64),
            Vec2D::new(3f64, -3f64),
            Vec2D::new(-2f64, 7f64)
        ]
        .into_iter()
        .sum::<Vec2D<_>>(),
        Vec2D::new(2f64, 9f64)
    );

    assert_eq!(
        [
            Vec3D::new(1f32, 5f32, 2f32),
            Vec3D::new(3f32, -3f32, 9f32),
            Vec3D::new(-2f32, 7f32, -18f32)
        ]
        .into_iter()
        .sum::<Vec3D<_>>(),
        Vec3D::new(2f32, 9f32, -7f32)
    );
}

#[test]
fn from_arr() {
    assert_eq!(Vec1D::from([5f32]), Vec1D::new(5f32));
    assert_eq!(Vec2D::from([5f32, -2f32]), Vec2D::new(5f32, -2f32));
    assert_eq!(
        Vec3D::from([5f64, -2f64, 7f64]),
        Vec3D::new(5f64, -2f64, 7f64)
    );
}

#[test]
fn into_arr() {
    let x: [_; 1] = Vec1D::new(5f32).into();
    assert_eq!(x, [5f32]);
    let x: [_; 2] = Vec2D::new(5f32, -2f32).into();
    assert_eq!(x, [5f32, -2f32]);
    let x: [_; 3] = Vec3D::new(5f64, -2f64, 7f64).into();
    assert_eq!(x, [5f64, -2f64, 7f64]);
}

#[test]
fn from_tuple() {
    assert_eq!(Vec1D::from((5f32,)), Vec1D::new(5f32));
    assert_eq!(Vec2D::from((5f32, -2f32,)), Vec2D::new(5f32, -2f32));
    assert_eq!(
        Vec3D::from((5f64, -2f64, 7f64,)),
        Vec3D::new(5f64, -2f64, 7f64)
    );
}

#[test]
fn into_tuple() {
    let x: (_,) = Vec1D::new(5f32).into();
    assert_eq!(x, (5f32,));
    let x: (_, _) = Vec2D::new(5f32, -2f32).into();
    assert_eq!(x, (5f32, -2f32,));
    let x: (_, _, _) = Vec3D::new(5f64, -2f64, 7f64).into();
    assert_eq!(x, (5f64, -2f64, 7f64,));
}

#[test]
fn from_nd() {
    assert_eq!(Vec1D::from(VecND::from([5f32])), Vec1D::new(5f32));
    assert_eq!(
        Vec2D::from(VecND::from([5f32, -2f32])),
        Vec2D::new(5f32, -2f32)
    );
    assert_eq!(
        Vec3D::from(VecND::from([5f64, -2f64, 7f64])),
        Vec3D::new(5f64, -2f64, 7f64)
    );
}

#[test]
fn into_nd() {
    let x: VecND<_, 1> = Vec1D::new(5f32).into();
    assert_eq!(x, VecND::from([5f32]));
    let x: VecND<_, 2> = Vec2D::new(5f32, -2f32).into();
    assert_eq!(x, VecND::from([5f32, -2f32]));
    let x: VecND<_, 3> = Vec3D::new(5f64, -2f64, 7f64).into();
    assert_eq!(x, VecND::from([5f64, -2f64, 7f64]));
}
