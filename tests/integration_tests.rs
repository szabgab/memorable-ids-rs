//! Integration tests for memorable-ids library
//!
//! End-to-end tests that verify the complete workflow of ID generation,
//! parsing, and analysis functionality working together.

use memorable_ids::*;
use std::collections::HashSet;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_workflow_with_suffix() {
        // Generate ID with suffix
        let options = GenerateOptions {
            components: 2,
            suffix: Some(suffix_generators::number),
            separator: "_".to_string(),
        };
        let id = generate(options).unwrap();

        // Parse the generated ID
        let parsed = parse(&id, "_").unwrap();

        // Verify structure
        assert_eq!(parsed.components.len(), 2);
        assert!(parsed.suffix.is_some());

        let suffix = parsed.suffix.unwrap();
        assert_eq!(suffix.len(), 3);
        assert!(suffix.chars().all(|c| c.is_ascii_digit()));

        // Verify we can reconstruct the ID
        let mut parts = parsed.components;
        parts.push(suffix);
        let reconstructed = parts.join("_");
        assert_eq!(reconstructed, id);
    }

    #[test]
    fn test_multiple_id_generation_uniqueness() {
        let options = GenerateOptions {
            components: 3,
            suffix: Some(suffix_generators::number4),
            ..Default::default()
        };

        let mut generated_ids = HashSet::new();
        let num_ids = 1000;

        // Generate multiple IDs
        for _ in 0..num_ids {
            let id = generate(options.clone()).unwrap();
            generated_ids.insert(id);
        }

        // With 3 components + 4-digit suffix, we should have very few collisions
        let unique_count = generated_ids.len();
        let collision_rate = (num_ids - unique_count) as f64 / num_ids as f64;

        // Should have very low collision rate (< 1%)
        assert!(
            collision_rate < 0.01,
            "Collision rate too high: {:.2}% ({} unique out of {})",
            collision_rate * 100.0,
            unique_count,
            num_ids
        );
    }

    #[test]
    fn test_collision_analysis_accuracy() {
        let components = 2;
        let suffix_range = 100; // Small range to test collisions

        let analysis = get_collision_analysis(components, suffix_range);
        let total_combinations = analysis.total_combinations;

        // Verify total combinations calculation
        let expected_total = calculate_combinations(components, suffix_range);
        assert_eq!(total_combinations, expected_total);

        // Test a specific scenario
        let test_ids = 50;
        let predicted_prob =
            calculate_collision_probability(total_combinations, test_ids);

        // Find the scenario in analysis
        let scenario = analysis.scenarios.iter().find(|s| s.ids == test_ids);

        if let Some(scenario) = scenario {
            assert!(
                (scenario.probability - predicted_prob).abs() < 0.001,
                "Predicted probability should match analysis"
            );
        }
    }

    #[test]
    fn test_letter_suffix_generator_separately() {
        // Test letter suffix generator separately since it's not numeric
        let options = GenerateOptions {
            components: 2,
            suffix: Some(suffix_generators::letter),
            ..Default::default()
        };

        // Generate ID
        let id = generate(options).unwrap();

        // Split manually since parse() only detects numeric suffixes
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(
            parts.len(),
            3,
            "Should have 2 components + 1 letter suffix"
        );

        // Last part should be a single lowercase letter
        let suffix = parts.last().unwrap();
        assert_eq!(suffix.len(), 1);
        assert!(suffix.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_edge_case_parsing() {
        // Test various edge cases in parsing
        let test_cases = [
            ("single", vec!["single"], None),
            ("word-123", vec!["word"], Some("123")),
            ("a-b-c-d-e-999", vec!["a", "b", "c", "d", "e"], Some("999")),
            ("hyphen-word-test", vec!["hyphen", "word", "test"], None),
            ("mixed-123-word", vec!["mixed", "123", "word"], None), // 123 in middle, not suffix
        ];

        for (input, expected_components, expected_suffix) in test_cases {
            let parsed = parse(input, "-").unwrap();

            assert_eq!(
                parsed.components, expected_components,
                "Components mismatch for input: {}",
                input
            );
            assert_eq!(
                parsed.suffix,
                expected_suffix.map(|s| s.to_string()),
                "Suffix mismatch for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_cross_separator_compatibility() {
        let separators = ["-", "_", ".", "|", ":"];

        for separator in separators {
            let options = GenerateOptions {
                components: 3,
                suffix: Some(suffix_generators::number),
                separator: separator.to_string(),
            };

            // Generate ID with custom separator
            let id = generate(options).unwrap();

            // Verify separator is used
            assert!(
                id.contains(separator),
                "ID should contain separator: {}",
                separator
            );

            // Parse with same separator
            let parsed = parse(&id, separator).unwrap();

            // Verify correct parsing
            assert_eq!(parsed.components.len(), 3);
            assert!(parsed.suffix.is_some());

            // Verify reconstruction
            let mut parts = parsed.components;
            parts.push(parsed.suffix.unwrap());
            let reconstructed = parts.join(separator);
            assert_eq!(reconstructed, id);
        }
    }

    #[test]
    fn test_performance_characteristics() {
        use std::time::Instant;

        let options = GenerateOptions {
            components: 3,
            suffix: Some(suffix_generators::number),
            ..Default::default()
        };

        // Test generation performance
        let start = Instant::now();
        let mut ids = Vec::new();

        for _ in 0..1000 {
            let id = generate(options.clone()).unwrap();
            ids.push(id);
        }

        let generation_time = start.elapsed();

        // Should be able to generate 1000 IDs quickly (< 100ms)
        assert!(
            generation_time.as_millis() < 100,
            "Generation should be fast, took: {:?}",
            generation_time
        );

        // Test parsing performance
        let start = Instant::now();

        for id in &ids {
            let _parsed = parse(id, "-").unwrap();
        }

        let parsing_time = start.elapsed();

        // Should be able to parse 1000 IDs quickly (< 50ms)
        assert!(
            parsing_time.as_millis() < 50,
            "Parsing should be fast, took: {:?}",
            parsing_time
        );
    }

    #[test]
    fn test_library_exports() {
        // Test that all expected functions and types are exported
        let _options = GenerateOptions::default();
        let _id = generate(_options).unwrap();
        let _parsed = parse(&_id, "-").unwrap();
        let _combinations = calculate_combinations(2, 1000);
        let _probability = calculate_collision_probability(1000, 50);
        let _analysis = get_collision_analysis(2, 1000);
        let _suffix = default_suffix().unwrap();

        // Test suffix generators module
        let _num_suffix = suffix_generators::number().unwrap();
        let _hex_suffix = suffix_generators::hex().unwrap();
        let _letter_suffix = suffix_generators::letter().unwrap();

        // Test dictionary exports
        let _dict = get_dictionary();
        let _stats = get_dictionary_stats();
    }
}
