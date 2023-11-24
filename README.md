# Rust API Redesign Project

## Introduction

Welcome to my Rust API Redesign Project! This repository documents my journey in learning Rust and applying it to redesign an existing API, originally written in Typescript with NestJS, for enhanced performance and efficiency. A unique aspect of this project is the integration of an API and a Discord bot that simultaneously interact with the database, showcasing Rust's ability to handle concurrent tasks effectively.

## Project Overview

In this project, I focus on leveraging Rust's low-level control and high-level capabilities to rebuild my API, originally crafted in Typescript. The primary goal is to enhance performance and reliability. A noteworthy feature of this redesign is the concurrent operation of the API and a Discord bot, both utilizing the same database resources. This demonstrates Rust's exceptional ability to manage multiple tasks and shared resources without compromising on performance.

### Learning Objectives

- Gain a deep understanding of Rust and its features.
- Explore popular Rust crates like `tokio` for asynchronous runtime, `serde` for serialization and deserialization, and `seaORM` for database interactions.
- Navigate and utilize documentation from crates.io and various crate documentation websites.
- Implement best practices for Rust development to ensure code efficiency and safety.

## Highlights and Progress

Throughout this project, I have achieved several key milestones:

- **Concurrent API and Discord Bot Operation**: One of the standout achievements of this project so far is running the API and a Discord bot concurrently, both accessing the same database. This setup exemplifies Rust's prowess in handling parallel tasks and shared resources efficiently, ensuring seamless operation and data consistency.

- **Generalization of Database Queries**: The use of `seaORM` has enabled me to prototype a generalized approach for SQL database interactions, minimizing redundancy and enhancing code maintainability.

- **Streamlined Routing**: The application's routing logic has been refined for efficiency, showcasing Rust's powerful type system and library support.

## Future Plans

Once the API rewrite is complete and stable, the next phase of this project will involve rebuilding my website using the `yew` crate, a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly.

## Contributing

While this project is primarily for my learning and exploration, suggestions, and discussions are welcome! If you have insights or suggestions, feel free to open an issue or submit a pull request.

`cargo watch -w src -w Cargo.toml -x check -x test -x run`

---

Thank you for visiting my project! Stay tuned for more updates as I continue my journey with Rust.
