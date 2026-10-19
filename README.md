# Rust Task Manager

A lightweight, robust command-line task manager built in Rust to practice data structures, ownership, and automated unit testing.

## Features

- **Automated ID Generation:** Uses thread-safe static counters (`LazyLock` & `Mutex`) to assign unique IDs to tasks.
- **Task Management:** Add new tasks, change their status (`Pending`, `InProgress`, `Done`), and delete them by ID.
- **Comprehensive Testing:** Includes built-in unit tests verifying core logic and state changes.

## Code Structure

- **`Task` Struct:** Holds an `id`, `description`, and a `Status` enum.
- **`Tasks` Container:** Manages a vector of tasks (`Vec<Task>`) with methods for manipulation and querying.

## Getting Started

### Prerequisites
Make sure to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### Running Tests
To run the built-in test suite and verify everything works:
```bash
cargo test
