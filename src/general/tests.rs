use crate::*;

#[test]
fn norm_sqr() {
    fn test<V: Vector>(v: V, a: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(v.norm_sqr(), a, "sqrt({v:?}) != {a}");
    }
    test(VecND::from([2f32]), 4f32);
    test(VecND::from([-3f64]), 9f64);
    test(VecND::from([2f32, 3f32]), 13f32);
    test(VecND::from([-4f64, 3f64]), 25f64);
    test(VecND::from([3f32, 4f32, 5f32]), 50f32);
    test(VecND::from([-4f64, 3f64, 2f64]), 29f64);
}

#[test]
fn dot() {
    fn test<V: Vector>(a: V, b: V, p: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(a.dot(b), p, "{a:?} dot {b:?} != {p}");
    }
    test(VecND::from([2f32]), VecND::from([5f32]), 10f32);
    test(VecND::from([2f64]), VecND::from([-4f64]), -8f64);
    test(VecND::from([2f32, 3f32]), VecND::from([3f32, 5f32]), 21f32);
    test(VecND::from([-5f64, 8f64]), VecND::from([6f64, 9f64]), 42f64);
    test(
        VecND::from([10f32, -8f32, 6f32]),
        VecND::from([3f32, -5f32, 7f32]),
        112f32,
    );
    test(
        VecND::from([2f64, 3f64, 4f64]),
        VecND::from([3f64, 4f64, -5f64]),
        -2f64,
    );
}

#[test]
fn is_finite() {
    assert!(!VecND::from([f64::NAN]).is_finite());
    assert!(VecND::from([0f64]).is_finite());
    assert!(!VecND::from([f64::NAN, f64::NAN]).is_finite());
    assert!(!VecND::from([f64::INFINITY, 0f64]).is_finite());
    assert!(!VecND::from([0f64, f64::INFINITY]).is_finite());
    assert!(VecND::from([0f64, 0f64]).is_finite());
    assert!(!VecND::from([f64::NAN, f64::NAN, f64::NAN]).is_finite());
    assert!(!VecND::from([f64::NAN, f64::INFINITY, 0f64]).is_finite());
    assert!(!VecND::from([f64::NAN, 0f64, f64::INFINITY]).is_finite());
    assert!(!VecND::from([f64::INFINITY, 0f64, 0f64]).is_finite());
    assert!(!VecND::from([0f64, f64::INFINITY, f64::INFINITY]).is_finite());
    assert!(!VecND::from([0f64, f64::NAN, 0f64]).is_finite());
    assert!(!VecND::from([0f64, 0f64, f64::NAN]).is_finite());
    assert!(VecND::from([0f64, 0f64, 0f64]).is_finite());
}

#[test]
fn has_nan() {
    assert!(VecND::from([f64::NAN]).has_nan());
    assert!(!VecND::from([0f64]).has_nan());
    assert!(VecND::from([f64::NAN, f64::NAN]).has_nan());
    assert!(VecND::from([f64::NAN, 0f64]).has_nan());
    assert!(VecND::from([0f64, f64::NAN]).has_nan());
    assert!(!VecND::from([0f64, 0f64]).has_nan());
    assert!(VecND::from([f64::NAN, f64::NAN, f64::NAN]).has_nan());
    assert!(VecND::from([f64::NAN, f64::NAN, 0f64]).has_nan());
    assert!(VecND::from([f64::NAN, 0f64, f64::NAN]).has_nan());
    assert!(VecND::from([f64::NAN, 0f64, 0f64]).has_nan());
    assert!(VecND::from([0f64, f64::NAN, f64::NAN]).has_nan());
    assert!(VecND::from([0f64, f64::NAN, 0f64]).has_nan());
    assert!(VecND::from([0f64, 0f64, f64::NAN]).has_nan());
    assert!(!VecND::from([0f64, 0f64, 0f64]).has_nan());
}

