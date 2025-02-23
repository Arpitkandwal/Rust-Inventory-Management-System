use std::collections::HashMap;

// Trait for Inventory Operations
trait Operations {
    fn add_product(&mut self, product: Product) -> Result<(), String>;
    fn delete_product(&mut self, name: &str, id: u32) -> Result<(), String>;
    fn edit_product(&mut self, name: &str, id: u32, new_price: f64, new_quantity: u32) -> Result<(), String>;
    fn display_products(&self);

    fn record_sale(&mut self, name: &str, id: u32, quantity_sold: u32, sale_price: f64) -> Result<(), String>;
    fn display_sales(&self);

    fn record_purchase(&mut self, name: &str, id: u32, quantity_purchase: u32, buy_price: f64) -> Result<(), String>;
    fn display_purchase(&self);
}

// Product Struct
#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Sale{
    product_id: u32,
    product_name: String,
    quantity_sold: u32,
    sale_price: f64,
    total_price: f64,
}
struct Purchase{
    product_id: u32,
    product_name: String,
    quantity_purchase: u32,
    buy_price: f64,
    total_price: f64,
}

// Inventory Struct
struct Inventory {
    products: HashMap<String, Vec<Product>>,
    sales: Vec<Sale>,
    purchases: Vec<Purchase>
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }
}

// Implement Operations Trait for Inventory
impl Operations for Inventory {
    // Add a product to inventory
    fn add_product(&mut self, product: Product) -> Result<(), String> {
        self.products
            .entry(product.name.clone())
            .or_insert(Vec::new()) 
            .push(product);
        Ok(())
    }

    // Delete a product by name and ID
    fn delete_product(&mut self, name: &str, id: u32) -> Result<(), String> {
        match self.products.get_mut(name) {
            Some(products) => match products.iter().position(|p| p.id == id) {
                Some(pos) => {
                    products.remove(pos);
                    if products.is_empty() {
                        self.products.remove(name);
                    }
                    Ok(())
                }
                None => Err(format!("❌ Product with ID {} not found under {}", id, name)),
            },
            None => Err(format!("❌ No product found with name {}", name)),
        }
    }
    

    // Edit a product's price and quantity by name and ID
    fn edit_product(&mut self, name: &str, id: u32, new_price: f64, new_quantity: u32) -> Result<(), String> {
        match self.products.get_mut(name) {
            Some(products) => match products.iter_mut().find(|p| p.id == id) {
                Some(product) => {
                    product.price = new_price;
                    product.quantity = new_quantity;
                    Ok(())
                }
                None => Err(format!("❌ Product with ID {} not found under {}", id, name)),
            },
            None => Err(format!("❌ No product found with name {}", name)),
        }
    }
    

    // Display all products in inventory
    fn display_products(&self) {
        println!("\n📦 Inventory:");
        for (name, products) in &self.products {
            for product in products {
                println!(
                    "🔹 Name: {}, ID: {}, Price: ₹{}, Description: {}, Quantity: {}",
                    name, product.id, product.price, product.description, product.quantity
                );
            }
        }
    }


    fn record_sale(&mut self, name: &str, id: u32, quantity: u32, sale_price: f64) -> Result<(), String> {
        match self.products.get_mut(name) {
            Some(products) => {
                match products.iter_mut().find(|p| p.id == id) {
                    Some(product) if product.quantity >= quantity => {
                        product.quantity -= quantity;
                        let total_price = (quantity as f64) * sale_price;

                        self.sales.push(Sale {
                            product_id: id,
                            product_name: name.to_string(),
                            quantity_sold: quantity,
                            sale_price,
                            total_price,
                        });

                        Ok(())
                    }
                    Some(_) => Err(format!("❌ Not enough stock for {} (ID: {}).", name, id)),
                    None => Err(format!("❌ Product ID {} not found in {}", id, name)),
                }
            }
            None => Err(format!("❌ Product {} not found", name)),
        }
    }

