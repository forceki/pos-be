# 🚀 High-Performance POS Backend (Rust)

> **"Built for Speed, Reliability, and Fast Delivery."**

A modern Point of Sale (POS) backend API built with **Rust** and **Actix Web**. This project leverages the safety and performance of Rust to ensure lightning-fast response times, low memory footprint, and robust type safety, making it ideal for high-throughput retail environments.

## ✨ Key Features

* **🦀 Blazing Fast:** Built on top of Actix Web (one of the fastest web frameworks in the world).
* **🛡️ Type-Safe Database:** Uses **SQLx** for compile-time checked SQL queries against MySQL.
* **🔐 Secure Authentication:** Industrial-grade security with **Argon2** hashing and **JWT** (JSON Web Tokens).
* **🏢 Multi-Tenancy:** Built-in support for multi-tenant architecture (Data isolation by `tenant_id`).
* **📦 Clean Architecture:** Modular design using the Controller-Service-Repository pattern for easy maintenance and scalability.
* **📄 Pagination & Metadata:** Standardized API responses with automatic pagination meta handling.
* **✅ Input Validation:** Strict DTO validation and graceful error handling.

## 🛠️ Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Framework:** [Actix Web 4](https://actix.rs/)
* **Database:** [MySQL](https://www.mysql.com/)
* **ORM/Query Builder:** [SQLx](https://github.com/launchbadge/sqlx)
* **Authentication:** `jsonwebtoken`, `argon2`
* **Serialization:** `serde`, `serde_json`
* **Utilities:** `chrono` (Time), `uuid` (IDs), `slug` (URL friendly)

## 📂 Project Structure

We follow a strict **Clean Architecture** to ensure the code remains readable as the project grows.

```text
src/
├── api/             # Route Configuration (Router)
├── controllers/     # HTTP Request Handlers (Input parsing, Response formatting)
├── services/        # Business Logic (Validation, Slug generation, Calculation)
├── repository/      # Database Access Layer (Raw SQL via SQLx)
├── models/          # Database Structs (Entities)
├── dtos/            # Data Transfer Objects (Request/Response structs)
├── middleware/      # Middleware (Auth Satpam, Logger)
├── utils/           # Helper functions (JWT, Pagination, Response Wrapper)
├── app_state.rs     # Dependency Injection Container
└── main.rs          # Entry point & Server config

