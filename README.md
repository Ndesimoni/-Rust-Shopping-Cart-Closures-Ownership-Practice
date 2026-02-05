# -Rust-Shopping-Cart-Closures-Ownership-Practice
A small Rust project built to practice ownership, borrowing, and closures by modeling a real-world shopping cart system.  This project focuses on how behavior can be passed into functions safely using Rust’s type system instead of hard-coding logic.


## What This Project Demonstrates

- Structuring real-world data using struct

- Owning collections with Vec<T>

- Applying reusable logic using closures

- Understanding FnMut vs FnOnce

- Enforcing business rules at compile time

- Avoiding unnecessary cloning or shared mutable state

## 🧠 Core Idea

### Instead of writing many functions for different behaviors,
we write one traversal function and pass behavior into it.

This keeps the code:

Flexible

Reusable

Safe


## 🏗️ Project Structure

### Supermarket Item

//#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}


### Shopping Cart

//#[derive(Debug)]
struct ShoppingCart {
    item: Vec<SupermarketItem>,
}

The cart owns all items.
No item exists outside the cart.


## 🔁 Traversing Items with Closures
fn traverse_items<F>(&mut self, mut operation: F)
where
    F: FnMut(&mut SupermarketItem),

What this does 

“Take a function and apply it to every item in the cart.”

This allows different behaviors without rewriting traversal logic.

### Example 1: Apply a Discount
items.traverse_items(|item| item.price *= 0.85);


Applies a 15% discount to every item.

### Example 2: Normalize Item Names
items.traverse_items(|item| {
    item.name = item.name.to_lowercase();
});


Same function, different behavior.

🧾 Checkout and Ownership (FnOnce)
fn checkout<F>(self, operation: F)
where
    F: FnOnce(ShoppingCart),

Why this matters:

self is moved

The cart can’t be reused after checkout

Business rules are enforced by Rust

In real life, once you check out, the cart is gone.

### Example: Calculate Total Price
let mut total_price = 0.0;

items.checkout(|mut cart| {
    cart.traverse_items(|item| total_price += item.price);
    println!("{:.2}", total_price);
});

Rust encourages explicit, safe control flow.

🚀 Why Rust Fits This Problem

Rust forces you to think about:

Who owns the data?

Who is allowed to modify it?

When should data stop existing?

This leads to code that is:

Predictable

Safe

Easy to reason about
