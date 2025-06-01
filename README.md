# Memorable IDs

[![Crates version](https://img.shields.io/crates/v/memorable-ids)](https://crates.io/crates/memorable-ids)
[![Rust version](https://img.shields.io/badge/rust-v1.79-blue.svg?logo=rust&label=MSRV)](https://www.rust-lang.org)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/memorable-ids)](https://crates.io/crates/memorable-ids)

A flexible Rust library for generating human-readable, memorable identifiers.
Uses combinations of adjectives, nouns, verbs, adverbs, and prepositions with optional numeric/custom suffixes.

## Features

- 🎯 **Human-readable** - Generate IDs like `cute-rabbit`, `quick-owl-dance-quietly`, etc
- 🔧 **Flexible** - 1-5 word components with customizable separators
- 📊 **Predictable** - Built-in collision analysis and capacity planning
- 🎲 **Extensible** - Custom suffix generators and vocabulary
- 📝 **TypeScript** - Full type safety and IntelliSense support
- ⚡ **Fast** - ~1M IDs per second generation speed
- 🪶 **Lightweight** - ~10KB vocabulary, zero dependencies

## Installation

```toml
[dependencies]
memorable-ids = "0.1.0"
```

## Quick Start

```rust
use memorable_ids::{generate, GenerateOptions, suffix_generators};

// Basic usage
let id = generate(GenerateOptions::default()).unwrap();
// Output: "cute-rabbit"

// More components for uniqueness
let id = generate(GenerateOptions {
    components: 3,
    ..Default::default()
}).unwrap();
// Output: "large-fox-swim"

// Add numeric suffix for extra capacity
let id = generate(GenerateOptions {
    components: 2,
    suffix: Some(suffix_generators::number),
    ..Default::default()
}).unwrap();
// Output: "quick-mouse-042"

// Custom separator
let id = generate(GenerateOptions {
    separator: "_".to_string(),
    ..Default::default()
}).unwrap();
// Output: "warm_duck"
```

## API Reference

### Core Functions

```rust
// Generate memorable ID
generate(options: GenerateOptions) -> Result<String, MemorableIdError>

// Parse ID back to components
parse(id: &str, separator: &str) -> Result<ParsedId, MemorableIdError>

// Calculate total possible combinations
calculate_combinations(components: usize, suffix_range: u64) -> u64

// Get collision analysis
get_collision_analysis(components: usize, suffix_range: u64) -> CollisionAnalysis
```

### Configuration Options

```rust
GenerateOptions {
    components: usize,        // 1-5 word components (default: 2)
    suffix: Option<SuffixGenerator>, // Optional suffix function
    separator: String,        // Separator between parts (default: "-")
}
```

### Built-in Suffix Generators

```rust
suffix_generators::number()    // "042" (3-digit, ×1,000 combinations)
suffix_generators::number4()   // "1337" (4-digit, ×10,000 combinations)
suffix_generators::hex()       // "a7" (2-digit hex, ×256 combinations)
suffix_generators::timestamp() // "8429" (time-based, ×10,000 combinations)
suffix_generators::letter()    // "k" (single letter, ×26 combinations)
```

## Capacity Planning

### Total Combinations by Components

| Components | Total IDs   | Example                          |
|------------|-------------|----------------------------------|
| 1          | 87          | `bright`                         |
| 2          | 5,916       | `cute-rabbit`                    |
| 3          | 236,640     | `large-fox-swim`                 |
| 4          | 6,389,280   | `happy-owl-dance-quietly`        |
| 5          | 166,121,280 | `clever-fox-run-quickly-through` |

### Recommended Configurations

| Use Case                  | Configuration          | Capacity | Example               |
|---------------------------|------------------------|----------|-----------------------|
| Small apps (<1K IDs)      | 2 components           | 5,916    | `cute-rabbit`         |
| Medium apps (1K-50K IDs)  | 3 components           | 236,640  | `large-fox-swim`      |
| Large apps (50K-500K IDs) | 2 components + suffix  | 5M+      | `cute-rabbit-042`     |
| Enterprise (500K+ IDs)    | 4+ components + suffix | 50M+     | `happy-owl-dance-042` |

### Collision Probability Examples

**2 components (5,916 total):**
- 100 IDs: 0.84% collision chance
- 500 IDs: 19.5% collision chance

**3 components (236,640 total):**
- 10,000 IDs: 0.211% collision chance
- 50,000 IDs: 5.2% collision chance

**2 components + 3-digit suffix (5,916,000 total):**
- 100,000 IDs: 0.084% collision chance
- 1,000,000 IDs: 8.4% collision chance

## Advanced Usage

### Custom Suffix Generators

```rust
// Custom timestamp suffix
fn timestamp_suffix() -> Option<String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    Some(format!("{:06}", timestamp % 1000000))
}

let id = generate(GenerateOptions {
    components: 2,
    suffix: Some(timestamp_suffix),
    ..Default::default()
}).unwrap();
```

### Parsing IDs

```rust
use memorable_ids::parse;

let parsed = parse("cute-rabbit-042", "-").unwrap();
// ParsedId {
//   components: vec!["cute", "rabbit"],
//   suffix: Some("042")
// }
```

### Error Handling

```rust
use memorable_ids::{generate, GenerateOptions, MemorableIdError};

match generate(GenerateOptions { components: 6, ..Default::default() }) {
    Ok(id) => println!("Generated: {}", id),
    Err(MemorableIdError::InvalidComponentCount(count)) => {
        eprintln!("Invalid component count: {}", count);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Dictionary Access

```rust
use memorable_ids::{get_dictionary_stats, get_dictionary};

let stats = get_dictionary_stats();
println!("Available words: {} adjectives, {} nouns",
         stats.adjectives, stats.nouns);

let dict = get_dictionary();
println!("First adjective: {}", dict.adjectives[0]);
```

## Examples

### Web Application

```rust
use memorable_ids::{generate, GenerateOptions, suffix_generators};

// User-friendly URLs
fn generate_slug() -> String {
    generate(GenerateOptions {
        components: 2,
        separator: "-".to_string(),
        ..Default::default()
    }).unwrap()
}

// Unique database IDs
fn generate_record_id() -> String {
    generate(GenerateOptions {
        components: 3,
        suffix: Some(suffix_generators::number4),
        separator: "-".to_string(),
    }).unwrap()
}

// Session identifiers
fn generate_session_id() -> String {
    generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::timestamp),
        separator: "_".to_string(),
    }).unwrap()
}
```

### Batch Generation

```rust
use std::collections::HashSet;

fn generate_unique_batch(count: usize) -> Vec<String> {
    let mut ids = HashSet::new();
    let options = GenerateOptions {
        components: 3,
        suffix: Some(suffix_generators::number4),
        ..Default::default()
    };

    while ids.len() < count {
        if let Ok(id) = generate(options.clone()) {
            ids.insert(id);
        }
    }

    ids.into_iter().collect()
}
```

## Security Notes

⚠️ **Important**: These IDs are **NOT cryptographically secure**. They use `rand::rng()` which is suitable for non-cryptographic purposes only.

- ✅ **Good for**: User-friendly identifiers, temporary IDs, non-sensitive references
- ❌ **NOT for**: Session tokens, passwords, security-critical identifiers

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

### Development

```bash
# Clone repository
git clone https://github.com/riipandi/memorable-ids-rs.git
cd memorable-ids-rs

# Run tests
cargo test

# Build the library
cargo build --release
```

## License

This project is open-sourced software licensed under the [MIT license](https://choosealicense.com/licenses/mit/).

Copyrights in this project are retained by their contributors.
See the [license file](./LICENSE) for more information.

---

<sub>🤫 Psst! If you like my work you can support me via [GitHub sponsors](https://github.com/sponsors/riipandi).</sub>
