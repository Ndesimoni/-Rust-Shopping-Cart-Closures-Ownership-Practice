// ======================================================================
// unwrap_or_else — LAZY FALLBACK USING CLOSURES
// ======================================================================
//
// WHAT IS IT?
// A method you call on an Option or Result. It's a safe way to get
// the value out, with a backup plan (a closure) if there's nothing inside.
//
// STEP BY STEP HOW IT WORKS:
//
// Step 1: You have a variable that might or might not contain a value.
//         Option can be Some(value) or None.
//         Result can be Ok(value) or Err(error).
//
// Step 2: You call .unwrap_or_else() on it and pass in a closure.
//         That closure is your backup plan.
//
// Step 3: Rust looks inside the Option/Result:
//
//         IF it's Some(value) or Ok(value):
//           -> Rust pulls out the value and gives it to you.
//           -> The closure you passed in? Completely ignored.
//              Never runs. Thrown away.
//
//         IF it's None or Err(e):
//           -> There's no value, so Rust calls your closure.
//           -> Whatever the closure returns becomes the value you get back.
//           -> (For Result, Rust passes the error INTO the closure so
//              you can look at it and decide what to do.)
//
// Step 4: Either way, you ALWAYS get a value back. Your code never crashes.
//
// SIGNATURE (for Option):
//   fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T
//
// SIGNATURE (for Result):
//   fn unwrap_or_else<F: FnOnce(E) -> T>(self, f: F) -> T
//                           ^^^ Result passes the ERROR to your closure!
//
// WHY FnOnce?
// The closure is called AT MOST once (only when None/Err), so
// FnOnce is the right trait — it accepts any closure.
//
// ======================================================================
// WHY A CLOSURE AND NOT JUST A REGULAR VALUE? (LAZY vs EAGER)
// ======================================================================
//
// Because the closure is LAZY — it only runs when needed.
//
// unwrap_or("hello".to_string())
//   -> ALWAYS allocates the String, even if the Option is Some
//   -> The default value is computed RIGHT NOW, before Rust even
//      checks if the Option is Some or None. WASTEFUL.
//
// unwrap_or_else(|| "hello".to_string())
//   -> Allocates ONLY if the Option is None
//   -> The closure sits there doing nothing until Rust actually
//      needs it. EFFICIENT.
//
// When the Option is Some, unwrap_or_else skips the closure entirely.
// This matters when the fallback is expensive (allocations, I/O, etc.)
//
// ======================================================================
// unwrap_or_else vs unwrap_or vs unwrap — COMPARISON
// ======================================================================
//
// unwrap()
//   - PANICS if None/Err. Crashes your program.
//   - Use only when you're 100% sure the value exists.
//   let x: Option<i32> = None;
//   x.unwrap();  // PANIC! thread 'main' panicked
//
// unwrap_or(default)
//   - Returns `default` if None/Err.
//   - The default is ALWAYS computed, even if not needed (EAGER).
//   - Good for cheap defaults (literals, simple values).
//   let x: Option<i32> = None;
//   x.unwrap_or(0);  // returns 0
//
// unwrap_or_else(|| closure)
//   - Returns the closure's result if None/Err.
//   - The closure is ONLY called when needed (LAZY).
//   - Good for EXPENSIVE defaults (database calls, computations, allocations).
//   let x: Option<String> = None;
//   x.unwrap_or_else(|| String::from("default"));  // only allocates if None
//
// ======================================================================

