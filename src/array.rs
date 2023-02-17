use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;

#[inline]
fn transpose_array_of_uninit<T, const N: usize>(arr: [MaybeUninit<T>; N]) -> MaybeUninit<[T; N]> {
    debug_assert_eq!(
        std::mem::size_of::<ManuallyDrop<[MaybeUninit<T>; N]>>(),
        std::mem::size_of::<MaybeUninit<[T; N]>>()
    );
    let arr: ManuallyDrop<[MaybeUninit<T>; N]> = ManuallyDrop::new(arr);
    // SAFETY: MaybeUninit<[T; N]> and ManuallyDrop<[MaybeUninit<T>; N]> have the same layout
    unsafe { std::mem::transmute_copy(&arr) }
}

#[inline]
pub(crate) fn arr_zip_map<T, S, O, F: Fn(T, S) -> O, const N: usize>(
    a: [T; N],
    b: [S; N],
    map: F,
) -> [O; N] {
    // SAFETY: creating completely not initialized array
    let mut data: [MaybeUninit<O>; N] = unsafe { MaybeUninit::uninit().assume_init() };
    for ((v, a), b) in data.iter_mut().zip(a).zip(b) {
        v.write(map(a, b));
    }
    let data = transpose_array_of_uninit(data);
    // SAFETY: all elements has been initialized
    unsafe { data.assume_init() }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::arr_zip_map;

    #[test]
    fn test_arr_zip_map() {
        assert_eq!(
            arr_zip_map([5i8, 8i8], [3u16, 2u16], |a, b| a as i32 + b as i32),
            [8i32, 10i32]
        );

        assert_eq!(
            arr_zip_map(
                [(6f64, "abc".to_string()), (-12345f64, "rust".to_string())],
                [1u8, 100u8],
                |(a1, a2), b| a1 + a2.len() as f64 + b as f64
            ),
            [10f64, -12241f64]
        );

        assert_eq!(
            arr_zip_map([5i32; 0], [12f64; 0], |a, b| a as f64 + b),
            [],
            "empty case"
        );

        let never_arr: [Infallible; 0] = [];
        assert_eq!(
            arr_zip_map([12usize; 0], never_arr, |a, b| a + never_to_num(b)),
            []
        );
        assert_eq!(
            arr_zip_map(never_arr, [12usize; 0], |a, b| number_to_never(b, a)),
            []
        );
    }

    fn never_to_num(_: Infallible) -> usize {
        12
    }

    fn number_to_never(_: usize, _: Infallible) -> Infallible {
        unreachable!()
    }
}
