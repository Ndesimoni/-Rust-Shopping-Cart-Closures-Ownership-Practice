// ======================================================================
// .retain() — FILTER A COLLECTION IN-PLACE USING A CLOSURE
// ======================================================================
//
// WHAT IS IT?
// .retain() is a method on String, Vec, HashSet, and other collections.
// It goes through every element one by one and asks your closure:
//   "Should I KEEP this element?"
//   - Closure returns true  -> KEEP the element
//   - Closure returns false -> REMOVE the element
//
// After .retain() finishes, the collection only contains the elements
// where your closure returned true. Everything else is gone.
//
// It modifies the collection IN-PLACE — it doesn't create a new one.
//
// ======================================================================
// SIGNATURES
// ======================================================================
//
// For String:
//   fn retain<F>(&mut self, f: F)
//   where F: FnMut(char) -> bool
//
//   -> Goes through each CHARACTER in the string
//   -> Your closure receives a `char` and returns true/false
//   -> Characters where you return false are removed
//
// For Vec<T>:
//   fn retain<F>(&mut self, f: F)
//   where F: FnMut(&T) -> bool
//
//   -> Goes through each ELEMENT in the vector
//   -> Your closure receives a REFERENCE to the element (&T)
//   -> Elements where you return false are removed
//
// NOTE: Both use FnMut, NOT Fn. Why?
//   Because .retain() calls the closure MULTIPLE times (once per element),
//   and the closure might need to mutate captured variables (like a counter
//   or a collection to store removed items). FnMut allows this.
//
// ======================================================================
// WHY DOES .retain() EXIST? WHAT PROBLEM DOES IT SOLVE?
// ======================================================================
//
// PROBLEM: You have a collection and want to remove some elements
// based on a condition, WITHOUT creating a new collection.
//
// WITHOUT .retain(), you'd have to:
//
//   1. Create a new collection and copy only the elements you want:
//      let new_vec: Vec<i32> = old_vec.into_iter().filter(|x| x > &5).collect();
//      -> This ALLOCATES a new Vec. Wasteful if you just want to modify in-place.
//
//   2. Manually iterate with indices and remove (error-prone):
//      // indices shift as you remove elements — easy to get bugs!
//      let mut i = 0;
//      while i < vec.len() {
//          if vec[i] < 5 { vec.remove(i); } else { i += 1; }
//      }
//      -> This is O(n²) and easy to mess up.
//
// WITH .retain():
//   vec.retain(|x| x > &5);
//   -> One line. In-place. Efficient. No new allocation. No index bugs.
//
// ======================================================================
// HOW .retain() WORKS STEP BY STEP
// ======================================================================
//
// Given: "PLaY STaTION"
// Call:  game_console.retain(|ch| ch != 'a')
//
//   char 'P' -> ch != 'a'? YES (true)  -> KEEP
//   char 'L' -> ch != 'a'? YES (true)  -> KEEP
//   char 'a' -> ch != 'a'? NO  (false) -> REMOVE
//   char 'Y' -> ch != 'a'? YES (true)  -> KEEP
//   char ' ' -> ch != 'a'? YES (true)  -> KEEP
//   char 'S' -> ch != 'a'? YES (true)  -> KEEP
//   char 'T' -> ch != 'a'? YES (true)  -> KEEP
//   char 'a' -> ch != 'a'? NO  (false) -> REMOVE
//   char 'T' -> ch != 'a'? YES (true)  -> KEEP
//   char 'I' -> ch != 'a'? YES (true)  -> KEEP
//   char 'O' -> ch != 'a'? YES (true)  -> KEEP
//   char 'N' -> ch != 'a'? YES (true)  -> KEEP
//
// Result: "PLY STaTION" becomes "PLY STTION"
//
// ======================================================================

