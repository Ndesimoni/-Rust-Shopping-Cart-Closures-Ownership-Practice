// =============================================================================
// CLOSURES AND MUTATION IN RUST
// =============================================================================
//
// WHAT IS A MUTATING CLOSURE?
// ---------------------------
// A mutating closure is a closure that modifies (mutates) a variable it has
// captured from its surrounding environment. In Rust, closures can capture
// variables in three ways:
//
//   1. By immutable reference (&T)   — implements `Fn`
//   2. By mutable reference (&mut T) — implements `FnMut`
//   3. By value (T)                  — implements `FnOnce`
//
// When a closure needs to CHANGE a captured variable (like pushing to a Vec,
// incrementing a counter, etc.), it captures that variable by **mutable
// reference**. This makes the closure implement the `FnMut` trait.
//
// SYNTAX:
// -------
//   let mut closure_name = |params| { body that mutates captured variable };
//       ^^^
//       The binding itself must be declared `mut` because calling a FnMut
//       closure requires a mutable reference to the closure itself.
//
// THE PROBLEM IT SOLVES:
// ----------------------
// Without mutating closures, you would need to pass mutable data explicitly
// through function parameters every time you want to modify state. Mutating
// closures let you:
//
//   - Encapsulate mutable state alongside the logic that modifies it.
//   - Pass around "stateful functions" that remember and update their context.
//   - Use iterators like `.for_each()`, `.map()`, etc. that need to accumulate
//     or modify external state during iteration.
//
// KEY RUST RULE — BORROWING:
// --------------------------
// Rust's borrow checker enforces that you can have EITHER:
//   - One mutable reference (&mut T), OR
//   - Any number of immutable references (&T)
// ...but never both at the same time.
//
// This means: once a closure captures a variable mutably, you CANNOT use that
// variable directly until the closure is done (dropped or no longer used).
// This prevents data races at compile time.
//
// =============================================================================

fn main() {
    // -------------------------------------------------------------------------
    // STEP 1: Basic setup — create a mutable Vec and modify it directly
    // -------------------------------------------------------------------------
    let mut num = vec![1, 2, 31, 45, 45, 40];

    // `num.push(90)` mutates the Vec in place and returns `()` (unit type).
    // So `x` here is just `()` — push doesn't return the new Vec.
    let x = num.push(90);

    // Grab the value at index 2 (which is 31). This will be used later.
    let index = num[2];

    // Print the full Vec after the push: [1, 2, 31, 45, 45, 40, 90]
    println!("{:?}", num);

    // -------------------------------------------------------------------------
    // STEP 2: A mutating closure that pushes to the captured Vec
    // -------------------------------------------------------------------------
    // This closure captures `num` by MUTABLE REFERENCE because it calls
    // `num.push(...)`, which requires `&mut Vec`.
    //
    // The closure binding must be `let mut` because calling a FnMut closure
    // mutates the closure's internal state (its mutable borrow of `num`).
    //
    // Signature breakdown:
    //   |index|       — takes one parameter
    //   num.push(index) — mutates the captured `num` Vec
    let mut num_arr = |index| num.push(index);

    // Call the closure, pushing `index` (31) cast to u32 into the Vec.
    num_arr(index as u32);

    // Print the Vec after closure call: [1, 2, 31, 45, 45, 40, 90, 31]
    // NOTE: We can use `num` here because the closure `num_arr` is about to
    // be shadowed (re-declared) below, so the mutable borrow ends.
    println!("{:?}", num);

    // -------------------------------------------------------------------------
    // STEP 3: A mutating closure that also prints inside itself
    // -------------------------------------------------------------------------
    // Same idea, but the closure wraps `num.push(index)` inside `println!`.
    // `num.push(index)` returns `()`, so this prints `()`.
    let mut num_arr = |index| println!("{:?}", num.push(index));

    // Calls push AND prints the return value of push (which is `()`)
    num_arr(index as u32);

    // -------------------------------------------------------------------------
    // STEP 4: A mutating closure with NO parameters
    // -------------------------------------------------------------------------
    // This closure takes no arguments — it always pushes the value 80.
    // It still captures `num` by mutable reference.
    //
    // Because there are no parameters, the syntax is just `||`.
    let mut num_arr = || println!("{:?}", num.push(80));

    // Each call pushes 80 onto the Vec and prints `()`
    num_arr(); // pushes 80
    num_arr(); // pushes 80 again
    num_arr(); // pushes 80 a third time

    // -------------------------------------------------------------------------
    // FINAL: Print the Vec after all mutations
    // -------------------------------------------------------------------------
    // The closure `num_arr` is no longer used after this point, so the
    // mutable borrow it held on `num` is released. We can now read `num`
    // with an immutable reference via println!.
    //
    // Expected output: [1, 2, 31, 45, 45, 40, 90, 31, 31, 80, 80, 80]
    println!("{:?}", num);
}

// =============================================================================
// SUMMARY
// =============================================================================
//
// | Concept              | Detail                                            |
// |----------------------|---------------------------------------------------|
// | Mutating closure     | A closure that modifies a captured variable       |
// | Trait                | Implements `FnMut`                                |
// | Capture mode         | By mutable reference (`&mut T`)                   |
// | Binding requirement  | `let mut closure = ...` (must be mut)             |
// | Borrow rule          | While closure is alive, nothing else can use the  |
// |                      | captured variable (no aliasing with &mut)         |
// | Common use cases     | Accumulating results, modifying collections,      |
// |                      | stateful callbacks, iterator side effects         |
// =============================================================================
