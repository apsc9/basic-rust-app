# Basic Rust Application

A simple console application written in Rust to demonstrate fundamental language features, including structs, methods, error handling (`Option` and `Result`), and basic state management.

## Description

This project was created as a learning exercise to explore core Rust concepts. It consists of two main parts:

1.  A **Calculator** that can perform basic arithmetic operations and maintain a history of these operations, which can be re-executed.
2.  A **Shapes** module that defines geometric shapes (`Rectangle`, `Circle`) with methods to manipulate their properties, including validation logic.

The main application runs a pre-defined sequence of operations and method calls to demonstrate the functionality of both the calculator and the shapes, printing the results to the console.

## Features

*   **Calculator**:
    *   Basic arithmetic operations (e.g., addition) with overflow protection.
    *   Keeps a history of all successful operations.
    *   Ability to "redo" an operation from history by its index.
    *   Uses Rust's `Option<T>` type to handle calculations that may fail (like overflow).
*   **Shapes**:
    *   `Rectangle` and `Circle` struct definitions.
    *   Getters and setters for managing shape properties.
    *   Input validation in setters to prevent invalid dimensions (e.g., a negative radius).
    *   Uses Rust's `Result<T, E>` type to handle validation errors.

## Getting Started

### Prerequisites

Before you begin, ensure you have the Rust toolchain installed on your system. You can install it by following the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building the Project

1.  Clone the repository or navigate to the project directory.
2.  Go into the `basic_app` crate directory.
3.  Use Cargo to build the application:
    ```bash
    cargo build
    ```
    For an optimized release build, use the `--release` flag:
    ```bash
    cargo build --release
    ```

### Running the Application

To execute the program and see the demonstration output, run the following command from the `basic_app` directory:

```bash
cargo run
```

### Running Tests

The project includes a suite of unit tests to verify its functionality. To run them, use the `test` command:

```bash
cargo test
```

## Project Structure

*   `src/main.rs`: The main entry point of the application. It demonstrates the usage of the `Calculator` and `Shape` modules.
*   `src/calculator.rs`: Contains the logic for the `Calculator`, including the `Operation` and `OperationType` definitions.
*   `src/shapes.rs`: Contains the definitions and implementations for the `Rectangle` and `Circle` structs and their associated errors.
