# 🚀 Request Header Inspector (WhoAmI API)
> A high-performance API to retrieve client IP, browser, OS, and language from request headers, built with Rust and Axum.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)

## 🌟 Features
✔ **Extracts client IP address (supports `x-forwarded-for`)**  
✔ **Parses browser and operating system from User-Agent**  
✔ **Detects preferred language from Accept-Language**  
✔ **Counts total API requests**  
✔ **Fast and lightweight Rust-based implementation**  
✔ **Built using Axum for high-performance async handling**  

---

## 🛠 Tech Stack
| Technology | Purpose |
|------------|---------|
| **Rust** 🦀 | Systems programming language |
| **Axum** ⚡ | Web framework |
| **Hyper** 🔗 | HTTP handling |
| **Serde** 📦 | Serialization/Deserialization |

---

## 🚀 Getting Started

### Prerequisites
First, install Rust using [Rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For more details, visit the official Rust installation page:  
[🔗 Rust Installation Guide](https://www.rust-lang.org/tools/install)

---

### Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/whoami-api.git
cd whoami-api
```

2. Build the project:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

4. Start the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`.

---

## 📡 API Endpoints

### 1. Get Client Info (`GET /api/whoami`)
Returns the client’s **IP address, language, browser, OS, and total requests handled**.

#### **Example Request**
```bash
curl http://localhost:3000/api/whoami
```

#### **Example Response**
```json
{
  "total_requests": 15,
  "ipaddress": "192.168.1.1",
  "language": "en-US,en;q=0.9",
  "software": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
  "os": "Windows",
  "browser_name": "Chrome",
  "browser_version": "91.0.4472.124",
  "parsed_language": "en-US"
}
```

---

## 🔍 Error Handling

The API returns structured error responses:

```json
{
  "type": "InternalError",
  "message": "Failed to process request"
}
```

Error types:
- `InternalError`: Unexpected server-side failure.

---

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Run a specific test:
```bash
cargo test test_ip_extraction
```

Run tests with output:
```bash
cargo test -- --nocapture
```

---

## 📂 Project Structure
```
/whoami-api
├── src/
│   ├── main.rs           # Application entry point
│   ├── handlers/         # Request handlers
│       ├── whoami.rs     # WhoAmI request handler
│   ├── models/           # Data structures
│       ├── whoami.rs     # API response struct
│   ├── utils/            # Utility functions
│       ├── parser.rs     # Parse OS, browser, and language
├── tests/                # Integration tests
├── Cargo.toml           # Dependencies
└── README.md            # Documentation
```

---

## 🛠 Development Commands

### Cargo Commands
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation errors
cargo check

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### Development Tips
- Use `cargo watch` for auto-reloading:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```
- Format code:
  ```bash
  cargo fmt
  ```
- Check code style:
  ```bash
  cargo clippy
  ```

---

## 🤝 Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

---

## 📝 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📖 Full Tutorial & Blog Post
📌 **Read the step-by-step guide on my blog:**  
**👉 [Building a WhoAmI Microservice with Rust and Axum](https://dev.to/yourusername/building-a-whoami-microservice-with-rust-and-axum)**

The tutorial covers:
- Setting up a Rust project with Axum
- Extracting IP, language, and OS from headers
- Handling errors and testing the API
- Optimizing for performance
- Writing API documentation

---

## 🔗 Contact
- Blog: [dev.to/danielphilipjohnson](https://dev.to/danielphilipjohnson)
---