#[test]
fn norm() {
    fn test<V: Vector>(v: V, a: V::Cmp)
    where
        for<'l> &'l V: VectorRefOps<V::Cmp, V>,
    {
        assert_eq!(v.norm(), a, "sqrt({v:?}) != {a}");
    }
    test(VecND::from([2f32]), 2f32);
    test(VecND::from([-3f64]), 3f64);
    test(VecND::from([5f32, 12f32]), 13f32);
    test(VecND::from([-4f64, 3f64]), 5f64);
    test(VecND::from([2f32, 3f32, 6f32]), 7f32);
    test(VecND::from([-9f64, 6f64, 2f64]), 11f64);
}

#[test]
fn distance() {
    assert_eq!(VecND::from([1f64]).distance(VecND::from([3f64])), 2f64);
    assert_eq!(
        VecND::from([1f64, 2f64]).distance(VecND::from([5f64, -1f64])),
        5f64
    );
    assert_eq!(
        VecND::from([3f32, 2f32, 1f32]).distance(VecND::from([4f32, -2f32, 9f32])),
        9f32
    );
}

#[test]
fn distance_sqr() {
    assert_eq!(VecND::from([1f32]).distance_sqr(VecND::from([3f32])), 4f32);
    assert_eq!(
        VecND::from([1f32, 2f32]).distance_sqr(VecND::from([5f32, -1f32])),
        25f32
    );
    assert_eq!(
        VecND::from([3f64, 2f64, 1f64]).distance_sqr(VecND::from([4f64, -2f64, 9f64])),
        81f64
    );
}

#[test]
fn zero() {
    assert_eq!(VecND::zero(), VecND::from([0f32]));
    assert_eq!(VecND::zero(), VecND::from([0f64, 0f64]));
    assert_eq!(VecND::zero(), VecND::from([0f32, 0f32, 0f32]));
}

#[test]
fn is_zero() {
    assert!(VecND::from([0.0]).is_zero());
    assert!(!VecND::from([1.0]).is_zero());
    assert!(VecND::from([0.0, 0.0]).is_zero());
    assert!(!VecND::from([0.0, 1.0]).is_zero());
    assert!(!VecND::from([1.0, 0.0]).is_zero());
    assert!(!VecND::from([1.0, 1.0]).is_zero());
    assert!(VecND::from([0.0, 0.0, 0.0]).is_zero());
    assert!(!VecND::from([0.0, 0.0, 1.0]).is_zero());
    assert!(!VecND::from([0.0, 1.0, 0.0]).is_zero());
    assert!(!VecND::from([0.0, 1.0, 1.0]).is_zero());
    assert!(!VecND::from([1.0, 0.0, 0.0]).is_zero());
    assert!(!VecND::from([1.0, 0.0, 1.0]).is_zero());
    assert!(!VecND::from([1.0, 1.0, 0.0]).is_zero());
    assert!(!VecND::from([1.0, 1.0, 1.0]).is_zero());
}

#[test]
fn neg() {
    assert_eq!(-VecND::from([2f32]), VecND::from([-2f32]));
    assert_eq!(-&VecND::from([-3f64]), VecND::from([3f64]));
    assert_eq!(-VecND::from([2f32, -5f32]), VecND::from([-2f32, 5f32]));
    assert_eq!(-&VecND::from([-3f64, -12f64]), VecND::from([3f64, 12f64]));
    assert_eq!(
        -VecND::from([2f32, -5f32, 1f32]),
        VecND::from([-2f32, 5f32, -1f32])
    );
    assert_eq!(
        -&VecND::from([-3f64, -12f64, 5f64]),
        VecND::from([3f64, 12f64, -5f64])
    );
}

