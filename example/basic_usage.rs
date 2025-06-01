//! Basic usage examples for memorable-ids
//!
//! This example demonstrates the fundamental features of the memorable-ids library
//! including basic ID generation, parsing, and different configuration options.

use memorable_ids::{
    generate, parse, suffix_generators, GenerateOptions, MemorableIdError,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Memorable IDs - Basic Usage Examples ===\n");

    // 1. Basic ID generation with default settings
    basic_generation()?;

    // 2. Different component counts
    component_variations()?;

    // 3. Using different suffix generators
    suffix_examples()?;

    // 4. Custom separators
    separator_examples()?;

    // 5. Parsing generated IDs
    parsing_examples()?;

    // 6. Error handling
    error_handling_examples();

    Ok(())
}

fn basic_generation() -> Result<(), MemorableIdError> {
    println!("1. Basic ID Generation:");
    println!("----------------------");

    // Default: 2 components, no suffix, "-" separator
    for i in 1..=5 {
        let id = generate(GenerateOptions::default())?;
        println!("  ID {}: {}", i, id);
    }
    println!();

    Ok(())
}

fn component_variations() -> Result<(), MemorableIdError> {
    println!("2. Different Component Counts:");
    println!("------------------------------");

    for components in 1..=5 {
        let id = generate(GenerateOptions {
            components,
            ..Default::default()
        })?;
        println!("  {} component(s): {}", components, id);
    }
    println!();

    Ok(())
}

fn suffix_examples() -> Result<(), MemorableIdError> {
    println!("3. Suffix Generator Examples:");
    println!("-----------------------------");

    // 3-digit number suffix
    let id = generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::number),
        ..Default::default()
    })?;
    println!("  With 3-digit number: {}", id);

    // 4-digit number suffix
    let id = generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::number4),
        ..Default::default()
    })?;
    println!("  With 4-digit number: {}", id);

    // Hex suffix
    let id = generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::hex),
        ..Default::default()
    })?;
    println!("  With hex suffix: {}", id);

    // Timestamp suffix
    let id = generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::timestamp),
        ..Default::default()
    })?;
    println!("  With timestamp: {}", id);

    // Letter suffix
    let id = generate(GenerateOptions {
        components: 2,
        suffix: Some(suffix_generators::letter),
        ..Default::default()
    })?;
    println!("  With letter suffix: {}", id);

    println!();
    Ok(())
}

fn separator_examples() -> Result<(), MemorableIdError> {
    println!("4. Custom Separator Examples:");
    println!("------------------------------");

    let separators = ["-", "_", ".", ":", "|"];

    for separator in separators {
        let id = generate(GenerateOptions {
            components: 3,
            separator: separator.to_string(),
            ..Default::default()
        })?;
        println!("  Separator '{}': {}", separator, id);
    }
    println!();

    Ok(())
}

fn parsing_examples() -> Result<(), MemorableIdError> {
    println!("5. Parsing Examples:");
    println!("--------------------");

    // Generate some IDs and parse them back
    let test_cases = [
        GenerateOptions {
            components: 2,
            suffix: None,
            separator: "-".to_string(),
        },
        GenerateOptions {
            components: 3,
            suffix: Some(suffix_generators::number),
            separator: "-".to_string(),
        },
        GenerateOptions {
            components: 2,
            suffix: Some(suffix_generators::hex),
            separator: "_".to_string(),
        },
    ];

    for (i, options) in test_cases.iter().enumerate() {
        let id = generate(options.clone())?;
        let parsed = parse(&id, &options.separator)?;

        println!("  Example {}:", i + 1);
        println!("    Generated ID: {}", id);
        println!("    Components: {:?}", parsed.components);
        println!("    Suffix: {:?}", parsed.suffix);
        println!();
    }

    Ok(())
}

fn error_handling_examples() {
    println!("6. Error Handling Examples:");
    println!("---------------------------");

    // Invalid component count
    match generate(GenerateOptions {
        components: 6, // Invalid: max is 5
        ..Default::default()
    }) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Invalid components error: {}", e),
    }

    // Empty separator
    match generate(GenerateOptions {
        components: 2,
        separator: "".to_string(), // Invalid: cannot be empty
        ..Default::default()
    }) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Empty separator error: {}", e),
    }

    // Invalid parsing
    match parse("", "-") {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Parse error: {}", e),
    }

    println!();
}
