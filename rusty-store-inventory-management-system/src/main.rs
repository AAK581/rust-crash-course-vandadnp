use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// Enum to differentiate transaction types
#[derive(Debug, Clone)]
enum TransactionType {
    Sale,
    Purchase,
}

// Struct for a product
#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    quantity: u32,
}

// Struct for a transaction (sale or purchase)
#[derive(Debug, Clone)]
struct Transaction {
    id: u32,
    product_id: u32,
    quantity: u32,
    transaction_type: TransactionType,
    timestamp: u64,
}

// Struct to manage the store
struct Store {
    inventory: HashMap<u32, Product>,
    transactions: HashMap<u32, Transaction>,
    next_product_id: u32,
    next_transaction_id: u32,
    password: String,
}

impl Store {
    fn new() -> Self {
        Store {
            inventory: HashMap::new(),
            transactions: HashMap::new(),
            next_product_id: 1,
            next_transaction_id: 1,
            password: "admin123".to_string(),
        }
    }

    // Authenticate user
    fn authenticate(&self, input_password: &str) -> bool {
        self.password == input_password
    }

    // Add a product
    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<u32, String> {
        if name.is_empty() || price < 0.0 {
            return Err("Invalid product name or price".to_string());
        }
        let id = self.next_product_id;
        self.inventory.insert(id, Product { id, name, price, quantity });
        self.next_product_id += 1;
        Ok(id)
    }

    // Edit a product
    fn edit_product(&mut self, id: u32, name: String, price: f64, quantity: u32) -> Result<(), String> {
        if name.is_empty() || price < 0.0 {
            return Err("Invalid product name or price".to_string());
        }
        let product = self.inventory.get_mut(&id).ok_or("Product not found")?;
        product.name = name;
        product.price = price;
        product.quantity = quantity;
        Ok(())
    }

    // Delete a product
    fn delete_product(&mut self, id: u32) -> Result<(), String> {
        self.inventory.remove(&id).ok_or("Product not found")?;
        Ok(())
    }

    // Record a transaction (sale or purchase)
    fn record_transaction(
        &mut self,
        product_id: u32,
        quantity: u32,
        transaction_type: TransactionType,
    ) -> Result<u32, String> {
        if quantity == 0 {
            return Err("Quantity must be greater than zero".to_string());
        }
        let product = self.inventory.get_mut(&product_id).ok_or("Product not found")?;
        
        match transaction_type {
            TransactionType::Sale => {
                if product.quantity < quantity {
                    return Err("Insufficient stock".to_string());
                }
                product.quantity -= quantity;
            }
            TransactionType::Purchase => {
                product.quantity += quantity;
            }
        }

        let id = self.next_transaction_id;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.transactions.insert(id, Transaction {
            id,
            product_id,
            quantity,
            transaction_type,
            timestamp,
        });
        self.next_transaction_id += 1;
        Ok(id)
    }

    // Inventory report
    fn inventory_report(&self) -> String {
        let mut report = String::from("Inventory Report\nID | Name | Price | Quantity\n");
        for product in self.inventory.values() {
            report.push_str(&format!("{} | {} | ${:.2} | {}\n", product.id, product.name, product.price, product.quantity));
        }
        report
    }

    // Transaction report
    fn transaction_report(&self) -> String {
        let mut total_sales = 0.0;
        let mut total_purchases = 0.0;
        let mut report = String::from("Transaction Report\nID | Type | Product | Quantity | Price | Total | Timestamp\n");
        for tx in self.transactions.values() {
            let product = self.inventory.get(&tx.product_id);
            let product_name = product.map(|p| p.name.as_str()).unwrap_or("Unknown");
            let price = product.map(|p| p.price).unwrap_or(0.0);
            let total = tx.quantity as f64 * price;
            match tx.transaction_type {
                TransactionType::Sale => {
                    total_sales += total;
                    report.push_str(&format!(
                        "{} | Sale | {} | {} | ${:.2} | ${:.2} | {}\n",
                        tx.id, product_name, tx.quantity, price, total, tx.timestamp
                    ));
                }
                TransactionType::Purchase => {
                    total_purchases += total;
                    report.push_str(&format!(
                        "{} | Purchase | {} | {} | ${:.2} | ${:.2} | {}\n",
                        tx.id, product_name, tx.quantity, price, total, tx.timestamp
                    ));
                }
            }
        }
        report.push_str(&format!("\nTotal Sales: ${:.2}\nTotal Purchases: ${:.2}\n", total_sales, total_purchases));
        report
    }
}

fn main() {
    let mut store = Store::new();
    println!("Rusty Store Inventory Management System");
    
    // Authentication
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    if !store.authenticate(password.trim()) {
        println!("Authentication failed!");
        return;
    }
    println!("Authentication successful!");

    loop {
        println!("\nMenu:\n1. Add Product\n2. Edit Product\n3. Delete Product\n4. Record Sale\n5. Record Purchase\n6. Inventory Report\n7. Transaction Report\n8. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("Price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price = price.trim().parse().unwrap_or(-1.0);

                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity = quantity.trim().parse().unwrap_or(0);

                match store.add_product(name, price, quantity) {
                    Ok(id) => println!("Product added with ID: {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                print!("Product ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse().unwrap_or(0);

                print!("New name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("New price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price = price.trim().parse().unwrap_or(-1.0);

                print!("New quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity = quantity.trim().parse().unwrap_or(0);

                match store.edit_product(id, name, price, quantity) {
                    Ok(()) => println!("Product updated"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                print!("Product ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse().unwrap_or(0);

                match store.delete_product(id) {
                    Ok(()) => println!("Product deleted"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                print!("Product ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse().unwrap_or(0);

                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity = quantity.trim().parse().unwrap_or(0);

                if let Some(product) = store.inventory.get(&id) {
                    println!("Using product price: ${:.2}", product.price);
                }

                match store.record_transaction(id, quantity, TransactionType::Sale) {
                    Ok(id) => println!("Sale recorded with ID: {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "5" => {
                print!("Product ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse().unwrap_or(0);

                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity = quantity.trim().parse().unwrap_or(0);

                if let Some(product) = store.inventory.get(&id) {
                    println!("Using product price: ${:.2}", product.price);
                }

                match store.record_transaction(id, quantity, TransactionType::Purchase) {
                    Ok(id) => println!("Purchase recorded with ID: {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "6" => println!("\n{}", store.inventory_report()),
            "7" => println!("\n{}", store.transaction_report()),
            "8" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}