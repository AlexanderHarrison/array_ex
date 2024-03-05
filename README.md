# Array EX

This crate provides an easy and powerful method of initializing arrays at compile time.

```rust
const ARRAY: [usize; 16] = array_ex::array![
    // specify element type
    usize,

    // add 1, 2, 3
    [1, 2, 3],

    // add 3 '4's
    [4; 3],

    // pad with zero until 16 elements
    [0; ..16]
];
```