    fn display_sales(&self) {
        println!("\n🛒 Sales Transactions:");
        
        match self.sales.is_empty() {
            true => println!("❌ No sales recorded yet."),
            false => {
                for sale in &self.sales {
                    println!(
                        "📌 Product: {}, ID: {}, Quantity Sold: {}, Price per unit: ₹{}, Total: ₹{}",
                        sale.product_name, sale.product_id, sale.quantity_sold, sale.sale_price, sale.total_price
                    );
                }
            }
        }
    }

    // function for checing purchase record
    fn record_purchase(&mut self, name: &str, id: u32, quantity: u32, buy_price: f64) -> Result<(), String> {
        match self.products.get_mut(name) {
            Some(products) => {
                match products.iter_mut().find(|p| p.id == id) {
                    Some(product) => {
                        product.quantity += quantity;
                        let total_price = (quantity as f64) * buy_price;
    
                        self.purchases.push(Purchase {
                            product_id: id,
                            product_name: name.to_string(),
                            quantity_purchase: quantity,
                            buy_price,
                            total_price,
                        });
    
                        Ok(())
                    }
                    None => Err(format!("❌ Product ID {} not found in {}", id, name)),
                }
            }
            None => Err(format!("❌ Product {} not found", name)),
        }
    }
    

    fn display_purchase(&self) {
        println!("\n🛒 Purchase Transactions:");
        
        match self.purchases.is_empty() {
            true => println!("❌ No purchase recorded yet."),
            false => {
                for purchase in &self.purchases {
                    println!(
                        "📌 Product: {}, ID: {}, Quantity Purchased: {}, Price per unit: ₹{}, Total: ₹{}",
                        purchase.product_name, purchase.product_id, purchase.quantity_purchase, purchase.buy_price, purchase.total_price
                    );
                }
            }
        }
    }

}

// Main Function to Test
fn main() {
    let mut inventory = Inventory::new();

    // Adding products
    match inventory.add_product(Product {
        id: 1,
        name: "Laptop".to_string(),
        description: "Gaming Laptop".to_string(),
        price: 80000.0,
        quantity: 5,
    }) {
        Ok(_) => println!("✅ Laptop added successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    match inventory.add_product(Product {
        id: 2,
        name: "Laptop".to_string(),
        description: "Business Laptop".to_string(),
        price: 60000.0,
        quantity: 3,
    }) {
        Ok(_) => println!("✅ Business Laptop added successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    inventory.display_products();

    // Editing a product
    match inventory.edit_product("Laptop", 1, 75000.0, 6) {
        Ok(_) => println!("\n✏️ Updated Inventory after Editing:"),
        Err(err) => println!("❌ Error: {}", err),
    }
    inventory.display_products();

    // Deleting a product
    match inventory.delete_product("Laptop", 2) {
        Ok(_) => println!("\n🗑️ Inventory after Deleting a Product:"),
        Err(err) => println!("❌ Error: {}", err),
    }
    inventory.display_products();

    match inventory.record_sale("Laptop", 1, 2, 85000.0) {
        Ok(_) => println!("\n✅ Sale recorded successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    match inventory.record_purchase("Laptop", 1, 2, 85000.0) {
        Ok(_) => println!("\n✅ Purchase recorded successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    // Sell a Mouse
    match inventory.record_sale("Mouse", 2, 3, 1400.0) {
        Ok(_) => println!("\n✅ Sale recorded successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    match inventory.record_purchase("Mouse", 2, 3, 1400.0) {
        Ok(_) => println!("\n✅ Purchase recorded successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    // Attempt to sell more than available stock
    match inventory.record_sale("Laptop", 1, 10, 85000.0) {
        Ok(_) => println!("\n✅ Sale recorded successfully!"),
        Err(err) => println!("❌ Error: {}", err),
    }

    // Display updated inventory
    println!("\n📦 Updated Inventory:");
    inventory.display_products();

    // Show sales report
    inventory.display_sales();
    inventory.display_purchase();
}
