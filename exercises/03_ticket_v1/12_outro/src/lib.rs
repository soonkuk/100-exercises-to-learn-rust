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

pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u16) -> Self {
        let order = Order{
            product_name,
            quantity,
            unit_price,
        };

        order.validate_product_name();
        order.validate_quantity();
        order.validate_unit_price();

        order
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn set_product_name(&mut self, product_name: String) {
        self.product_name = product_name;
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn set_quantity(&mut self, quantity: u16) {
        self.quantity = quantity;
    }
    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    pub fn set_unit_price(&mut self, unit_price: u16) {
        self.unit_price = unit_price
    }

    pub fn total(&self) -> u16 {
        self.quantity * self.unit_price
    }

    fn validate_product_name(&self) {
        if self.product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if self.product_name.len() > 300 {
            panic!("Product name cannot be longer than 300");
        }
    }

    fn validate_quantity(&self) {
        if self.quantity == 0 {
            panic!("Quantity cannot be zero");
        }
    }

    fn validate_unit_price(&self) {
        if self.unit_price == 0 {
            panic!("UnitPrice cannot be zero");
        }
    }
}