# 🔍 Subdomain Brute Forcer

A high-performance, concurrent subdomain enumeration tool written in Rust. It uses asynchronous DNS resolution to quickly discover valid subdomains for a given domain.

## Features

- 🚀 **Concurrent DNS lookups** using Tokio async runtime
- 📝 **Custom wordlist support** – use any wordlist file
- 🎨 **Colored output** for easy reading (errors in red)
- ⚡ **Fast** – checks hundreds of subdomains per second
- 🔧 **Simple input** – interactive prompts for domain and wordlist

## Prerequisites

- Rust (1.70+)
- Cargo
- Working DNS configuration

## Installation

```bash
git clone https://github.com/0xMush/subdomain-brute.git
cd subdomain-brute
cargo build --release
cargo run
```

## Example

```bash
$ cargo run
Enter Domain eg:example.com
google.com
Enter Wordlist Path: test.txt
Loaded Content: 15
www.google.com -> 142.250.185.46
mail.google.com -> 142.250.185.46
admin.google.com -> 142.250.185.46
```
