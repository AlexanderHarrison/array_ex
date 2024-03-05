# Array EX

This crate provides an easy and powerful method of initializing arrays at compile time.

```rust
const ARRAY: [usize; 8] = array_ex::array![
    // specify element type
    usize,

    // add 1, 2, 3
    [1, 2, 3],

    // add zero until 8 elements
    [0; ..8]
];
assert_eq!(
    ARRAY,
    [1, 2, 3, 0, 0, 0, 0, 0],
)
```

There are multiple methods of adding array elements:

```rust
array![usize,
    // directly add elements
    [1, 2, 3, 4],
    [1; 4],

    // cycle all elements n times
    [* [1, 2, 3]; 3]

    // repeat element until array contains n elements
    [0; ..10],

    // cycle elements until array contains n elements
    [* [1, 2, 3]; ..10],

    // add another instantiated array
    [* array![usize, [1, 2, 3], [0; ..5]] ],

    // cycle another instantiated array
    [* array![usize, [1, 2], [0; ..4]]; 3],

    // add another constant array
    [* OTHER_CONSTANT_ARRAY],
];
```

## Why?

By default, rust's compile-time array initialization methods are not very useful.
You either have to manually specify every element (`[a, b, c]`) or you are restricted to only a single element (`[a; n]`).
This macro covers many more use-cases for initializing static arrays.

## Caveats

- The `*` symbol is needed to diambiguate cycling elements with constructing an array of arrays.
    For example, `array![ [usize; 2], [[1, 2]; 3] ]` would not be parsed correctly otherwise.
- Error messages are NOT pretty.
- Unsafe code is used to avoid requiring `Default` to be derived for the element type. 
    The code is safe - arrays are guaranteed to be constructed at compile-time, which automatically checks for UB.
- There may be cases that elements defined by function calls (such as `array![u8, [* gen_array(); 100]]`) will be constructed multiple times.
    This may lead to bloated compile times. You can extract these function calls out to avoid regenerating arrays.
- Constructing large arrays in general will degrade compile times. Construction is generally instantaneous up to 10_000 elements.
