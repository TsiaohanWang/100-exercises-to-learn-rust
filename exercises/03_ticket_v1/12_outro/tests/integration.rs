use outro_02::Order;

mod outro_02 {
    pub struct Order {
        product_name: String,
        quantity: u32,
        unit_price: u32,
    }

    impl Order {
        pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
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
                unit_price,
            }
        }

        pub fn set_product_name(&mut self, product_name: String) {
            if product_name.is_empty() {
                panic!("The product name cannot be empty!")
            }
            if product_name.len() > 300 {
                panic!("The product name is too long!")
            }

            self.product_name = product_name;
        }
        pub fn set_quantity(&mut self, quantity: u32) {
            if quantity == 0 {
                panic!("The quantity cannot be zero!")
            }

            self.quantity = quantity;
        }
        pub fn set_unit_price(&mut self, unit_price: u32) {
            if unit_price == 0 {
                panic!("The unit price cannot be zero!")
            }

            self.unit_price = unit_price;
        }

        pub fn product_name(&self) -> &String {
            &self.product_name
        }
        pub fn quantity(&self) -> &u32 {
            &self.quantity
        }
        pub fn unit_price(&self) -> &u32 {
            &self.unit_price
        }

        pub fn total(&self) -> u32 {
            self.unit_price * self.quantity
        }
    }
}

// Files inside the `tests` directory are only compiled when you run tests.
// As a consequence, we don't need the `#[cfg(test)]` attribute for conditional compilation—it's
// implied.

#[test]
fn test_order() {
    let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

    assert_eq!(order.product_name(), "Rusty Book");
    assert_eq!(order.quantity(), &3);
    assert_eq!(order.unit_price(), &2999);
    assert_eq!(order.total(), 8997);

    order.set_product_name("Rust Book".to_string());
    order.set_quantity(2);
    order.set_unit_price(3999);

    assert_eq!(order.product_name(), "Rust Book");
    assert_eq!(order.quantity(), &2);
    assert_eq!(order.unit_price(), &3999);
    assert_eq!(order.total(), 7998);
}

// Validation tests
#[test]
#[should_panic]
fn test_empty_product_name() {
    Order::new("".to_string(), 3, 2999);
}

#[test]
#[should_panic]
fn test_long_product_name() {
    Order::new("a".repeat(301), 3, 2999);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("Rust Book".to_string(), 0, 2999);
}

#[test]
#[should_panic]
fn test_zero_unit_price() {
    Order::new("Rust Book".to_string(), 3, 0);
}
