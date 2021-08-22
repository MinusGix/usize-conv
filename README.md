# usize-conv

**You should just use https://crates.io/crates/usize_cast, this repo was made before I knew an established version already existed. **


This small library came from a desire to have conversions to/from usize from various unsigned integer types that would not error.
Normally you must do:
```rust
let value: u32 = /* value */;

let index: usize = value as usize;
// or
let index: usize = value.try_into().unwrap();
```
The first truncates (I believe?) if the size of a usize is a `u16`. (And if `value` was a `u64` and the target platform was one where `usize` as equivalent to a `u32`...).
The second attempts conversion, but _errors at runtime_.
This isn't exactly nice. I don't want my program running on a 16 bit platform (or even a 32 bit one!) if it has the potential for bad logic or random crashes. I'd prefer nice compile time errors to force myself to really pay attention to what I am doing.

## Usage
```rust
use usize_conv::{FromUsize, IntoUsize};
let value: u32 = /* value */;
let index: usize = value.into_usize();
```
This would error at compile time if `usize`'s size is not >= `32` bits.
```rust
let value = u32::from_usize(index);
```
Would convert a `usize` into a `u32` and error if the `usize`'s size is > `32` bits (ex: `u64`) and thus not able to assuredly fit into a `u32`.
