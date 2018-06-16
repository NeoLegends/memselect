# memselect

![Travis](https://img.shields.io/travis/NeoLegends/memselect.svg)

No-std compatible memoizing selectors for Rust.

Memselect allows you to create efficient selectors for memoizing expensive
computations. The selectors can be composed to create higher-level selectors
that benefit from memoization all the way down. Monomorphization ensures
efficient runtime behavior.

## Example
```rust
use memselect::{new1, new2, Selector2};

let mut computations = 0;

{
    let base = new1(|num: u32| num, |num| num * 2);

    let mut selector = new2(
        base, // You can nest selectors
        |num: u32| num * 3,
        |num1, num2| { // This function gets the output of `base` and the fn above
            computations += 1;
            (*num1, *num2)
        },
    );

    assert_eq!(selector.select(2, 3), (4, 9));
    assert_eq!(selector.select(2, 3), (4, 9));
}

// Value was computed only once
assert_eq!(computations, 1);
```