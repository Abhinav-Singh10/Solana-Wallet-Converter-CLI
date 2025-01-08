# Solana Wallet Converter CLI

A simple Command-Line Interface (CLI) program written in Rust for converting between Solana wallet formats:
- Base58-encoded secret keys
- Wallet format (u8 array)

This program helps you easily decode Base58-encoded secret keys into a raw `u8` array and encode a raw wallet format back into Base58 strings.

## Features
- Convert **Base58 Secret Key** to **Wallet Format (u8 array)**.
- Convert **Wallet Format (u8 array)** to **Base58 Secret Key**.
- Interactive and user-friendly CLI interface.

---

## Getting Started

### Prerequisites
Make sure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Also, ensure you have the `bs58` crate added to your project. If not, you can add it by including it in your `Cargo.toml` file:

```toml
[dependencies]
bs58 = "0.4"
```

### Installation
Clone the repository and navigate into the project directory:

```bash
git clone <your-repo-url>
cd <project-directory>
```

### Running the Program
Run the program using the following command:

```bash
cargo run
```

---

## Usage
Upon running, the CLI will guide you through the following options:

1. Convert Base58 Secret Key to Wallet Format (u8 array).
2. Convert Wallet Format (u8 array) to Base58 Secret Key.
3. Exit the program.

### Example Workflow

#### Option 1: Convert Base58 Secret Key to Wallet Format (u8 array)
Input a Base58-encoded secret key (e.g., `gdtKSTXYULQNx87...`). The program will output the corresponding wallet format as a `u8` array.

**Input:**
```
Enter the Base58 string: gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt
```

**Output:**
```
Wallet in u8 array: [34, 46, 55, 124, 141, 190, 24, 204, ...]
```

#### Option 2: Convert Wallet Format (u8 array) to Base58 Secret Key
Input a wallet format (comma-separated `u8` array). The program will output the corresponding Base58 string.

**Input:**
```
Enter the Wallet (comma-separated u8 array): 34,46,55,124,141,190,24,204,...
```

**Output:**
```
Base58 string: gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt
```

#### Option 3: Exit
Select this option to exit the program.

---

## Additional Information
- **u8 arrays**: Solana wallet files commonly store the wallet's secret key as a `u8` array. This format is widely used in the backend for interacting with Solana blockchain programs.
- **Base58 style secret keys**: Wallet applications like Phantom and Solflare often use Base58-encoded strings to store and share secret keys in a more compact and human-readable format.

---

## Acknowledgments
- [bs58 crate documentation](https://docs.rs/bs58)
- Solana blockchain and wallet encoding documentation

---

Enjoy converting your Solana wallet keys! 😃

