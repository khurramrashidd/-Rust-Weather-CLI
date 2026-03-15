# 🌦 Rust Weather CLI

A simple **Command Line Weather Application written in Rust** that fetches real-time weather data from an online API and displays it in the terminal.

This project demonstrates how to use **HTTP requests, JSON parsing, and external crates** in Rust.

---

## 🚀 Features

* Fetch real-time weather data from an API
* Display temperature and wind speed
* Simple command line interface
* Lightweight and fast
* Beginner-friendly Rust networking project

---

## 🛠 Built With

* **Rust**
* **reqwest** (HTTP requests)
* **serde** (JSON deserialization)
* **serde_json**

---

## 📂 Project Structure

```text
weather_cli/
│
├── Cargo.toml
├── Cargo.lock
└── src
    └── main.rs
```

---

## ⚙️ Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/rust-weather-cli.git
```

### 2. Navigate to the project folder

```bash
cd rust-weather-cli
```

### 3. Build the project

```bash
cargo build
```

---

## ▶️ Usage

Run the program:

```bash
cargo run -- mumbai
```

Example output:

```text
City: mumbai
Temperature: 30.4°C
Wind Speed: 11 km/h
```

## 📦 Dependencies

Add the following to `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🧠 Concepts Practiced

This project demonstrates several important Rust concepts:

* Command line arguments (`std::env`)
* HTTP requests
* JSON parsing
* Struct deserialization
* External crate usage
* Building CLI tools in Rust

---

## 🔮 Future Improvements

Possible improvements to the project:

* Fetch weather for real city coordinates
* Add humidity and weather condition
* Colorful terminal output
* Support multiple cities
* Display weather forecast

---

## 📜 License

This project is open source and available under the **MIT License**.

---

## 👨‍💻 Author

**Khurram Rashid**
B.Tech Computer Science Engineering
Amity University