#[test]
fn add() {
    assert_eq!(
        VecND::from([1f32]) + VecND::from([4f32]),
        VecND::from([5f32])
    );
    assert_eq!(
        VecND::from([1f32]) + &VecND::from([4f32]),
        VecND::from([5f32])
    );
    assert_eq!(
        &VecND::from([1f32]) + VecND::from([4f32]),
        VecND::from([5f32])
    );
    assert_eq!(
        &VecND::from([1f32]) + &VecND::from([4f32]),
        VecND::from([5f32])
    );

    assert_eq!(
        VecND::from([1f64, 5f64]) + VecND::from([4f64, 2f64]),
        VecND::from([5f64, 7f64])
    );
    assert_eq!(
        VecND::from([1f64, 5f64]) + &VecND::from([4f64, 2f64]),
        VecND::from([5f64, 7f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64]) + VecND::from([4f64, 2f64]),
        VecND::from([5f64, 7f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64]) + &VecND::from([4f64, 2f64]),
        VecND::from([5f64, 7f64])
    );

    assert_eq!(
        VecND::from([1f64, 5f64, -2f64]) + VecND::from([4f64, 2f64, -1f64]),
        VecND::from([5f64, 7f64, -3f64])
    );
    assert_eq!(
        VecND::from([1f64, 5f64, -2f64]) + &VecND::from([4f64, 2f64, -1f64]),
        VecND::from([5f64, 7f64, -3f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64, -2f64]) + VecND::from([4f64, 2f64, -1f64]),
        VecND::from([5f64, 7f64, -3f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64, -2f64]) + &VecND::from([4f64, 2f64, -1f64]),
        VecND::from([5f64, 7f64, -3f64])
    );
}

#[test]
fn sub() {
    assert_eq!(
        VecND::from([1f32]) - VecND::from([4f32]),
        VecND::from([-3f32])
    );
    assert_eq!(
        VecND::from([1f32]) - &VecND::from([4f32]),
        VecND::from([-3f32])
    );
    assert_eq!(
        &VecND::from([1f32]) - VecND::from([4f32]),
        VecND::from([-3f32])
    );
    assert_eq!(
        &VecND::from([1f32]) - &VecND::from([4f32]),
        VecND::from([-3f32])
    );

    assert_eq!(
        VecND::from([1f64, 5f64]) - VecND::from([4f64, 2f64]),
        VecND::from([-3f64, 3f64])
    );
    assert_eq!(
        VecND::from([1f64, 5f64]) - &VecND::from([4f64, 2f64]),
        VecND::from([-3f64, 3f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64]) - VecND::from([4f64, 2f64]),
        VecND::from([-3f64, 3f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64]) - &VecND::from([4f64, 2f64]),
        VecND::from([-3f64, 3f64])
    );

    assert_eq!(
        VecND::from([1f64, 5f64, -2f64]) - VecND::from([4f64, 2f64, -1f64]),
        VecND::from([-3f64, 3f64, -1f64])
    );
    assert_eq!(
        VecND::from([1f64, 5f64, -2f64]) - &VecND::from([4f64, 2f64, -1f64]),
        VecND::from([-3f64, 3f64, -1f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64, -2f64]) - VecND::from([4f64, 2f64, -1f64]),
        VecND::from([-3f64, 3f64, -1f64])
    );
    assert_eq!(
        &VecND::from([1f64, 5f64, -2f64]) - &VecND::from([4f64, 2f64, -1f64]),
        VecND::from([-3f64, 3f64, -1f64])
    );
}

#[test]
fn mul() {
    assert_eq!(VecND::from([4f32]) * 3f32, VecND::from([12f32]));
    assert_eq!(VecND::from([4f32]) * &3f32, VecND::from([12f32]));
    assert_eq!(&VecND::from([4f32]) * 3f32, VecND::from([12f32]));
    assert_eq!(&VecND::from([4f32]) * &3f32, VecND::from([12f32]));

    assert_eq!(
        VecND::from([2f64, 3f64]) * -5f64,
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        VecND::from([2f64, 3f64]) * &-5f64,
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        &VecND::from([2f64, 3f64]) * -5f64,
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        &VecND::from([2f64, 3f64]) * &-5f64,
        VecND::from([-10f64, -15f64])
    );

    assert_eq!(
        VecND::from([-1f64, 7f64, 5f64]) * -10f64,
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        VecND::from([-1f64, 7f64, 5f64]) * &-10f64,
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        &VecND::from([-1f64, 7f64, 5f64]) * -10f64,
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        &VecND::from([-1f64, 7f64, 5f64]) * &-10f64,
        VecND::from([10f64, -70f64, -50f64])
    );
}

