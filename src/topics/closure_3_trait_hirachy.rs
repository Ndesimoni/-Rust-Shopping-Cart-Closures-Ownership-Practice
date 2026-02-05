/*
================================================================================
                       CLOSURE TRAIT HIERARCHY IN RUST
================================================================================

OVERVIEW
────────
Every closure in Rust automatically implements one of THREE traits based on
how it captures variables from its surrounding scope. These traits form a
HIERARCHY - understanding this hierarchy is crucial for working with closures,
iterators, and function parameters.

THE THREE CLOSURE TRAITS
────────────────────────

1. Fn - IMMUTABLE BORROW (Most Restrictive)
   - Borrows captured variables by IMMUTABLE REFERENCE (&T)
   - Can be called multiple times
   - Can be passed as parameter multiple times
   - Implements: Fn + FnMut + FnOnce
   - Use when: You only need to READ captured variables

2. FnMut - MUTABLE BORROW (Middle)
   - Borrows captured variables by MUTABLE REFERENCE (&mut T)
   - Can be called multiple times BUT modifies captured variables
   - Can only have ONE active reference at a time
   - Implements: FnMut + FnOnce (but NOT Fn)
   - Use when: You need to MODIFY captured variables between calls

3. FnOnce - MOVE SEMANTICS (Least Restrictive on Usage)
   - TAKES OWNERSHIP of captured variables (moved)
   - Can be called ONLY ONCE
   - Consumes the closure and the captured variables
   - Does NOT implement Fn or FnMut
   - Use when: Closure outlives the original scope or must take ownership

THE TRAIT HIERARCHY (Subtyping Relationship)
──────────────────────────────────────────────

    FnOnce (Most General - Called Once, Takes Ownership)
       ↑
       | Every closure is at least FnOnce
       |
    FnMut (Can Be Called Multiple Times, Borrows Mutably)
       ↑
       | Mutable closures are also FnOnce
       |
    Fn (Can Be Called Multiple Times, Borrows Immutably)
       ↑
       | Immutable closures are also FnMut and FnOnce

Think of it like: Fn ⊆ FnMut ⊆ FnOnce
- Every Fn closure IS-A FnMut (can be used where FnMut is expected)
- Every FnMut closure IS-A FnOnce (can be used where FnOnce is expected)
- Every FnOnce closure is FnOnce (but not FnMut or Fn)

CAPTURE MODES & TRAITS
──────────────────────

How a closure captures determines which trait it implements:

1. DOESN'T CAPTURE → Fn
   Example: |x| x + 1
   - No variables captured → implements Fn
   - Can be called infinite times
   - Most flexible

2. CAPTURES BY IMMUTABLE REFERENCE (&T) → Fn
   Example: |x| x + captured_var (where captured_var never changes)
   - Borrows immutably from outer scope
   - Can be called multiple times safely
   - Original variable still accessible after closure definition

3. CAPTURES BY MUTABLE REFERENCE (&mut T) → FnMut
   Example: |x| { counter += 1; x + counter } (modifying counter)
   - Borrows mutably from outer scope
   - Modifies captured variables
   - Can only be called while no other references exist

4. CAPTURES BY MOVE → FnOnce
   Example: |x| x + captured_var where captured_var is moved into closure
   - Takes ownership of captured variables
   - Moves happen when value type can't be copied (String, Vec, etc)
   - Can only be called once before closure is dropped

IMPORTANT: Rust automatically chooses the LEAST RESTRICTIVE trait possible.
If a closure only reads variables, it implements Fn (not FnMut).

================================================================================
*/

// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 1: Fn TRAIT - IMMUTABLE BORROW
// ═════════════════════════════════════════════════════════════════════════════

fn example_1_fn_trait() {
    println!("\n=== EXAMPLE 1: Fn Trait (Immutable Borrow) ===");

    let multiplier = 5; // Captured by immutable reference

    // This closure ONLY READS 'multiplier', doesn't modify it
    // Therefore, it implements the Fn trait
    let multiply = |x: i32| -> i32 {
        x * multiplier // Just reading, not modifying
    };

    // Can call multiple times ✓
    println!("First call: {}", multiply(3));   // Output: 15
    println!("Second call: {}", multiply(4));  // Output: 20
    println!("Third call: {}", multiply(5));   // Output: 25

    // Original variable is still accessible ✓
    println!("multiplier is still accessible: {}", multiplier);

    // Can pass to functions expecting Fn ✓
    apply_function(&multiply, 10);

    // Can use with iterators (which expect Fn) ✓
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|&n| n * multiplier).collect();
    println!("Doubled: {:?}", doubled);
}

