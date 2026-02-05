// This file demonstrates passing an FnMut closure to a function.
// The closure mutates a variable (a Vec) from its surrounding environment,
// and the function calls that closure multiple times.

// A generic function that takes a closure `duration` and calls it 3 times.
// - `F` is a generic type representing whatever closure the caller passes in.
// - `mut duration` is needed because FnMut closures require mutable access to be called,
//   since they may mutate their captured variables internally.
// - `FnMut()` means: a closure that takes no arguments, returns nothing,
//   and is allowed to mutate variables it captured from its environment.
//   We need FnMut here (not Fn) because the closure pushes into a Vec.
fn working_time<F>(mut duration: F)
where
    F: FnMut(),
{
    duration(); // First call — pushes `hours` into the captured Vec
    duration(); // Second call — pushes again
    duration()  // Third call — pushes a third time. No semicolon, so this is the return value (which is () since push returns nothing)
}

fn main() {
    // An immutable integer. The closure will capture this by copy (i32 implements Copy).
    let hours = 12;

    // These two lines are commented out — they were a previous version that just printed.
    // That version only needed Fn (not FnMut) since println! doesn't mutate captured variables.
    // let time = || println!("{}", hours);
    // working_time(time);

    // Create a mutable, empty Vec<i32>.
    // `mut` is required because the closure will call .push() on it, which mutates the Vec.
    let mut our_vec = Vec::new();

    // Create a closure that captures `our_vec` by mutable reference and `hours` by copy.
    // Each time it's called, it pushes the value 12 into the Vec.
    // This closure implements FnMut because it mutates a captured variable (our_vec).
    // Note: `time` itself doesn't need `mut` here because ownership of the closure
    // (including the mutable borrow of our_vec) transfers into `working_time`.
    let time = || our_vec.push(hours);

    // Pass the closure to working_time, which calls it 3 times.
    // After this call, our_vec has been mutated to contain [12, 12, 12].
    // The mutable borrow of our_vec by the closure ends here when `time` is dropped.
    working_time(time);

    // Print the Vec using Debug formatting {:?}.
    // This works because the closure's mutable borrow of our_vec has ended (the closure was
    // moved into working_time and dropped when that function returned).
    // Output: [12, 12, 12]
    println!("{:?}", our_vec)
}
