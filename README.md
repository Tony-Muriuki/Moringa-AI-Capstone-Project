# Moringa-AI-Capstone-Project || Rust Beginner Project: Personal Budget Tracker

A simple command-line budget tracker built with Rust to demonstrate basic language concepts.

## What This Project Teaches

- Rust structs and implementations
- Ownership and borrowing concepts
- Vector manipulation
- User input handling
- Basic error handling
- String manipulation

## Quick Start

### Prerequisites

- Rust installed (visit [rustup.rs](https://rustup.rs/))

### Installation & Running

1. **Clone or download this project**

   ```bash
   # If using git
   git clone <your-repo-url>
   cd rust_beginner_project

   # Or create manually
   mkdir rust_beginner_project
   cd rust_beginner_project
   ```

2. **Copy the files**

   - Copy `Cargo.toml` to the root directory
   - Create `src/` directory
   - Copy `main.rs` to `src/main.rs`

3. **Run the project**
   ```bash
   cargo run
   ```

### Expected Workflow

1. The program shows pre-loaded budget items
2. It calculates and displays the total
3. You can add a new budget item interactively
4. The program recalculates the total with your new item

### Sample Output

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
```

## Project Structure

```
rust_beginner_project/
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    └── main.rs         # Main application code
```

## Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

## Troubleshooting

**Error: "borrow of moved value"**

- This is Rust's ownership system in action
- Use `.clone()` or references as needed

**Error: "failed to parse"**

- Make sure to enter valid numbers when prompted
- The program expects decimal format (e.g., 12.50)

**Cargo not found**

- Restart your terminal after installing Rust
- Run `source ~/.cargo/env` on Linux/Mac