// Helper function that expects a Fn closure
fn apply_function<F: Fn(i32) -> i32>(f: &F, input: i32) {
    println!("Result from function: {}", f(input));
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 2: FnMut TRAIT - MUTABLE BORROW
// ═════════════════════════════════════════════════════════════════════════════

fn example_2_fnmut_trait() {
    println!("\n=== EXAMPLE 2: FnMut Trait (Mutable Borrow) ===");

    let mut counter = 0; // Captured by MUTABLE reference

    // This closure MODIFIES 'counter' on each call
    // Therefore, it implements FnMut trait (not Fn)
    let increment = |x: i32| -> i32 {
        counter += 1;        // MODIFYING captured variable!
        println!("Call #{}: adding {}", counter, x);
        x + counter
    };

    // Can call multiple times (unlike FnOnce)
    println!("Result 1: {}", increment(10)); // counter becomes 1
    println!("Result 2: {}", increment(20)); // counter becomes 2
    println!("Result 3: {}", increment(30)); // counter becomes 3

    // counter was modified by the closure
    println!("Final counter value: {}", counter); // Output: 3

    // Can't use with immutable iterators (like .map() which expects Fn)
    // let results: Vec<_> = vec![1,2,3].iter().map(increment).collect();
    // ↑ ERROR: increment is FnMut, but map() expects Fn

    // But CAN use with mutable iteration patterns
    apply_function_mut(&mut increment, 5);
}

// Helper function that expects an FnMut closure
fn apply_function_mut<F: FnMut(i32) -> i32>(f: &mut F, input: i32) {
    println!("First call: {}", f(input));
    println!("Second call: {}", f(input + 1));
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 3: FnOnce TRAIT - TAKES OWNERSHIP (MOVE)
// ═════════════════════════════════════════════════════════════════════════════

fn example_3_fnonce_trait() {
    println!("\n=== EXAMPLE 3: FnOnce Trait (Move Semantics) ===");

    let name = String::from("Product A"); // Expensive to copy, must be MOVED

    // This closure MOVES 'name' into itself
    // String is not Copy, so it's moved, not borrowed
    // Therefore, implements ONLY FnOnce trait
    let describe = || {
        println!("Product: {}", name); // Consumes the String
    };

    // Can call ONLY ONCE ✓
    describe(); // First call - consumes 'name'

    // Original variable is CONSUMED (moved into closure)
    // println!("{}", name);
    // ↑ ERROR: name has been moved into the closure!

    // Can't call again ✓
    // describe();
    // ↑ ERROR: closure has already been called, name is gone!

    println!("\n--- Using FnOnce with function ---");

    let message = String::from("Hello, Rust!");

    // Use with function that consumes the closure
    let result = call_once(|| {
        println!("Executing FnOnce closure: {}", message);
        42
    });
    println!("Result: {}", result);
}

// Function that takes ownership of closure (FnOnce)
fn call_once<F: FnOnce() -> i32>(f: F) -> i32 {
    f() // Can call exactly once, then closure is dropped
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 4: FORCED MOVE - MAKING Fn BECOME FnOnce
// ═════════════════════════════════════════════════════════════════════════════

fn example_4_forced_move() {
    println!("\n=== EXAMPLE 4: Forced Move (move keyword) ===");

    let data = vec![1, 2, 3, 4, 5];

    // WITHOUT 'move' - data is borrowed (Fn trait)
    let closure1 = || {
        println!("Data in closure1: {:?}", data);
    };
    println!("Can still use data: {:?}", data); // ✓ data is still here
    closure1();

    println!();

    // WITH 'move' - data is MOVED into closure (FnOnce)
    let closure2 = move || {
        println!("Data in closure2 (moved): {:?}", data);
    };
    // println!("Can still use data: {:?}", data);
    // ↑ ERROR: data was moved into closure2!
    closure2();
    // closure2();
    // ↑ ERROR: Can only call once - closure was consumed

    println!("\nThe 'move' keyword forces capture by value (move), not reference");
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 5: COMPARING ALL THREE TRAITS SIDE-BY-SIDE
// ═════════════════════════════════════════════════════════════════════════════

fn example_5_comparison() {
    println!("\n=== EXAMPLE 5: Side-by-Side Comparison ===");

    // Setup: Three similar closures with different capture behaviors

    // 1. Fn - Only reads external state
    let limit = 100;
    let check_fn = |x: i32| x < limit; // Fn trait
    println!("\nFn Trait Example:");
    println!("  Can call multiple times: {} and {}",
             check_fn(50), check_fn(150));

    // 2. FnMut - Modifies external state
    let mut attempts = 0;
    let mut check_fnmut = |x: i32| {
        // Fn(i32) -> bool { x < limit }; // Fn trait
        attempts += 1;
        println!("  Attempt #{}", attempts);
        x < limit
    }; // FnMut trait
    println!("\nFnMut Trait Example:");
    check_fnmut(50);
    check_fnmut(150);
    println!("  Total attempts: {}", attempts);

    // 3. FnOnce - Takes ownership
    let config = String::from("settings.toml");
    let load_config = move || {
        println!("  Loading: {}", config); // Consumes config
    }; // FnOnce trait
    println!("\nFnOnce Trait Example:");
    load_config(); // Can call once
    // load_config(); // ← ERROR: closure already called, config moved
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 6: PRACTICAL USE CASES - ITERATORS AND FUNCTION PARAMETERS
// ═════════════════════════════════════════════════════════════════════════════

fn example_6_practical_use_cases() {
    println!("\n=== EXAMPLE 6: Practical Use Cases ===");

    println!("\n--- Using Fn with .map() (expects Fn) ---");
    let numbers = vec![1, 2, 3, 4, 5];
    let factor = 2;

    // This works because closure only READS factor (Fn trait)
    let doubled: Vec<_> = numbers.iter()
        .map(|&x| x * factor)
        .collect();
    println!("Doubled: {:?}", doubled);

    println!("\n--- Using FnMut with .fold() (expects FnMut) ---");
    let sum = numbers.iter()
        .fold(0, |mut acc, &x| {
            acc += x; // Modifies accumulator
            acc
        });
    println!("Sum: {}", sum);

    println!("\n--- Function taking FnOnce parameter ---");
    let expensive_resource = String::from("huge data");

    execute_once(|| {
        println!("Processing: {}", expensive_resource);
        // Resource is consumed here
    });
}

// Takes ownership of the closure and executes it once
fn execute_once<F: FnOnce()>(closure: F) {
    closure(); // Executes once, closure is dropped
}


// ═════════════════════════════════════════════════════════════════════════════
// EXAMPLE 7: TRAIT BOUNDS - WORKING WITH GENERIC CLOSURES
// ═════════════════════════════════════════════════════════════════════════════

fn example_7_trait_bounds() {
    println!("\n=== EXAMPLE 7: Trait Bounds with Closures ===");

    // Function accepting ANY closure that implements Fn
    // (most restrictive - can be called unlimited times)
    fn apply_many<F: Fn(i32) -> i32>(f: F, count: i32) {
        for i in 0..count {
            println!("Call {}: {}", i + 1, f(i));
        }
    }

    let multiplier = 3;
    apply_many(|x| x * multiplier, 3);

    // Function accepting FnMut (can modify state between calls)
    fn apply_mut_times<F: FnMut(i32)>(mut f: F, count: i32) {
        for i in 0..count {
            f(i);
        }
    }

    let mut total = 0;
    apply_mut_times(|x| {
        total += x;
        println!("Running total: {}", total);
    }, 4);

    // Function accepting FnOnce (one-time use, takes ownership)
    fn apply_once<F: FnOnce() -> String>(f: F) -> String {
        f()
    }

    let result = apply_once(|| String::from("Done!"));
    println!("Result: {}", result);
}


/*
================================================================================
QUICK REFERENCE: WHEN TO USE WHICH TRAIT
════════════════════════════════════════════════════════════════════════════

USE Fn WHEN:
- Closure will be called multiple times
- Only needs to READ captured variables
- Passing closure to functions like .map(), .filter()
- Want most flexibility in how closure is used

USE FnMut WHEN:
- Closure will be called multiple times
- MUST MODIFY captured variables
- Using with .fold(), .for_each()
- Need to accumulate state across calls

USE FnOnce WHEN:
- Closure only needs to be called once
- Must take OWNERSHIP of captured variables
- Crossing async boundaries or thread boundaries
- Using .into_iter() or consuming iterators

THE RULE: Rust chooses the LEAST RESTRICTIVE trait that satisfies the closure's
behavior. This ensures maximum flexibility while maintaining safety.

================================================================================
*/

fn main() {
    println!("CLOSURE TRAIT HIERARCHY - COMPREHENSIVE EXAMPLES\n");

    example_1_fn_trait();
    example_2_fnmut_trait();
    example_3_fnonce_trait();
    example_4_forced_move();
    example_5_comparison();
    example_6_practical_use_cases();
    example_7_trait_bounds();

    println!("\n✓ All examples completed!");
}
