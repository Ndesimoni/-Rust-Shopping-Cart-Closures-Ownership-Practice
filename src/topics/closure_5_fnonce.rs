// ======================================================================
// FnOnce - THE CLOSURE THAT CONSUMES CAPTURED VARIABLES
// ======================================================================
//
// Rust has 3 closure traits, forming a hierarchy:
//   FnOnce  (most general - ALL closures implement this)
//     └─ FnMut  (closures that don't consume captures)
//         └─ Fn  (closures that only read captures)
//
// FnOnce = the closure MOVES (takes ownership of) a captured variable
//          into the closure body. Because ownership is transferred,
//          the closure can only be called ONCE — after that, the
//          captured value is gone (consumed/dropped).
//
// HOW RUST DECIDES WHICH TRAIT A CLOSURE GETS:
//   - Fn:     closure only READS captured variables (borrows &T)
//   - FnMut:  closure MODIFIES captured variables (borrows &mut T)
//   - FnOnce: closure MOVES captured variables (takes ownership of T)
//
// KEY RULE: once a closure moves a value, that value no longer exists
//           in the outer scope. The closure owns it now.
// ======================================================================

fn main() {
    // ------------------------------------------------------------------
    // EXAMPLE 1: Fn closure (for comparison)
    // `number` is i32 which implements Copy, so it gets COPIED into
    // the closure, not moved. This means `multiply` implements Fn
    // and can be called multiple times.
    // ------------------------------------------------------------------
    let number = 35;

    let multiply = || number;  // copies `number` (i32 is Copy)
    let a = multiply();        // works fine
    let b = multiply();        // works fine again — Fn, not FnOnce
    println!("{} and {}", a, b);

    // ------------------------------------------------------------------
    // EXAMPLE 2: FnOnce closure — moving a String
    // `String` does NOT implement Copy. When the closure captures
    // `name` and returns it, ownership MOVES into the closure.
    // This makes `show_name` an FnOnce closure.
    // ------------------------------------------------------------------
    let name = String::from("nde simon");

    let show_name = || name;   // MOVES `name` into the closure
    // `name` is no longer valid here! The closure owns it.

    let result = show_name();  // consumes the closure, returns the String
    println!("{}", result);

    // show_name();  // ERROR! can't call again — FnOnce already consumed
    // println!("{name}");  // ERROR! `name` was moved into the closure

    // ------------------------------------------------------------------
    // EXAMPLE 3: FnOnce with explicit move and drop
    // Another common FnOnce pattern: the closure drops the value
    // inside its body.
    // ------------------------------------------------------------------
    let data = String::from("important data");

    let consume = || {
        let _moved = data;     // takes ownership of `data`
        println!("consumed: {}", _moved);
        // `_moved` is dropped here at the end of the closure
    };

    consume();                 // works once
    // consume();              // ERROR! FnOnce — already consumed `data`

    // ------------------------------------------------------------------
    // EXAMPLE 4: Using FnOnce as a function parameter
    // When a function accepts `impl FnOnce()`, it means:
    //   "give me any closure, I'll call it exactly once"
    // ------------------------------------------------------------------
    let message = String::from("hello from FnOnce!");

    fn call_once(f: impl FnOnce()) {
        f();
        // f();  // ERROR! FnOnce can only be called once
    }

    call_once(|| {
        println!("{}", message);  // moves `message` into closure
    });

    // ------------------------------------------------------------------
    // SUMMARY:
    // ------------------------------------------------------------------
    // | Trait  | Can call | Captures       | Example              |
    // |--------|----------|----------------|----------------------|
    // | Fn     | many     | &T (borrow)    | || println!("{}", x) |
    // | FnMut  | many     | &mut T (mut)   | || vec.push(1)       |
    // | FnOnce | once     | T (move/own)   | || drop(x)           |
    //
    // WHEN TO USE FnOnce:
    // - When your closure needs to take ownership of a non-Copy value
    // - When accepting callbacks that only need to run once
    // - Thread::spawn requires FnOnce (closure runs once in new thread)
    // - Common in APIs: .unwrap_or_else(|| default_value)
}
