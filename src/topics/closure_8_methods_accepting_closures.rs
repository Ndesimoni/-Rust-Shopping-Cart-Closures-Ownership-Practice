// ======================================================================
// METHODS THAT ACCEPT CLOSURES
// ======================================================================
//
// WHAT IS IT?
// A method (or function) that takes a closure as a parameter.
// Instead of hardcoding behavior inside the method, you let the
// CALLER decide what happens by passing in a closure.
//
// Think of it like this:
//   - Normal method: "I do step A, step B, step C."
//   - Method accepting closure: "I do step A, then YOU tell me
//     what to do for step B, then I do step C."
//
// The method defines the STRUCTURE (when to call the closure,
// what to do with the result), and the caller defines the BEHAVIOR
// (what the closure actually does).
//
// ======================================================================
// WHY DO THEY EXIST? WHAT PROBLEM DO THEY SOLVE?
// ======================================================================
//
// PROBLEM: You want to write a reusable method, but part of the
// logic needs to be different every time it's called.
//
// WITHOUT closures, you'd have to:
//   1. Write a separate method for each variation (code duplication)
//   2. Use if/else or match for every possible case (messy, not extensible)
//   3. Use trait objects with dynamic dispatch (slower, more complex)
//
// WITH closures:
//   - The method says: "Give me a function to call, and I'll use it"
//   - The caller passes in whatever logic they want
//   - Zero code duplication. Fully flexible. Zero runtime cost.
//
// REAL-WORLD ANALOGY:
//   A lock doesn't care HOW you get the key.
//   You could type a password, scan your fingerprint, or use a physical key.
//   The lock just says: "Give me something that produces a key,
//   and I'll check if it's correct."
//   That "something that produces a key" = a closure.
//
// ======================================================================
// HOW TO WRITE A METHOD THAT ACCEPTS A CLOSURE
// ======================================================================
//
// SYNTAX:
//   fn method_name<F>(parameter: F)    // F is a generic type
//   where
//       F: FnOnce() -> ReturnType,     // F must be a closure with this shape
//   {
//       // call the closure inside the method body
//       let result = parameter();
//   }
//
// BREAKING IT DOWN:
//
//   <F>                    = "there's a generic type called F"
//   F: FnOnce() -> String  = "F must be a closure that:
//                               - takes no arguments ()
//                               - returns a String
//                               - can be called at least once (FnOnce)"
//
// WHICH TRAIT TO USE?
//   F: Fn()       = "I'll call this closure multiple times, it only reads"
//   F: FnMut()    = "I'll call this closure multiple times, it may mutate"
//   F: FnOnce()   = "I'll call this closure exactly once"
//
// RULE OF THUMB: Use the most general trait your method needs:
//   - If you only call the closure once -> FnOnce (most flexible for caller)
//   - If you call it multiple times -> FnMut or Fn
//
// ======================================================================

use std::io::stdin;

// ======================================================================
// YOUR CODE EXAMPLE — EXPLAINED LINE BY LINE
// ======================================================================

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

// The Vault has ONE method: unlock
// It doesn't know HOW the user will provide the password.
// It just says: "Give me a closure that returns a String,
// and I'll check if it matches."
impl Vault {
    // ---- BREAKING DOWN THIS SIGNATURE: ----
    //
    // fn unlock<F>        -> "unlock" is a method with a generic type F
    // (self, procedure: F) -> takes ownership of self (consumes the vault)
    //                         and takes a closure called `procedure`
    // -> Option<String>   -> returns Some(treasure) or None
    //
    // where F: FnOnce() -> String
    //   -> F must be a closure that:
    //      - takes NO arguments: ()
    //      - returns a String
    //      - implements FnOnce (can be called once)
    //
    // WHY FnOnce?
    //   Because we only call `procedure()` once (line 1 of the body).
    //   FnOnce is the most flexible — it accepts ANY closure
    //   (Fn, FnMut, or FnOnce closures all work).
    //
    // WHY `self` (not `&self`)?
    //   Because if the password is correct, we MOVE self.treasure out.
    //   After unlocking, the vault is consumed — you can't use it again.
    //   This makes sense: once you open a vault and take the treasure,
    //   the vault is "used up".
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        // Step 1: Call the closure to get the user's password attempt
        let user_password = procedure();

        // Step 2: Compare the attempt with the real password
        if user_password == self.password {
            // Step 3a: Correct! Return the treasure (moved out of self)
            Some(self.treasure)
        } else {
            // Step 3b: Wrong password. Return None.
            None
        }
    }
}

