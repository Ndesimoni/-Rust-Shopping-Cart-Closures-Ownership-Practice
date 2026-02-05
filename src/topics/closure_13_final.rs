// This file demonstrates that regular named functions can be passed
// anywhere a closure trait (Fn, FnMut, FnOnce) is expected.
// In Rust, all functions automatically implement Fn, FnMut, and FnOnce.

// A generic function that accepts any callable `fruit_type` and calls it 3 times.
// - `T` is a generic type representing any closure OR function that matches the bound.
// - `mut fruit_type` is required because the bound is FnMut, and calling an FnMut
//   requires mutable access. Even though `organic_fruits` doesn't actually mutate
//   anything, the signature still demands `mut` because FnMut is the declared bound.
// - `FnMut()` means: takes no arguments, returns nothing, and may mutate captured state.
//   (In this case nothing is captured since we pass a regular function, but the
//   signature is flexible enough to also accept closures that do mutate.)
fn fruits<T>(mut fruit_type: T)
where
    T: FnMut(),
{
    fruit_type(); // First call — prints "African Mango"
    fruit_type(); // Second call — prints "African Mango"
    fruit_type(); // Third call — prints "African Mango"
}

// A regular named function (not a closure). It takes no arguments and returns nothing.
// Because all functions in Rust automatically implement Fn, FnMut, and FnOnce,
// this function satisfies the `FnMut()` bound required by `fruits`.
fn organic_fruits() {
    println!("African Mango") // Prints the text to the console
}

fn main() {
    // Pass the function `organic_fruits` directly to `fruits`.
    // No closure syntax (|| ...) is needed — just the function name without parentheses.
    // Writing `organic_fruits` (no parens) gives a function pointer, not a call.
    // Writing `organic_fruits()` would call it immediately and pass the return value instead.
    // This works because named functions implement all three closure traits (Fn, FnMut, FnOnce).
    // Output:
    //   African Mango
    //   African Mango
    //   African Mango
    fruits(organic_fruits);
}
