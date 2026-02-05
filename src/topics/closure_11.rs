// This file demonstrates how to pass FnMut closures to a method,
// allowing the closure to mutate variables from its surrounding environment.

// Automatically implements the Debug trait so we can print Location with {:?}
#[derive(Debug)]
// A struct representing a place with a name and a treasure value
struct Location {
    name: String,     // The name of the city (owned String)
    treasure: i32,    // How much treasure this location holds
}

// Automatically implements Debug for Map as well
#[derive(Debug)]
// Map holds a borrowed slice of Locations. It does NOT own the data.
// The lifetime 'a guarantees that Map cannot outlive the Location data it references,
// preventing dangling references (pointing to freed memory).
struct Map<'a> {
    location: &'a [Location], // A reference to a slice of Location values
}

// Implement methods on Map. The 'a lifetime carries over from the struct definition.
impl<'a> Map<'a> {
    // `explore` is a generic method that accepts any closure matching FnMut(&Location).
    // - `F` is a generic type: the caller decides what closure to pass in.
    // - `&self` means explore borrows the Map immutably (it only reads the locations).
    // - `mut action` is needed because FnMut closures require mutable access to call them,
    //   since they may mutate their captured variables.
    // - `FnMut(&Location)` means: a closure that takes a reference to a Location,
    //   returns nothing, and is allowed to mutate variables it captured from its environment.
    //   We use FnMut instead of Fn because our closures need to modify external variables
    //   (like total_treasure and city_names).
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        // Get the index of the last element in the slice
        let final_index = self.location.len() - 1;
        // Start at the first element
        let mut current_index = 0;

        // Loop through every location in the slice by index
        while current_index <= final_index {
            // Borrow the current location from the slice
            let current_location = &self.location[current_index];
            // Call the closure, passing it a reference to the current location.
            // The closure decides what to do with each location.
            action(current_location);
            // Move to the next index
            current_index += 1
        }
    }
}

fn main() {
    // Create a fixed-size array of two Location values on the stack
    let locations = [
        Location {
            name: String::from("Abu Dhabi"), // String::from creates an owned String from a string literal
            treasure: 5,
        },
        Location {
            name: String::from("Al ain"),
            treasure: 10,
        },
    ];

    // Create a Map that borrows the locations array as a slice.
    // The Map does not own the data — it just points to it.
    let map = Map {
        location: &locations, // &locations converts the array into a borrowed slice &[Location]
    };

    // Declare a mutable variable to accumulate treasure.
    // It must be `mut` because the closure will modify it.
    let mut total_treasure = 0;

    // Pass a closure to explore. This closure captures `total_treasure` by mutable reference.
    // On each iteration, it sets total_treasure to the current location's treasure.
    // NOTE: This uses `=` (assignment), not `+=` (addition), so it overwrites each time.
    // After the loop, total_treasure will be 10 (the last location's value), not 15.
    // If the intent is to sum, this should be `+=` instead of `=`.
    map.explore(|location| {
        total_treasure = location.treasure;
    });

    // Prints: "the total treasures collected are: 10"
    println!("the total treasures collected are: {total_treasure}");

    // Create an empty Vec to collect city names.
    // Must be `mut` because the closure will push into it.
    let mut city_names: Vec<String> = Vec::new();

    // Pass a second closure to explore. This one captures `city_names` by mutable reference.
    // For each location, it clones the name and pushes it into the vector.
    // .clone() is needed because `location.name` is a &String (borrowed), and we need
    // an owned String to store in the Vec. We can't move it out of a reference.
    map.explore(|location| {
        city_names.push(location.name.clone());
    });

    // Prints: total city names:["Abu Dhabi", "Al ain"]
    // {:?} uses the Debug trait to print the Vec contents
    println!("total city names:{:?}", city_names)
}
