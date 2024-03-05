/// See readme for documentation.
#[macro_export]
macro_rules! array {
    ($t:ty,
        $(
            [ $($tail:tt)* ]
        ),* $(,)?
    ) => {{
        macro_rules! len {
            ($len:ident @ [* $e:expr; ..$arr_len:expr]) => { if $len < $arr_len { $len = $arr_len; } };
            ($len:ident @ [* $e:expr; $arr_len:expr]) => { $len += $e.len() * $arr_len };
            ($len:ident @ [* $e:expr]) => { $len += $e.len() };
            ($len:ident @ [$ele:expr; ..$arr_len:expr]) => { if $len < $arr_len { $len = $arr_len; } };
            ($len:ident @ [$ele:expr; $arr_len:expr]) => { $len += $arr_len };
            ($len:ident @ $arr:expr) => { $len += $arr.len() };
        }

        macro_rules! fill {
            ($t2:ty, $arr:ident, $i:ident @ [* $e:expr; ..$len:expr]) => { 
                const __SECTION_END: usize = $len;
                let mut cycle = 0;
                while $i < __SECTION_END {
                    $arr[$i] = MaybeUninit::new($e[cycle]);
                    $i += 1;
                    cycle += 1;
                    if cycle == $e.len() { cycle = 0; }
                }
            };
            ($t2:ty, $arr:ident, $i:ident @ [* $e:expr; $len:expr]) => { 
                const __SECTION_LEN: usize = $len*$e.len();
                let mut j = 0;
                let mut cycle = 0;
                while j < __SECTION_LEN {
                    $arr[$i] = MaybeUninit::new($e[cycle]);
                    $i += 1;
                    j += 1;
                    cycle += 1;
                    if cycle == $e.len() { cycle = 0; }
                }
            };
            ($t2:ty, $arr:ident, $i:ident @ [* $e:expr]) => { 
                const __SECTION_LEN: usize = $e.len();
                let mut j = 0;
                while j < __SECTION_LEN {
                    $arr[$i] = MaybeUninit::new($e[j]);
                    $i += 1;
                    j += 1;
                }
            };
            ($t2:ty, $arr:ident, $i:ident @ [$ele:expr; ..$len:expr]) => {{
                const __SECTION_ELE: MaybeUninit<$t2> = MaybeUninit::new($ele);
                const __SECTION_END: usize = $len;
                while $i < __SECTION_END {
                    $arr[$i] = __SECTION_ELE;
                    $i += 1;
                }
            }};
            ($t2:ty, $arr:ident, $i:ident @ [$ele:expr; $len:expr]) => {{
                const __SECTION_ELE: MaybeUninit<$t2> = MaybeUninit::new($ele);
                const __SECTION_LEN: usize = $len;
                let mut j = 0;
                while j < __SECTION_LEN {
                    $arr[$i] = __SECTION_ELE;
                    $i += 1;
                    j += 1;
                }
            }};
            ($t2:ty, $arr:ident, $i:ident @ $src:expr) => {{
                let mut j = 0;
                while j < $src.len() {
                    $arr[$i] = MaybeUninit::new($src[j]);
                    $i += 1;
                    j += 1;
                }
            }};
        }

        #[allow(unused_comparisons)]
        const __LEN: usize = {
            let mut _len = 0;
            $({len!(_len @ [ $($tail)* ]);})*
            _len
        };

        #[allow(long_running_const_eval)]
        const __ARR: [$t; __LEN] = {
            use std::mem::MaybeUninit;
            // SAFETY: undefined memory is verified during compile-time execution
            let mut arr = unsafe { MaybeUninit::<[MaybeUninit::<$t>; __LEN]>::uninit().assume_init() };
            let mut i = 0;

            $({fill!($t, arr, i @ [ $($tail)* ]);})*

            unsafe { std::mem::transmute::<[MaybeUninit::<$t>; __LEN], [$t; __LEN]>(arr) }
        };

        __ARR
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_1() {
        assert_eq!(
            array![usize, [1; 8]],
            [1usize; 8]
        )
    }

    #[test]
    fn repeat_n() {
        assert_eq!(
            array![usize, [1; 2], [2; 5], [3; 0], [4; 3]],
            [1usize, 1, 2, 2, 2, 2, 2, 4, 4, 4]
        )
    }

    #[test]
    fn repeat_long() {
        assert_eq!(
            array![usize, [2; 10_000]],
            [2; 10_000]
        )
    }
    
    #[test]
    fn fill_1() {
        assert_eq!(
            array![usize, [0; ..10]],
            [0; 10]
        )
    }

    #[test]
    fn fill_2() {
        assert_eq!(
            array![usize, [2; ..5], [0; ..10]],
            [2, 2, 2, 2, 2, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn fill_unused() {
        assert_eq!(
            array![usize, [2; ..5], [0; ..5]],
            [2, 2, 2, 2, 2]
        )
    }

    #[test]
    fn fill_before() {
        assert_eq!(
            array![usize, [2; ..5], [0; ..2]],
            [2, 2, 2, 2, 2]
        )
    }

    #[test]
    fn cycle_1() {
        assert_eq!(
            array![usize, [*[1]; 5]],
            [1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn cycle_2() {
        assert_eq!(
            array![usize, [*[1, 2]; 3]],
            [1, 2, 1, 2, 1, 2]
        )
    }

    #[test]
    fn cycle_3() {
        assert_eq!(
            array![usize, [*[1, 2, 3, 4]; ..2]],
            [1, 2]
        )
    }

    #[test]
    fn cycle_4() {
        assert_eq!(
            array![usize, [*[1, 2]; ..5]],
            [1, 2, 1, 2, 1]
        )
    }

    #[test]
    fn cycle_5() {
        assert_eq!(
            array![usize, [*[1, 2]; ..0]],
            []
        )
    }

    #[test]
    fn cycle_6() {
        assert_eq!(
            array![usize, [*[1, 2]; 0]],
            []
        )
    }

    #[test]
    fn cycle_7() {
        assert_eq!(
            array![usize, [*[1]; 5]],
            [1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn cycle_8() {
        assert_eq!(
            array![usize, [*[1]; ..5]],
            [1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn arr_fill() {
        assert_eq!(
            array![[usize; 2], [[1, 2]; 5]],
            [[1, 2]; 5]
        )
    }

    #[test]
    fn arr_cycle() {
        assert_eq!(
            array![[usize; 2], [*[[1, 2], [3, 4]]; 2]],
            [[1, 2], [3, 4], [1, 2], [3, 4]]
        )
    }

    #[test]
    fn concat_1() {
        assert_eq!(
            array![usize, [0, 1, 2], [1, 2, 3]],
            [0, 1, 2, 1, 2, 3]
        )
    }

    #[test]
    fn concat_2() {
        assert_eq!(
            array![usize, [0; ..4], [1, 2, 3], [0; ..8]],
            [0, 0, 0, 0, 1, 2, 3, 0]
        )
    }

    #[test]
    fn all_1() {
        assert_eq!(
            array![usize,
                [*[1, 2]; 1],
                [*[3, 4]; ..4],
            ],
            [1, 2, 3, 4]
        )
    }

    #[test]
    fn all_2() {
        assert_eq!(
            array![usize,
                [0; 2],
                [1; 1],
                [*[2, 3]; 2],
                [*[4, 5]; ..8],
            ],
            [0, 0, 1, 2, 3, 2, 3, 4]
        )
    }

    #[test]
    fn all_3() {
        assert_eq!(
            array![usize,
                [* [1, 2]; ..1],
                [*[3, 4]; 2],
                [2; 1],
            ],
            [1, 3, 4, 3, 4, 2]
        )
    }

    #[test]
    fn all_4() {
        assert_eq!(
            array![usize,
                [* [1, 2]; ..0],
                [* [1, 2]; 0],
                [1; 0],
                [1; ..0],
            ],
            []
        )
    }

    #[test]
    fn non_clone() {
        #[derive(Debug, PartialEq)]
        struct NonClone { pub a: usize }
        const NON_CLONE: NonClone = NonClone { a: 12 };

        assert_eq!(
            array![NonClone, [NonClone { a: 12 }; 4]],
            [NON_CLONE; 4]
        )
    }

    #[test]
    fn nested_1() {
        assert_eq!(
            array![usize,
                [* array![usize, [1,2,3], [0; ..5]]],
                [1],
            ],
            [1, 2, 3, 0, 0, 1]
        )
    }

    #[test]
    fn nested_2() {
        assert_eq!(
            array![usize,
                [* array![usize, [1,2,3], [0; ..5]]; 2]
            ],
            [1, 2, 3, 0, 0, 1, 2, 3, 0, 0]
        )
    }

    #[test]
    fn nested_3() {
        const ARR: [usize; 3] = [1, 2, 3];
        assert_eq!(
            array![usize,
                [* ARR],
                [0; ..4],
                [* ARR],
                [0; ..8],
            ],
            [1, 2, 3, 0, 1, 2, 3, 0]
        )
    }
    
    #[test]
    fn nested_4() {
        const ARR_1: [usize; 3] = [1, 2, 3];
        const ARR_2: [usize; 3] = [2, 3, 4];

        assert_eq!(
            array![[usize; 3], [* [ARR_1, ARR_2]; 2] ],
            [[1, 2, 3], [2, 3, 4], [1, 2, 3], [2, 3, 4]]
        )
    }
}