fn main() {
    // ------------------------------------------------------------------
    // YOUR CODE EXAMPLE — EXPLAINED LINE BY LINE
    // ------------------------------------------------------------------

    // A mutable String — must be `mut` because retain modifies it in-place
    let mut game_console = String::from("PLaY STaTION");

    // A mutable String to collect the characters we remove
    // This shows how the closure can CAPTURE and MUTATE outside variables
    let mut deleted_characters = String::new();

    // The closure that retain will call for EACH character:
    //
    // |value|  -> receives one char at a time
    //
    // if value != 'a':
    //   return true  -> KEEP this character in the string
    // else:
    //   push the 'a' into deleted_characters (capturing it from outer scope)
    //   return false -> REMOVE this character from the string
    //
    // This closure is FnMut because it MUTATES `deleted_characters`
    // by pushing characters into it.
    let holding_char = |value| {
        if value != 'a' {
            true   // keep it
        } else {
            deleted_characters.push(value);  // save the removed char
            false  // remove it
        }
    };

    // .retain() iterates through "PLaY STaTION" character by character,
    // calls holding_char for each one, and removes chars where it returns false.
    // After this line:
    //   game_console = "PLY STTION" (both 'a' chars removed)
    //   deleted_characters = "aa" (both 'a' chars saved)
    game_console.retain(holding_char);

    println!("upper case values: {game_console}");
    println!("the deleted values are: {}", deleted_characters);
    // OUTPUT:
    //   upper case values: PLY STTION
    //   the deleted values are: aa

    // ------------------------------------------------------------------
    // EXAMPLE 2: Vec::retain — remove even numbers
    // ------------------------------------------------------------------
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Keep only odd numbers (remove where n % 2 == 0)
    numbers.retain(|n| n % 2 != 0);

    println!("Odd only: {:?}", numbers);
    // OUTPUT: Odd only: [1, 3, 5, 7, 9]
    // The original vec is modified — no new vec created.

    // ------------------------------------------------------------------
    // EXAMPLE 3: Vec::retain — remove short strings
    // ------------------------------------------------------------------
    let mut names = vec!["nde", "a", "simon", "bo", "alexander"];

    // Keep only names with more than 2 characters
    names.retain(|name| name.len() > 2);

    println!("Long names: {:?}", names);
    // OUTPUT: Long names: ["nde", "simon", "alexander"]

    // ------------------------------------------------------------------
    // EXAMPLE 4: String::retain — keep only lowercase letters
    // ------------------------------------------------------------------
    let mut messy = String::from("H3llo W0rld! 123");

    messy.retain(|ch| ch.is_lowercase());

    println!("Lowercase only: {}", messy);
    // OUTPUT: Lowercase only: llorl

    // ------------------------------------------------------------------
    // EXAMPLE 5: Counting removed elements with a mutable closure
    // ------------------------------------------------------------------
    let mut scores = vec![95, 42, 88, 31, 76, 15, 99, 60];
    let mut removed_count = 0;

    // Keep scores >= 50, count how many we remove
    scores.retain(|&score| {
        if score >= 50 {
            true  // keep
        } else {
            removed_count += 1;  // count the failure
            false // remove
        }
    });

    println!("Passing scores: {:?}", scores);
    println!("Failed students: {}", removed_count);
    // OUTPUT:
    //   Passing scores: [95, 88, 76, 99, 60]
    //   Failed students: 3

    // ------------------------------------------------------------------
    // EXAMPLE 6: Moving removed items to another collection
    // ------------------------------------------------------------------
    let mut inventory = vec!["sword", "potion", "shield", "potion", "bow", "potion"];
    let mut used_potions = Vec::new();

    // Remove all potions and save them in used_potions
    inventory.retain(|&item| {
        if item == "potion" {
            used_potions.push(item);
            false // remove from inventory
        } else {
            true // keep in inventory
        }
    });

    println!("Inventory: {:?}", inventory);
    println!("Used potions: {:?}", used_potions);
    // OUTPUT:
    //   Inventory: ["sword", "shield", "bow"]
    //   Used potions: ["potion", "potion", "potion"]

    // ------------------------------------------------------------------
    // EXAMPLE 7: String::retain — remove all whitespace
    // ------------------------------------------------------------------
    let mut spaced = String::from("  h e l l o   w o r l d  ");

    spaced.retain(|ch| !ch.is_whitespace());

    println!("No spaces: {}", spaced);
    // OUTPUT: No spaces: helloworld

    // ------------------------------------------------------------------
    // EXAMPLE 8: retain vs filter — what's the difference?
    // ------------------------------------------------------------------
    let mut vec_a = vec![1, 2, 3, 4, 5];

    // .retain() — modifies vec_a IN-PLACE, no new allocation
    vec_a.retain(|n| n % 2 != 0);
    println!("retain result: {:?}", vec_a); // [1, 3, 5]

    let vec_b = vec![1, 2, 3, 4, 5];

    // .filter() — creates a NEW iterator, needs .collect() for a new Vec
    let vec_c: Vec<i32> = vec_b.into_iter().filter(|n| n % 2 != 0).collect();
    println!("filter result: {:?}", vec_c); // [1, 3, 5]

    // Both give the same result, but:
    //   retain -> modifies the original, no extra memory
    //   filter -> creates a new collection, original is consumed or unchanged
    //
    // Use retain when you want to modify in-place.
    // Use filter when you want to create a new collection.
}

// ======================================================================
// SUMMARY: .retain()
// ======================================================================
//
// WHAT: A method that filters a collection IN-PLACE using a closure.
//       Keep elements where the closure returns true, remove the rest.
//
// AVAILABLE ON:
//   String::retain(|char| -> bool)        — filters characters
//   Vec::retain(|&element| -> bool)       — filters elements
//   HashSet::retain(|&element| -> bool)   — filters set items
//   VecDeque::retain(|&element| -> bool)  — filters deque items
//
// WHY IT EXISTS:
//   - Modifies in-place (no new allocation)
//   - Clean one-liner instead of manual index tracking
//   - The closure can capture and mutate outside variables (FnMut)
//     so you can collect removed items, count them, log them, etc.
//
// CLOSURE TRAIT: FnMut (because it's called once per element,
//   and might need to mutate captured variables)
//
// retain vs filter:
//   .retain()  -> modifies collection in-place, no new allocation
//   .filter()  -> creates a new iterator/collection, original unchanged
//
// COMMON PATTERNS:
//   string.retain(|ch| ch.is_alphabetic())     — keep only letters
//   string.retain(|ch| !ch.is_whitespace())    — remove spaces
//   vec.retain(|x| x > &0)                     — keep positive numbers
//   vec.retain(|item| !to_remove.contains(item)) — remove blacklisted items
// ======================================================================
