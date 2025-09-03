# Getting Started with Rust: A Beginner's Toolkit

## 1. Title & Objective

**Technology Chosen:** Rust Programming Language  
**Why I chose it:** Rust is a modern systems programming language that emphasizes memory safety and performance. It's increasingly popular in web development, blockchain, and system programming.  
**End Goal:** Create a simple command-line program that demonstrates Rust's ownership system and basic syntax.

## 2. Quick Summary of the Technology

**What is Rust?**
Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It was originally developed by Mozilla and has gained significant adoption across the tech industry.

**Where is it used?**

- Web backends (using frameworks like Actix-web, Rocket)
- Operating systems and embedded systems
- Blockchain development
- Game engines
- Command-line tools

**Real-world example:** Discord uses Rust for their backend services, Dropbox uses it for file storage, and Firefox's rendering engine includes Rust components.

## 3. System Requirements

**OS:** Linux, macOS, or Windows  
**Tools Required:**

- Terminal/Command Prompt
- Text editor (VS Code recommended with rust-analyzer extension)
- Internet connection for package downloads

**Package Manager:** Cargo (comes with Rust installation)

## 4. Installation & Setup Instructions

### Step 1: Install Rust

Visit [rustup.rs](https://rustup.rs/) and run the installation command:

**On Linux/macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Windows:**
Download and run `rustup-init.exe` from the website.

### Step 2: Verify Installation

```bash
rustc --version
cargo --version
```

Expected output:

```
rustc 1.72.0 (5680fa18f 2023-08-23)
cargo 1.72.0 (103a7ff2e 2023-08-15)
```

### Step 3: Create a New Project

```bash
cargo new rust_beginner_project
cd rust_beginner_project
```

## 5. Minimal Working Example

### Project Description

I created a simple "Personal Budget Tracker" that demonstrates:

- Variables and data types
- Functions
- Structs
- Basic input/output
- Rust's ownership system

### Code (`src/main.rs`):

```rust
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
```

### Expected Output:

```
 Welcome to Personal Budget Tracker!
=====================================

 Your Budget Items:
1:  Coffee: $4.50 (Food)
2:  Netflix Subscription: $15.99 (Entertainment)
3:  Bus Fare: $2.75 (Transport)

 Total Budget: $23.24

 Add a new budget item:
Enter item name:
Lunch
Enter amount:
12.50
Enter category:
Food

 Added new item:
 Lunch: $12.50 (Food)

 Updated Total Budget: $35.74

Thank you for using Budget Tracker!
```

### How to Run:

```bash
cargo run
```

## 6. AI Prompt Journal

### Prompt 1:

**Used:** "Explain Rust programming language basics for a complete beginner, including what makes it unique"

**AI Response Summary:** The AI explained Rust's memory safety features, ownership system, and use cases. It helped me understand why Rust is different from other languages.

**Evaluation:** Very helpful for understanding the "why" behind Rust.

### Prompt 2:

**Used:** "Create a simple Rust program that demonstrates structs, functions, and basic I/O for beginners"

**AI Response Summary:** Provided a foundation for the budget tracker concept and showed how to structure a Rust program with user input.

**Evaluation:** Good starting point, but needed refinement for educational purposes.

### Prompt 3:

**Used:** "How do I handle user input and error handling in Rust for a beginner-friendly way"

**AI Response Summary:** Showed `.expect()` for basic error handling and `std::io` for input, which is perfect for beginners.

**Evaluation:** Excellent for keeping the example simple while being robust.

### Prompt 4:

**Used:** "What are common beginner mistakes when learning Rust and how to avoid them"

**AI Response Summary:** Highlighted ownership issues, borrowing confusion, and compilation errors that beginners typically face.

**Evaluation:** Invaluable for the troubleshooting section.

## 7. Common Issues & Fixes

### Issue 1: "borrow of moved value" Error

**Problem:** Trying to use a variable after it's been moved.

```rust
let name = String::from("test");
let item = BudgetItem::new(name, 10.0, "food".to_string());
println!("{}", name); // Error: name was moved
```

**Fix:** Use `.clone()` or references:

```rust
let name = String::from("test");
let item = BudgetItem::new(name.clone(), 10.0, "food".to_string());
println!("{}", name); // Now works
```

### Issue 2: Input Contains Newline Characters

**Problem:** `read_line()` includes the newline character.

**Fix:** Always use `.trim()`:

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");
let input = input.trim(); // Remove whitespace and newlines
```

### Issue 3: Parse Errors for Numbers

**Problem:** User enters invalid number format.

**Fix:** Use better error handling:

```rust
let amount: f64 = match amount_str.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid number, using 0.0");
        0.0
    }
};
```

### Issue 4: Cargo Build Fails

**Problem:** Missing dependencies or wrong Rust version.

**Fix:** Update Rust and clear cache:

```bash
rustup update
cargo clean
cargo build
```

## 8. References

### Official Documentation

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - The official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by running examples
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Understanding Rust's build system

### Video Resources

- [Rust Crash Course - Traversy Media](https://www.youtube.com/watch?v=zF34dRivLOw)
- [Rust Programming Course for Beginners - freeCodeCamp](https://www.youtube.com/watch?v=MsocPEZBd-M)

### Helpful Blog Posts

- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

### Community Resources

- [r/rust subreddit](https://www.reddit.com/r/rust/) - Active community
- [Rust Users Forum](https://users.rust-lang.org/) - Questions and discussions
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter

---

## Learning Reflection

Using AI prompts accelerated my learning significantly. Instead of getting lost in documentation, I could ask specific questions and get targeted explanations. The AI helped me:

1. **Understand concepts faster** - Complex topics like ownership were explained in beginner terms
2. **Generate practical examples** - Rather than toy programs, I got realistic use cases
3. **Anticipate problems** - The AI warned me about common pitfalls
4. **Structure learning** - It suggested a logical progression from basics to more complex features
