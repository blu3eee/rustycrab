# Full-Stack Rust Project

## Introduction

Welcome to my Full-Stack Rust Project! This repository is my journey into learning Rust by building a comprehensive full-stack application. The project is organized into several components: the `rustycrab-api` for backend API and Discord bot, `rustycrab-webapp` for the frontend web application, and `rustycrab-model` for shared data models used by both the API and the web application.

## Project Overview

The project is divided into distinct sections, each demonstrating different aspects of Rust's capabilities:

- **`rustycrab-api`**: backend, consisting of a Rust-based API with `axum` and a Discord bot using `twilight`
- **`rustycrab-webapp`**: frontend with `Yew` and `WebAssembly`
- **`rustycrab-model`**: shared data models and structures used by both the API and the web application

### Learning Objectives

- Gain a deep understanding of Rust and its features.
- Explore popular Rust crates like `tokio` for asynchronous runtime, `serde` for serialization and deserialization, and `seaORM` for database interactions.
- Navigate and utilize documentation from crates.io and various crate documentation websites.
- Implement best practices for Rust development to ensure code efficiency and safety.

## Highlights and Progress

- **Concurrent API and Discord Bot Operation**: One of the standout achievements of this project so far is running the API and a Discord bot concurrently, both accessing the same database. This setup exemplifies Rust's prowess in handling parallel tasks and shared resources efficiently, ensuring seamless operation and data consistency.

- **Generalization of Database Queries**: The use of `seaORM` has enabled me to prototype a generalized approach for SQL database interactions, minimizing redundancy and enhancing code maintainability.

## Future Plans

Once the API rewrite is complete and stable, the next phase of this project will involve rebuilding my website using the `yew` crate, a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly.

## Contributing

While this project is primarily for my learning and exploration, suggestions, and discussions are welcome! If you have insights or suggestions, feel free to open an issue or submit a pull request.

---

Thank you for visiting my project! Stay tuned for more updates as I continue my journey with Rust.
