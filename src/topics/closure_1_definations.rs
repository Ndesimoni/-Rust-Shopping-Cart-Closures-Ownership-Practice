/*
================================================================================
                         CLOSURES: DEFINITIONS & EXAMPLES
================================================================================

WHAT IS A CLOSURE?
─────────────────
A closure is an anonymous function that can capture variables from its
surrounding scope (the enclosing function or block). Unlike regular named
functions, closures don't require explicit names and can access local variables
without them being passed as parameters.

Key Characteristics:
- Anonymous (defined without a name)
- Can capture environment variables from surrounding scope
- Can have type annotations (optional - Rust can infer types)
- Stored in variables and passed around like any other value
- Automatically implements one of three traits: Fn, FnMut, or FnOnce
- Syntax: |parameters| -> return_type { body }

PROBLEMS CLOSURES SOLVE
──────────────────────
1. CODE REUSABILITY WITHOUT NAMED FUNCTIONS
   - Define small, one-off functions inline without creating separate functions
   - Reduces code clutter and improves readability

2. CAPTURING EXTERNAL STATE
   - Access variables from enclosing scope without passing them as parameters
   - Makes code more concise and intuitive

3. FUNCTIONAL PROGRAMMING PATTERNS
   - Enable higher-order functions (functions that take or return functions)
   - Essential for functional transformations

4. DEFERRED EXECUTION
   - Store logic to run later rather than executing immediately
   - Useful for callbacks, event handlers, async operations

WHEN TO USE CLOSURES
───────────────────
✓ Short callbacks and event handlers
✓ Functional operations (map, filter, fold on iterators)
✓ Customizing behavior without creating many separate functions
✓ Working with collections and iterators
✓ Avoiding boilerplate for simple, context-specific logic
✓ Accessing local variables without threading them through function calls

EXAMPLES IN THIS FILE
────────────────────
1. multiply_by: Captures 'multiple' and multiplies it by input
2. product: Formats item details (no captures)
3. profits: Calculates profit margin while accessing captured 'multiple'

================================================================================
*/

fn main() {
    // Define a variable in the local scope that will be captured by closures
    let multiple = 5;

    // EXAMPLE 1: Simple closure that CAPTURES a variable from outer scope
    // Syntax: |parameter| -> return_type { body }
    // This closure captures 'multiple' by immutable reference
    // It multiplies the input 'v' by the captured 'multiple' variable
    let multiply_by = |v| -> i32 {
        return multiple * v;
    };

    // EXAMPLE 2: Closure with string parameters and String return type
    // This closure does NOT capture any variables from outer scope
    // It takes two string references and returns a formatted String
    let product = |name: &str, category: &str| -> String {
        return format!("the item name is {name} and it have a product category of {category}");
    };

    // EXAMPLE 3: Closure that captures and performs calculations with side effects
    // This closure captures 'multiple' and uses it in a println!
    // Demonstrates that closures can have multiple statements
    let profits = |cost_price: i32, selling_price: i32| -> i32 {
        println!("the producer manufacture this for: {multiple}");
        return selling_price - cost_price;
    };

    // Execute and print results of each closure
    println!("The result is: {}", multiply_by(8));
    println!("The result is: {}", product("pants", "underwear"));
    println!("profits made: {}", profits(10, 13));
}