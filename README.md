# Parity

Provides an interface to check the evenness or oddness of a value.

Normally, you would just use the modulus operator to check if a number is even or odd. Some languages like ruby allow calling a method name `even?` or `odd?`. This crate provides `is_even` and `is_odd` methods on all primitive numeric types. Now you can write code like this:

```rust
// importing the trait is required to use the method on primitives
use parity::Parity;

for i in 0..100 {
  if i.is_even() {
    println("{i}");
  }
}

// or even like this
let x : Vec<_> = (0..100).map(u32::is_even).collect();

```

The implementation of the trait is very simple:

```rust
pub trait Parity {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}
impl Parity for i32 {
    #[inline]
    fn is_even(&self) -> bool {
        *self & 1 == 0
    }
    #[inline]
    fn is_odd(&self) -> bool {
        *self & 1 != 0
    }
}
// implemented for all numeric primitive types
// ...
```

The inline attribute is used to allow optimization across crate boundaries. This means there is no extra cost associated with using these methods compared to directly using the modulus or bitwise and operator.

