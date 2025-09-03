use std::io;

// Define a struct to represent a budget item
#[derive(Debug)]
struct BudgetItem {
    name: String,
    amount: f64,
    category: String,
}

impl BudgetItem {
    // Constructor function
    fn new(name: String, amount: f64, category: String) -> BudgetItem {
        BudgetItem {
            name,
            amount,
            category,
        }
    }

    // Method to display the item
    fn display(&self) {
        println!(" {}: ${:.2} ({})", self.name, self.amount, self.category);
    }
}

fn main() {
    println!(" Welcome to Personal Budget Tracker!");
    println!("=====================================");
    
    // Create a vector to store budget items
    let mut budget_items: Vec<BudgetItem> = Vec::new();
    
    // Add some sample items
    budget_items.push(BudgetItem::new(
        "Coffee".to_string(),
        4.50,
        "Food".to_string()
    ));
    
    budget_items.push(BudgetItem::new(
        "Netflix Subscription".to_string(),
        15.99,
        "Entertainment".to_string()
    ));
    
    budget_items.push(BudgetItem::new(
        "Bus Fare".to_string(),
        2.75,
        "Transport".to_string()
    ));
    
    // Display all items
    println!("\n Your Budget Items:");
    for (index, item) in budget_items.iter().enumerate() {
        print!("{}: ", index + 1);
        item.display();
    }
    
    // Calculate total
    let total: f64 = budget_items.iter().map(|item| item.amount).sum();
    println!("\n Total Budget: ${:.2}", total);
    
    // Interactive feature: Add a new item
    println!("\n Add a new budget item:");
    println!("Enter item name:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string();
    
    println!("Enter amount:");
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).expect("Failed to read input");
    let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");
    
    println!("Enter category:");
    let mut category = String::new();
    io::stdin().read_line(&mut category).expect("Failed to read input");
    let category = category.trim().to_string();
    
    // Add the new item
    let new_item = BudgetItem::new(name, amount, category);
    println!("\n Added new item:");
    new_item.display();
    
    budget_items.push(new_item);
    
    // Recalculate total
    let new_total: f64 = budget_items.iter().map(|item| item.amount).sum();
    println!("\n Updated Total Budget: ${:.2}", new_total);
    
    println!("\n Thank you for using Budget Tracker!");
}