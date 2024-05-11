# swap3

[![codecov](https://codecov.io/gh/sunsided/swap3-rs/graph/badge.svg?token=MSO2LJWX6X)](https://codecov.io/gh/sunsided/swap3-rs)

Provides utility functions for simultaneously swapping three values by rotating them
either left (`abc` → `bca`) or right (`abc` → `cab`). These functions can come in handy e.g.
when rotating elements of a binary tree in list representation.

The provided functions work on arbitrary types and do *not* require the type to be `Clone`, `Copy`
or `Default`.

## Examples

For individual references, the `swap3_bca` (rotate left) and `swap3_cab` (rotate right)
functions are available:

```rust
fn swap3_bca() {
    let mut a = 10;
    let mut b = 20;
    let mut c = 30;

    swap3::swap3_bca(&mut a, &mut b, &mut c);
    assert_eq!([a, b, c], [20, 30, 10]);
}
```

For slices, the `swap3_bca_slice` and `swap3_cab_slice` functions can be used:

```rust
use swap3::prelude::*;

fn swap3_bca() {
    let mut vec = vec![10, 20, 30, 40, 50, 60];
    vec.swap3_bca(0, 1, 4); // or swap3_bca_slice(&mut vec, 0, 1, 4)
    assert_eq!(vec, &[20, 50, 30, 40, 10, 60]);
}
```
