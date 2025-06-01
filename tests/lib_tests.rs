//! Tests for main library functionality
//!
//! Comprehensive tests for ID generation, parsing, collision analysis,
//! and suffix generators functionality.

use memorable_ids::*;

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_generate_default_options() {
        let options = GenerateOptions::default();
        let id = generate(options).unwrap();

        // Should have 2 parts separated by "-"
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 2, "Default should generate 2 components");

        // Each part should not be empty
        for part in parts {
            assert!(!part.is_empty(), "Component should not be empty");
        }
    }

    #[test]
    fn test_generate_with_different_components() {
        for components in 1..=5 {
            let options = GenerateOptions {
                components,
                ..Default::default()
            };
            let id = generate(options).unwrap();

            let parts: Vec<&str> = id.split('-').collect();
            assert_eq!(
                parts.len(),
                components,
                "Should generate {} components",
                components
            );
        }
    }

    #[test]
    fn test_generate_invalid_components() {
        let options = GenerateOptions {
            components: 0,
            ..Default::default()
        };
        assert!(generate(options).is_err(), "Should fail with 0 components");

        let options = GenerateOptions {
            components: 6,
            ..Default::default()
        };
        assert!(generate(options).is_err(), "Should fail with 6 components");
    }

    #[test]
    fn test_generate_with_custom_separator() {
        let options = GenerateOptions {
            components: 2,
            separator: "_".to_string(),
            ..Default::default()
        };
        let id = generate(options).unwrap();

        assert!(id.contains('_'), "Should contain custom separator");
        assert!(!id.contains('-'), "Should not contain default separator");

        let parts: Vec<&str> = id.split('_').collect();
        assert_eq!(parts.len(), 2, "Should have 2 parts with custom separator");
    }

    #[test]
    fn test_generate_with_empty_separator() {
        let options = GenerateOptions {
            components: 2,
            separator: "".to_string(),
            ..Default::default()
        };
        assert!(
            generate(options).is_err(),
            "Should fail with empty separator"
        );
    }

    #[test]
    fn test_generate_with_suffix() {
        let options = GenerateOptions {
            components: 2,
            suffix: Some(suffix_generators::number),
            ..Default::default()
        };
        let id = generate(options).unwrap();

        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 3, "Should have 2 components + 1 suffix");

        // Last part should be numeric
        let last_part = parts.last().unwrap();
        assert!(
            last_part.chars().all(|c| c.is_ascii_digit()),
            "Suffix should be numeric"
        );
        assert_eq!(last_part.len(), 3, "Number suffix should be 3 digits");
    }

    #[test]
    fn test_default_suffix() {
        let suffix = default_suffix().unwrap();
        assert_eq!(suffix.len(), 3, "Default suffix should be 3 digits");
        assert!(
            suffix.chars().all(|c| c.is_ascii_digit()),
            "Default suffix should be numeric"
        );
    }

    #[test]
    fn test_parse_id_without_suffix() {
        let parsed = parse("cute-rabbit", "-").unwrap();
        assert_eq!(parsed.components, vec!["cute", "rabbit"]);
        assert_eq!(parsed.suffix, None);
    }

    #[test]
    fn test_parse_id_with_suffix() {
        let parsed = parse("cute-rabbit-042", "-").unwrap();
        assert_eq!(parsed.components, vec!["cute", "rabbit"]);
        assert_eq!(parsed.suffix, Some("042".to_string()));
    }

    #[test]
    fn test_parse_id_with_custom_separator() {
        let parsed = parse("cute_rabbit_123", "_").unwrap();
        assert_eq!(parsed.components, vec!["cute", "rabbit"]);
        assert_eq!(parsed.suffix, Some("123".to_string()));
    }

    #[test]
    fn test_parse_empty_id() {
        assert!(parse("", "-").is_err(), "Should fail with empty ID");
    }

    #[test]
    fn test_parse_single_component() {
        let parsed = parse("cute", "-").unwrap();
        assert_eq!(parsed.components, vec!["cute"]);
        assert_eq!(parsed.suffix, None);
    }

    #[test]
    fn test_parse_single_component_with_numeric_suffix() {
        let parsed = parse("cute-123", "-").unwrap();
        assert_eq!(parsed.components, vec!["cute"]);
        assert_eq!(parsed.suffix, Some("123".to_string()));
    }

    #[test]
    fn test_calculate_collision_probability() {
        // Edge cases
        assert_eq!(calculate_collision_probability(1000, 0), 0.0);
        assert_eq!(calculate_collision_probability(1000, 1), 0.0);
        assert_eq!(calculate_collision_probability(1000, 1000), 1.0);
        assert_eq!(calculate_collision_probability(1000, 1001), 1.0);

        // Normal case - should be between 0 and 1
        let prob = calculate_collision_probability(10000, 100);
        assert!(
            prob > 0.0 && prob < 1.0,
            "Probability should be between 0 and 1"
        );
    }

    #[test]
    fn test_get_collision_analysis() {
        let analysis = get_collision_analysis(2, 1);

        assert!(
            analysis.total_combinations > 0,
            "Should have positive total combinations"
        );
        assert!(
            !analysis.scenarios.is_empty(),
            "Should have collision scenarios"
        );

        // Check scenarios are sorted by ID count
        for i in 1..analysis.scenarios.len() {
            assert!(
                analysis.scenarios[i].ids > analysis.scenarios[i - 1].ids,
                "Scenarios should be sorted by ID count"
            );
        }

        // Check probability increases with more IDs
        for i in 1..analysis.scenarios.len() {
            assert!(
                analysis.scenarios[i].probability
                    >= analysis.scenarios[i - 1].probability,
                "Probability should increase with more IDs"
            );
        }
    }

    #[test]
    fn test_suffix_generators_number() {
        let suffix = suffix_generators::number().unwrap();
        assert_eq!(suffix.len(), 3, "Number suffix should be 3 digits");
        assert!(
            suffix.chars().all(|c| c.is_ascii_digit()),
            "Number suffix should be numeric"
        );
    }

    #[test]
    fn test_suffix_generators_number4() {
        let suffix = suffix_generators::number4().unwrap();
        assert_eq!(suffix.len(), 4, "Number4 suffix should be 4 digits");
        assert!(
            suffix.chars().all(|c| c.is_ascii_digit()),
            "Number4 suffix should be numeric"
        );
    }

    #[test]
    fn test_suffix_generators_hex() {
        let suffix = suffix_generators::hex().unwrap();
        assert_eq!(suffix.len(), 2, "Hex suffix should be 2 characters");
        assert!(
            suffix.chars().all(|c| c.is_ascii_hexdigit()),
            "Hex suffix should be hexadecimal"
        );
    }

    #[test]
    fn test_suffix_generators_timestamp() {
        let suffix = suffix_generators::timestamp().unwrap();
        assert_eq!(suffix.len(), 4, "Timestamp suffix should be 4 digits");
        assert!(
            suffix.chars().all(|c| c.is_ascii_digit()),
            "Timestamp suffix should be numeric"
        );
    }

    #[test]
    fn test_suffix_generators_letter() {
        let suffix = suffix_generators::letter().unwrap();
        assert_eq!(suffix.len(), 1, "Letter suffix should be 1 character");
        assert!(
            suffix.chars().all(|c| c.is_ascii_lowercase()),
            "Letter suffix should be lowercase"
        );
    }

    #[test]
    fn test_generate_options_default() {
        let options = GenerateOptions::default();
        assert_eq!(options.components, 2);
        assert!(options.suffix.is_none());
        assert_eq!(options.separator, "-");
    }

    #[test]
    fn test_parsed_id_serialization() {
        let parsed = ParsedId {
            components: vec!["cute".to_string(), "rabbit".to_string()],
            suffix: Some("042".to_string()),
        };

        // Test serialization works (requires serde)
        let json = serde_json::to_string(&parsed).unwrap();
        assert!(json.contains("cute"));
        assert!(json.contains("rabbit"));
        assert!(json.contains("042"));

        // Test deserialization works
        let deserialized: ParsedId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, parsed);
    }
}