fn main() {
    // ------------------------------------------------------------------
    // YOUR EXAMPLE 1: Option is Some — closure is NOT called
    // ------------------------------------------------------------------
    // Step 1: option contains Some("nde boy")
    // Step 2: we call .unwrap_or_else() with closure || my_middle_name
    // Step 3: Rust sees Some -> pulls out "nde boy" -> closure IGNORED
    // Step 4: result = "nde boy"
    let option = Some("nde boy");

    let my_middle_name = "simon";
    let result = option.unwrap_or_else(|| my_middle_name);
    println!("{result}");
    // OUTPUT: "nde boy"
    // The closure || my_middle_name never ran. "simon" was never used.

    // ------------------------------------------------------------------
    // YOUR EXAMPLE 2: Option is None — closure IS called
    // ------------------------------------------------------------------
    // Step 1: option is None (no value inside)
    // Step 2: we call .unwrap_or_else() with closure best_beans
    // Step 3: Rust sees None -> calls best_beans closure
    //         -> closure captures bean_type from outer scope
    //         -> checks if bean_type == "black beans" -> yes
    //         -> returns "i love black beans"
    // Step 4: food = "i love black beans"
    let option: Option<&str> = None;
    let bean_type = "black beans";

    let best_beans = || {
        if bean_type == "black beans" {
            return "i love black beans";
        } else {
            return "not the best beans type";
        }
    };

    let food = option.unwrap_or_else(best_beans);
    println!("{food}");
    // OUTPUT: "i love black beans"

    // ------------------------------------------------------------------
    // EXAMPLE 3: Why unwrap_or_else instead of unwrap_or?
    // ------------------------------------------------------------------
    // This function simulates something expensive (imagine a database query)
    fn expensive_default() -> String {
        println!("  (computing expensive default...)");
        String::from("computed after heavy work")
    }

    let has_value: Option<String> = Some(String::from("already here"));

    // BAD — unwrap_or ALWAYS evaluates the argument:
    // let result = has_value.unwrap_or(expensive_default());
    // ^ expensive_default() runs even though has_value is Some!
    //   The String gets allocated and then immediately thrown away. Wasteful.

    // GOOD — unwrap_or_else only calls the closure when needed:
    let result = has_value.unwrap_or_else(|| expensive_default());
    println!("Ex3: {result}");
    // OUTPUT: "already here"
    // Notice: "computing expensive default..." is NOT printed.
    // The closure was never called because the Option was Some.
    // We saved the cost of calling expensive_default().

    // ------------------------------------------------------------------
    // EXAMPLE 4: unwrap_or_else with Result — the closure gets the error
    // ------------------------------------------------------------------
    // With Result, the big difference is: the closure RECEIVES the error.
    // This lets you look at what went wrong and decide what to return.

    // Case A: Ok — closure is not called
    let good_result: Result<i32, String> = Ok(42);
    let value = good_result.unwrap_or_else(|err| {
        // This entire block is skipped because good_result is Ok
        println!("Error occurred: {}", err);
        -1
    });
    println!("Ex4a: {value}"); // 42

    // Case B: Err — closure IS called, and receives "connection failed"
    let bad_result: Result<i32, String> = Err(String::from("connection failed"));
    let value = bad_result.unwrap_or_else(|err| {
        // `err` contains "connection failed" — we can inspect it!
        println!("  Error occurred: {}", err);
        -1 // return a fallback value
    });
    println!("Ex4b: {value}"); // -1

    // ------------------------------------------------------------------
    // EXAMPLE 5: Config/settings pattern — try user setting, fall back
    // ------------------------------------------------------------------
    // This is a very common real-world pattern:
    //   "Try to get X. If it doesn't exist, compute a default."

    fn get_user_setting() -> Option<String> {
        None // simulate: user hasn't configured this
    }

    fn get_system_default() -> String {
        String::from("system-default-theme")
    }

    // get_user_setting() returns None, so the closure runs
    // and calls get_system_default() to produce the fallback
    let theme = get_user_setting().unwrap_or_else(|| get_system_default());
    println!("Ex5: Theme = {theme}");
    // OUTPUT: "system-default-theme"

    // ------------------------------------------------------------------
    // EXAMPLE 6: Closure that captures mutable state
    // ------------------------------------------------------------------
    // The closure can capture and modify variables from the outer scope.
    // Here we track how many times the fallback was used.
    let mut counter = 0;

    let values: Vec<Option<i32>> = vec![Some(10), None, Some(30), None, None];

    let results: Vec<i32> = values
        .into_iter()
        .map(|opt| {
            opt.unwrap_or_else(|| {
                counter += 1;        // increment every time we use fallback
                counter * 100        // return 100, 200, 300...
            })
        })
        .collect();

    println!("Ex6: {:?}", results);
    // OUTPUT: [10, 100, 30, 200, 300]
    //
    // Walk through:
    //   Some(10)  -> unwrap gives 10, closure skipped
    //   None      -> closure runs: counter=1, returns 100
    //   Some(30)  -> unwrap gives 30, closure skipped
    //   None      -> closure runs: counter=2, returns 200
    //   None      -> closure runs: counter=3, returns 300

    // ------------------------------------------------------------------
    // EXAMPLE 7: unwrap_or_else is just cleaner syntax for match
    // ------------------------------------------------------------------
    // These two blocks do the EXACT same thing:
    let maybe_name: Option<&str> = None;

    // Version A: using unwrap_or_else (clean, one line)
    let name_a = maybe_name.unwrap_or_else(|| "anonymous");

    // Version B: using match (more verbose, same result)
    let name_b = match maybe_name {
        Some(n) => n,
        None => "anonymous",
    };

    println!("Ex7: {} == {}", name_a, name_b);
    // Both produce "anonymous". unwrap_or_else is just shorter.
}

// ======================================================================
// WHEN TO USE unwrap_or_else:
// ======================================================================
//
// USE IT WHEN:
// 1. You have an Option/Result and need a fallback value
// 2. The fallback is EXPENSIVE (allocations, I/O, computations)
//    -> the closure only runs when needed, saving work
// 3. You want to CAPTURE variables from the surrounding scope
//    in your fallback logic (closures can do this, plain values can't)
// 4. You need to INSPECT the error (Result version passes the error
//    into your closure so you can log it, check it, etc.)
// 5. You want cleaner code than a match/if-let block
//
// DON'T USE IT WHEN:
// 1. The fallback is a simple literal -> use unwrap_or() instead
//      option.unwrap_or(0)           // no need for a closure here
// 2. You want to panic on None -> use unwrap() or expect()
// 3. You want to return an error -> use the ? operator instead
//
// REAL-WORLD USES IN RUST:
// - Default config:    config.get("key").unwrap_or_else(|| default())
// - Error recovery:    result.unwrap_or_else(|e| log_and_default(e))
// - Caching:           cache.get(key).unwrap_or_else(|| compute(key))
// - Env variables:     env::var("PORT").unwrap_or_else(|_| "8080".into())
// - HashMap defaults:  map.get(key).cloned().unwrap_or_else(|| Vec::new())
// ======================================================================
