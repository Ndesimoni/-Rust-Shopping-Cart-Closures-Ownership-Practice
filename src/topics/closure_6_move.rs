// ======================================================================
// FnOnce — WHAT PROBLEM DOES IT SOLVE?
// ======================================================================
//
// THE PROBLEM:
// In Rust, every value has exactly ONE owner. When a closure captures
// a heap-allocated value (like String, Vec, Box), a question arises:
//   "Who owns this value — the outer function or the closure?"
//
// If the closure TAKES OWNERSHIP (moves the value into itself),
// then the original variable is gone. The closure consumed it.
// Rust needs a way to ENFORCE that this closure is only called once,
// because after the first call, the captured value has been used up.
//
// That's what FnOnce solves:
//   -> It tells the compiler: "this closure takes ownership of its
//      captures, so it can only be called ONE TIME."
//
// WITHOUT FnOnce, Rust would have no way to guarantee memory safety
// when closures consume their captured variables. You could
// accidentally use a value after it's been moved/dropped.
//
// ======================================================================
// THE 3 CLOSURE TRAITS (recap):
//
//   Fn      -> borrows captured values (&T)     -> call many times
//   FnMut   -> mutably borrows (&mut T)         -> call many times
//   FnOnce  -> takes ownership (T)              -> call ONCE only
//
// Every closure implements FnOnce. It's the most general trait.
// Fn and FnMut are subtypes of FnOnce.
// ======================================================================

// -------------------------------------------------------
// EXAMPLE 1: The basic problem FnOnce solves
// -------------------------------------------------------
// Without FnOnce rules, this would be a use-after-move bug:
fn example_1_basic() {
    let name = String::from("nde simon");

    // This closure MOVES `name` into itself and returns it.
    // After calling it once, `name` is gone — it was returned out.
    let give_name = move || name;

    let result = give_name(); // first call: moves `name` out
    println!("Got: {}", result);

    // give_name();  // COMPILE ERROR! closure already consumed `name`
    //               // FnOnce prevents this double-use bug
}

// -------------------------------------------------------
// EXAMPLE 2: Why not just use Fn?
// -------------------------------------------------------
// Fn only BORROWS. Sometimes you NEED to give ownership away.
// Real-world case: sending data to another thread.
fn example_2_threading() {
    let data = vec![1, 2, 3, 4, 5];

    // std::thread::spawn requires FnOnce because:
    //   - The closure runs in a NEW thread
    //   - The new thread needs to OWN the data
    //   - The closure only runs once (thread starts once)
    let handle = std::thread::spawn(move || {
        // `data` is now owned by this thread
        println!("Thread got: {:?}", data);
        // `data` is dropped when this thread ends
    });

    // println!("{:?}", data);  // ERROR! `data` moved to the thread
    handle.join().unwrap();
}

// -------------------------------------------------------
// EXAMPLE 3: FnOnce in function signatures
// -------------------------------------------------------
// When you write a function that accepts a closure, choosing
// the right trait matters:
//
//   fn do_something(f: impl Fn())      -> "I'll call f multiple times"
//   fn do_something(f: impl FnMut())   -> "I'll call f, maybe mutating state"
//   fn do_something(f: impl FnOnce())  -> "I'll call f exactly once"
//
// FnOnce is the MOST FLEXIBLE parameter — it accepts ANY closure,
// because all closures implement FnOnce.

fn run_once(f: impl FnOnce() -> String) -> String {
    f() // call it once, get the result
    // f() would be a compile error here — FnOnce enforced!
}

fn example_3_function_param() {
    let greeting = String::from("Hello, Rust!");

    // This works because run_once promises to call `f` only once
    let result = run_once(move || greeting);
    println!("{}", result);
}

// -------------------------------------------------------
// EXAMPLE 4: Option::unwrap_or_else — real-world FnOnce
// -------------------------------------------------------
// Many standard library functions use FnOnce.
// unwrap_or_else takes FnOnce because:
//   - It only calls the closure if the Option is None
//   - It calls it AT MOST once
//   - The closure might need to move expensive data
fn example_4_unwrap_or_else() {
    let maybe_name: Option<String> = None;

    let fallback = String::from("default user");

    // unwrap_or_else signature: fn unwrap_or_else(self, f: FnOnce() -> T) -> T
    let name = maybe_name.unwrap_or_else(|| fallback);
    //                                      ^^^^^^^^ moves `fallback`

    println!("Name: {}", name);
    // println!("{}", fallback);  // ERROR! moved into the closure
}

// -------------------------------------------------------
// EXAMPLE 5: FnOnce with drop — cleanup patterns
// -------------------------------------------------------
// FnOnce is useful for "do something and clean up" patterns
fn example_5_cleanup() {
    let temp_data = String::from("sensitive info");

    let cleanup = move || {
        println!("Processing: {}", temp_data);
        drop(temp_data); // explicitly destroy the data
        println!("Data has been securely destroyed");
    };

    cleanup();     // processes and destroys the data
    // cleanup();  // ERROR! data was already destroyed, FnOnce prevents this
}

// -------------------------------------------------------
// EXAMPLE 6: Your original code explained
// -------------------------------------------------------
fn example_6_your_code() {
    let names = String::from("nde simon");

    // `move` forces the closure to take ownership of `names`
    // The closure returns `names`, which MOVES it out
    // This makes the closure FnOnce
    let user_name = move || names;

    // First call: `names` is moved out of the closure and returned
    let result = user_name();
    println!("{}", result);

    // println!("{}", names);       // ERROR! `names` was moved into closure
    // println!("{}", user_name()); // ERROR! closure already consumed,
    //                              // it gave away `names` on first call
}

// -------------------------------------------------------
// SUMMARY: What problems does FnOnce solve?
// -------------------------------------------------------
// 1. MEMORY SAFETY: Prevents use-after-move bugs by ensuring
//    closures that consume values can only run once
//
// 2. OWNERSHIP TRANSFER: Allows closures to take full ownership
//    of captured data (needed for threads, async, callbacks)
//
// 3. API DESIGN: Lets function authors express "I'll call this
//    closure exactly once" — the compiler enforces it
//
// 4. ZERO-COST: All of this is checked at compile time.
//    No runtime overhead. No garbage collector needed.
//
// KEY INSIGHT: FnOnce exists because Rust's ownership model
// needs a way to handle closures that CONSUME their captures.
// Without it, you'd either:
//   - Need a garbage collector (like Java/JS)
//   - Risk use-after-free bugs (like C/C++)
//   - Not be able to move data into closures at all

fn main() {
    example_1_basic();
    example_2_threading();
    example_3_function_param();
    example_4_unwrap_or_else();
    example_5_cleanup();
    example_6_your_code();
}