#[test]
fn div() {
    assert_eq!(VecND::from([12f64]) / 3f64, VecND::from([4f64]));
    assert_eq!(VecND::from([12f64]) / &3f64, VecND::from([4f64]));
    assert_eq!(&VecND::from([12f64]) / 3f64, VecND::from([4f64]));
    assert_eq!(&VecND::from([12f64]) / &3f64, VecND::from([4f64]));

    assert_eq!(
        VecND::from([-10f32, -15f32]) / -5f32,
        VecND::from([2f32, 3f32])
    );
    assert_eq!(
        VecND::from([-10f32, -15f32]) / &-5f32,
        VecND::from([2f32, 3f32])
    );
    assert_eq!(
        &VecND::from([-10f32, -15f32]) / -5f32,
        VecND::from([2f32, 3f32])
    );
    assert_eq!(
        &VecND::from([-10f32, -15f32]) / &-5f32,
        VecND::from([2f32, 3f32])
    );

    assert_eq!(
        VecND::from([10f32, -70f32, -50f32]) / -10f32,
        VecND::from([-1f32, 7f32, 5f32])
    );
    assert_eq!(
        VecND::from([10f32, -70f32, -50f32]) / &-10f32,
        VecND::from([-1f32, 7f32, 5f32])
    );
    assert_eq!(
        &VecND::from([10f32, -70f32, -50f32]) / -10f32,
        VecND::from([-1f32, 7f32, 5f32])
    );
    assert_eq!(
        &VecND::from([10f32, -70f32, -50f32]) / &-10f32,
        VecND::from([-1f32, 7f32, 5f32])
    );
}

#[test]
fn num_mul() {
    assert_eq!(3f32 * VecND::from([4f32]), VecND::from([12f32]));
    assert_eq!(&3f32 * VecND::from([4f32]), VecND::from([12f32]));
    assert_eq!(3f32 * &VecND::from([4f32]), VecND::from([12f32]));
    assert_eq!(&3f32 * &VecND::from([4f32]), VecND::from([12f32]));

    assert_eq!(
        -5f64 * VecND::from([2f64, 3f64]),
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        &-5f64 * VecND::from([2f64, 3f64]),
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        -5f64 * &VecND::from([2f64, 3f64]),
        VecND::from([-10f64, -15f64])
    );
    assert_eq!(
        &-5f64 * &VecND::from([2f64, 3f64]),
        VecND::from([-10f64, -15f64])
    );

    assert_eq!(
        -10f64 * VecND::from([-1f64, 7f64, 5f64]),
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        &-10f64 * VecND::from([-1f64, 7f64, 5f64]),
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        -10f64 * &VecND::from([-1f64, 7f64, 5f64]),
        VecND::from([10f64, -70f64, -50f64])
    );
    assert_eq!(
        &-10f64 * &VecND::from([-1f64, 7f64, 5f64]),
        VecND::from([10f64, -70f64, -50f64])
    );
}

#[test]
fn add_assign() {
    let mut x = VecND::from([1f32]);
    x += VecND::from([4f32]);
    assert_eq!(x, VecND::from([5f32]));
    let mut x = VecND::from([1f32]);
    x += &VecND::from([4f32]);
    assert_eq!(x, VecND::from([5f32]));

    let mut x = VecND::from([1f64, 5f64]);
    x += VecND::from([4f64, 2f64]);
    assert_eq!(x, VecND::from([5f64, 7f64]));
    let mut x = VecND::from([1f64, 5f64]);
    x += &VecND::from([4f64, 2f64]);
    assert_eq!(x, VecND::from([5f64, 7f64]));

    let mut x = VecND::from([1f64, 5f64, -2f64]);
    x += VecND::from([4f64, 2f64, -1f64]);
    assert_eq!(x, VecND::from([5f64, 7f64, -3f64]));
    let mut x = VecND::from([1f64, 5f64, -2f64]);
    x += &VecND::from([4f64, 2f64, -1f64]);
    assert_eq!(x, VecND::from([5f64, 7f64, -3f64]));
}

