#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    item: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let mut start_index = 0;

        while start_index < self.item.len() {
            operation(&mut self.item[start_index]);
            start_index += 1
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self)
    }
}

fn main() {
    let mut items = ShoppingCart {
        item: vec![
            SupermarketItem {
                name: String::from("APPLE"),
                price: 3.99,
            },
            SupermarketItem {
                name: "BANANA".to_string(),
                price: 2.99,
            },
        ],
    };

    items.traverse_items(|item| item.price *= 0.85);

    items.traverse_items(|items_name| {
        items_name.name = items_name.name.to_lowercase();
    });

    let mut total_price = 0.0;

    items.checkout(|mut cart| {
        println!("{:?}", cart);

        cart.traverse_items(|items| total_price += items.price);
        println!("{:.2}", total_price)
    });
}
