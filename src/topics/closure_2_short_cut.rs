/*
================================================================================
                    CLOSURE CAPTURE & TYPE INFERENCE EXERCISE
================================================================================

WHAT THIS EXERCISE DEMONSTRATES
────────────────────────────────
This file explores how Rust closures handle variable CAPTURE from their
surrounding scope and how TYPE INFERENCE works with closures. It specifically
demonstrates the different ways closures can interact with variables.

THREE CLOSURE CAPTURE MODES
──────────────────────────

1. CAPTURE BY IMMUTABLE REFERENCE (&T)
   - Closure borrows the variable without taking ownership
   - Variable can still be used after closure definition
   - Most efficient and flexible - least restrictive

2. CAPTURE BY MUTABLE REFERENCE (&mut T)
   - Closure can modify the captured variable
   - Prevents other access while closure is active

3. CAPTURE BY MOVE
   - Closure takes OWNERSHIP of the variable
   - Original variable can no longer be used
   - Necessary when closure outlives the original scope

THE PROBLEM THIS SOLVES
───────────────────────
Understanding closure capture is crucial for:
- Avoiding borrow checker errors
- Writing memory-safe code that compiles
- Knowing when variables are available vs. moved
- Designing closures that work with iterators and callbacks
- Working with async/await where closures cross scope boundaries

KEY CONCEPTS IN THIS EXERCISE
────────────────────────────

1. TYPE INFERENCE
   - Rust infers closure parameter types automatically
   - Once inferred, the type is LOCKED IN for that closure
   - Passing different types later causes a compiler error

2. SHADOWING vs CAPTURE
   - Parameter with same name as outer variable SHADOWS it
   - The parameter becomes a new binding, not the outer variable

3. PARAMETER TYPES
   - Can be explicit: |param: u32| or inferred: |param|
   - Once the closure's type is inferred, it stays that way

================================================================================
*/

fn main() {
    // Variables in outer scope available for capture
    let name = "pants";
    let price = 20;

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE 1: ZERO-PARAMETER CLOSURE THAT CAPTURES
    // ─────────────────────────────────────────────────────────────────────
    // Closure with NO parameters - captures 'name' by immutable reference
    // Returns the captured 'name' variable when called
    // Demonstrates: Closure can access outer scope without parameters
    let product_name = || name;
    product_name();

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE 2: PARAMETER SHADOWS OUTER VARIABLE
    // ─────────────────────────────────────────────────────────────────────
    // Closure has a parameter NAMED 'name' (same as outer variable)
    // The parameter SHADOWS the outer 'name' - it's a different binding
    // Returns the PARAMETER value, not the captured outer 'name'
    // Demonstrates: Parameter names shadow outer variables
    let product_name_2 = |name| name;
    product_name_2(name);

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE 3: EXPLICIT TYPE ANNOTATION
    // ─────────────────────────────────────────────────────────────────────
    // Closure parameter explicitly typed as u32
    // Takes a u32 and returns it
    // Redefines the binding 'product_name_2' (shadowing the previous one)
    let product_name_2 = |product_price: u32| product_price;
    product_name_2(price);


    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE 4: TYPE INFERENCE - FIRST CALL LOCKS THE TYPE
    // ─────────────────────────────────────────────────────────────────────
    // Closure with NO type annotation - type is inferred from first use
    // First call: product_name_3(price) where price is i32
    //            → Rust infers this closure takes i32
    // IMPORTANT: Once inferred, the type is FIXED for this closure
    // Attempting to pass a different type (like 'name: &str') will fail!
    let product_name_3 = |product_price| product_price;
    product_name_3(price);

    // INTENTIONAL ERROR DEMONSTRATION (commented for compilation):
    // product_name_3(name);
    // ↑ ERROR: Expected i32, found &str
    // This shows type inference locking - Rust inferred i32 from first call

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE 5: EXPLICIT CASTING WITH TYPE-INFERRED CLOSURE
    // ─────────────────────────────────────────────────────────────────────
    // Redefines product_name_3 (shadowing again)
    // First call casts price to i32: price as i32
    // Type is inferred as i32 and locked in
    let product_name_3 = |product_price| product_price;
    product_name_3(price as i32);

    // INTENTIONAL ERROR DEMONSTRATION (commented for compilation):
    // product_name_3(name);
    // ↑ ERROR: Expected i32, found &str
    // Same issue - closure expects i32, but receives &str (string slice)
    // Demonstrates that type inference is strict and unforgiving
}

/*
================================================================================
KEY TAKEAWAYS
════════════════════════════════════════════════════════════════════════════

1. CAPTURE MODES
   - Closures capture from surrounding scope automatically
   - No parameters needed if you just want captured variables
   - Lowest restrictive option (immutable) is default

2. TYPE INFERENCE
   - Rust infers closure parameter types from usage
   - Once inferred, the type is LOCKED IN
   - All calls must match that inferred type
   - This prevents bugs but requires consistency

3. SHADOWING
   - Parameters with same names as outer variables shadow them
   - The parameter becomes a new binding
   - This can prevent accidental capture when you mean to use parameters

4. WHEN THIS MATTERS
   - Working with iterators: .map(|x| x * 2) infers x's type from context
   - Callbacks: Function signature determines closure parameter types
   - Higher-order functions: Type of closure parameter must match expectations

================================================================================
*/