fn main() {
    // ------------------------------------------------------------------
    // USING YOUR VAULT CODE
    // ------------------------------------------------------------------
    // Create a vault with a password and a treasure inside
    let vault = Vault {
        password: String::from("DIFFICULTY PASSED"),
        treasure: String::from("new move unlock,: 🏹 and 🥋"),
    };

    // Create a closure that asks the user for a password.
    // This closure:
    //   - Takes no arguments: ||
    //   - Returns a String (the trimmed user input)
    //   - Reads from stdin — this is the "procedure" for getting a password
    //
    // The vault doesn't care that we're using stdin.
    // We could just as easily pass a closure that returns a hardcoded string,
    // reads from a file, or generates a random password. The vault
    // doesn't know or care — it just calls the closure.
    let password_hack = || {
        let mut user_input = String::new();
        println!("To unlock new moves: INPUT PASSWORD");
        stdin().read_line(&mut user_input).expect("Failed to read input");
        return user_input.trim().to_string();
    };

    // Pass the closure to vault.unlock()
    // The vault calls password_hack(), gets the String back,
    // compares it with "DIFFICULTY PASSED", and returns:
    //   Some("new move unlock,: 🏹 and 🥋") if correct
    //   None if wrong
    let result = vault.unlock(password_hack);
    println!("{:?}", result);

    // ------------------------------------------------------------------
    // EXAMPLE 2: Same vault, different closure — hardcoded password
    // ------------------------------------------------------------------
    // THIS is the power of methods accepting closures:
    // Same method, completely different behavior!
    let vault2 = Vault {
        password: String::from("1234"),
        treasure: String::from("secret document"),
    };

    // This time we pass a simple closure that returns a hardcoded string.
    // No stdin, no user input — just returns "1234".
    let auto_unlock = || String::from("1234");

    let result2 = vault2.unlock(auto_unlock);
    println!("Auto unlock: {:?}", result2);
    // OUTPUT: Auto unlock: Some("secret document")

    // ------------------------------------------------------------------
    // EXAMPLE 3: Closure that captures a variable
    // ------------------------------------------------------------------
    let vault3 = Vault {
        password: String::from("open sesame"),
        treasure: String::from("gold coins"),
    };

    // The closure captures `saved_password` from the outer scope
    let saved_password = String::from("open sesame");
    let use_saved = move || saved_password;

    let result3 = vault3.unlock(use_saved);
    println!("Saved password: {:?}", result3);
    // OUTPUT: Saved password: Some("gold coins")

    // ------------------------------------------------------------------
    // EXAMPLE 4: Writing your own method that accepts a closure
    // ------------------------------------------------------------------
    // Here's a simpler example to see the pattern clearly.

    // A function that repeats an action 3 times
    fn repeat_3_times<F: FnMut()>(mut action: F) {
        action(); // call 1
        action(); // call 2
        action(); // call 3
    }

    let mut count = 0;
    repeat_3_times(|| {
        count += 1;
        println!("  Action called! Count: {}", count);
    });
    // OUTPUT:
    //   Action called! Count: 1
    //   Action called! Count: 2
    //   Action called! Count: 3
    //
    // Note: we used FnMut because the closure MUTATES `count`.
    // And `mut action` because calling a FnMut closure requires &mut self.

    // ------------------------------------------------------------------
    // EXAMPLE 5: Method that transforms data using a closure
    // ------------------------------------------------------------------
    fn transform_name<F: Fn(&str) -> String>(name: &str, transformer: F) -> String {
        transformer(name)
    }

    // Same function, three different behaviors:
    let upper = transform_name("nde simon", |n| n.to_uppercase());
    let loud = transform_name("nde simon", |n| format!("{}!!!", n));
    let first = transform_name("nde simon", |n| {
        n.split_whitespace().next().unwrap_or("").to_string()
    });

    println!("Upper: {}", upper);   // "NDE SIMON"
    println!("Loud: {}", loud);     // "nde simon!!!"
    println!("First: {}", first);   // "nde"

    // ------------------------------------------------------------------
    // EXAMPLE 6: Returning different values based on a condition
    // ------------------------------------------------------------------
    fn get_or_compute<F: FnOnce() -> i32>(cached: Option<i32>, compute: F) -> i32 {
        match cached {
            Some(val) => val,           // use cached value
            None => compute(),          // call closure to compute
        }
    }

    let cached_result = get_or_compute(Some(42), || {
        println!("Computing...");  // this never prints — value is cached
        999
    });
    println!("Cached: {}", cached_result); // 42

    let fresh_result = get_or_compute(None, || {
        println!("  Computing...");  // this DOES print — no cache
        999
    });
    println!("Fresh: {}", fresh_result);  // 999
}

// ======================================================================
// SUMMARY: METHODS ACCEPTING CLOSURES
// ======================================================================
//
// WHAT: A method that takes a closure as a parameter, letting the
//       caller inject custom behavior.
//
// WHY: Avoids code duplication. One method handles the structure,
//      the closure handles the specifics. Infinitely flexible.
//
// HOW:
//   fn my_method<F>(closure: F)
//   where F: FnOnce() -> ReturnType
//   {
//       let result = closure();
//   }
//
// WHICH TRAIT:
//   FnOnce  -> call once         (use for most cases)
//   FnMut   -> call many times   (closure may mutate captured vars)
//   Fn      -> call many times   (closure only reads captured vars)
//
// YOUR VAULT EXAMPLE:
//   The vault method says: "Give me ANY way to get a password"
//   You can pass:
//     - A closure that reads from stdin (user types it)
//     - A closure that returns a hardcoded string (for testing)
//     - A closure that reads from a file
//     - A closure that captures a variable from the outer scope
//   The vault doesn't care. It just calls the closure and checks.
//
// STANDARD LIBRARY EXAMPLES:
//   vec.iter().map(|x| x * 2)         -> map accepts Fn closure
//   vec.iter().filter(|x| x > &5)     -> filter accepts Fn closure
//   vec.sort_by(|a, b| a.cmp(b))      -> sort_by accepts FnMut closure
//   option.unwrap_or_else(|| default)  -> unwrap_or_else accepts FnOnce
//   thread::spawn(|| do_work())        -> spawn accepts FnOnce closure
// ======================================================================
