// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if product_name.is_empty() {
            panic!("The product name cannot be empty!")
        }
        if product_name.len() > 300 {
            panic!("The product name is too long!")
        }
        if quantity == 0 {
            panic!("The quantity cannot be zero!")
        }
        if unit_price == 0 {
            panic!("The unit price cannot be zero!")
        }

        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    fn product_name(&self) -> &String {
        &self.product_name
    }
    fn quantity(&self) -> u32 {
        self.quantity
    }
    fn unit_price(&self) -> u32 {
        self.unit_price
    }

    
    fn total(&self) -> u32 {
        self.unit_price * self.quantity
    }
}