#[test]
fn sub_assign() {
    let mut x = VecND::from([1f32]);
    x -= VecND::from([4f32]);
    assert_eq!(x, VecND::from([-3f32]));
    let mut x = VecND::from([1f32]);
    x -= &VecND::from([4f32]);
    assert_eq!(x, VecND::from([-3f32]));

    let mut x = VecND::from([1f64, 5f64]);
    x -= VecND::from([4f64, 2f64]);
    assert_eq!(x, VecND::from([-3f64, 3f64]));

    let mut x = VecND::from([1f64, 5f64]);
    x -= &VecND::from([4f64, 2f64]);
    assert_eq!(x, VecND::from([-3f64, 3f64]));

    let mut x = VecND::from([1f64, 5f64, -2f64]);
    x -= VecND::from([4f64, 2f64, -1f64]);
    assert_eq!(x, VecND::from([-3f64, 3f64, -1f64]));

    let mut x = VecND::from([1f64, 5f64, -2f64]);
    x -= &VecND::from([4f64, 2f64, -1f64]);
    assert_eq!(x, VecND::from([-3f64, 3f64, -1f64]));
}

#[test]
fn mul_assign() {
    let mut x = VecND::from([4f32]);
    x *= 3f32;
    assert_eq!(x, VecND::from([12f32]));
    let mut x = VecND::from([4f32]);
    x *= &3f32;
    assert_eq!(x, VecND::from([12f32]));

    let mut x = VecND::from([2f64, 3f64]);
    x *= -5f64;
    assert_eq!(x, VecND::from([-10f64, -15f64]));
    let mut x = VecND::from([2f64, 3f64]);
    x *= &-5f64;
    assert_eq!(x, VecND::from([-10f64, -15f64]));

    let mut x = VecND::from([-1f64, 7f64, 5f64]);
    x *= -10f64;
    assert_eq!(x, VecND::from([10f64, -70f64, -50f64]));
    let mut x = VecND::from([-1f64, 7f64, 5f64]);
    x *= &-10f64;
    assert_eq!(x, VecND::from([10f64, -70f64, -50f64]));
}

#[test]
fn div_assign() {
    let mut x = VecND::from([12f64]);
    x /= 3f64;
    assert_eq!(x, VecND::from([4f64]));
    let mut x = VecND::from([12f64]);
    x /= &3f64;
    assert_eq!(x, VecND::from([4f64]));

    let mut x = VecND::from([-10f32, -15f32]);
    x /= -5f32;
    assert_eq!(x, VecND::from([2f32, 3f32]));
    let mut x = VecND::from([-10f32, -15f32]);
    x /= &-5f32;
    assert_eq!(x, VecND::from([2f32, 3f32]));

    let mut x = VecND::from([10f32, -70f32, -50f32]);
    x /= -10f32;
    assert_eq!(x, VecND::from([-1f32, 7f32, 5f32]));
    let mut x = VecND::from([10f32, -70f32, -50f32]);
    x /= &-10f32;
    assert_eq!(x, VecND::from([-1f32, 7f32, 5f32]));
}

#[test]
fn sum() {
    assert_eq!(
        [
            VecND::from([1f64]),
            VecND::from([3f64]),
            VecND::from([-2f64])
        ]
        .into_iter()
        .sum::<VecND<_, 1>>(),
        VecND::from([2f64])
    );

    assert_eq!(
        [
            VecND::from([1f64, 5f64]),
            VecND::from([3f64, -3f64]),
            VecND::from([-2f64, 7f64])
        ]
        .into_iter()
        .sum::<VecND<_, 2>>(),
        VecND::from([2f64, 9f64])
    );

    assert_eq!(
        [
            VecND::from([1f32, 5f32, 2f32]),
            VecND::from([3f32, -3f32, 9f32]),
            VecND::from([-2f32, 7f32, -18f32])
        ]
        .into_iter()
        .sum::<VecND<_, 3>>(),
        VecND::from([2f32, 9f32, -7f32])
    );
}

#[test]
fn from_into_arr() {
    let x: [_; 1] = VecND::from([5f32]).into();
    assert_eq!(x, [5f32]);
    let x: [_; 2] = VecND::from([5f32, -2f32]).into();
    assert_eq!(x, [5f32, -2f32]);
    let x: [_; 3] = VecND::from([5f64, -2f64, 7f64]).into();
    assert_eq!(x, [5f64, -2f64, 7f64]);
}
