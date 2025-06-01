//! Tests for dictionary module
//!
//! Comprehensive tests for dictionary functionality including
//! word arrays, statistics, and dictionary structure validation.

use memorable_ids::dictionary::*;

#[cfg(test)]
mod dictionary_tests {
    use super::*;

    #[test]
    fn test_dictionary_stats_counts() {
        let stats = get_dictionary_stats();
        assert_eq!(stats.adjectives, 87, "Expected 87 adjectives");
        assert_eq!(stats.nouns, 72, "Expected 72 nouns");
        assert_eq!(stats.verbs, 40, "Expected 40 verbs");
        assert_eq!(stats.adverbs, 27, "Expected 27 adverbs");
        assert_eq!(stats.prepositions, 26, "Expected 26 prepositions");
    }

    #[test]
    fn test_dictionary_creation() {
        let dict = get_dictionary();
        assert_eq!(dict.adjectives.len(), 87);
        assert_eq!(dict.nouns.len(), 72);
        assert_eq!(dict.verbs.len(), 40);
        assert_eq!(dict.adverbs.len(), 27);
        assert_eq!(dict.prepositions.len(), 26);
    }

    #[test]
    fn test_word_arrays_not_empty() {
        assert!(
            !ADJECTIVES.is_empty(),
            "Adjectives array should not be empty"
        );
        assert!(!NOUNS.is_empty(), "Nouns array should not be empty");
        assert!(!VERBS.is_empty(), "Verbs array should not be empty");
        assert!(!ADVERBS.is_empty(), "Adverbs array should not be empty");
        assert!(
            !PREPOSITIONS.is_empty(),
            "Prepositions array should not be empty"
        );
    }

    #[test]
    fn test_word_arrays_contain_expected_words() {
        // Test some known words exist in each category
        assert!(ADJECTIVES.contains(&"cute"), "Should contain 'cute'");
        assert!(ADJECTIVES.contains(&"large"), "Should contain 'large'");
        assert!(
            ADJECTIVES.contains(&"dangerous"),
            "Should contain 'dangerous'"
        );

        assert!(NOUNS.contains(&"rabbit"), "Should contain 'rabbit'");
        assert!(NOUNS.contains(&"fox"), "Should contain 'fox'");
        assert!(NOUNS.contains(&"computer"), "Should contain 'computer'");

        assert!(VERBS.contains(&"sing"), "Should contain 'sing'");
        assert!(VERBS.contains(&"run"), "Should contain 'run'");
        assert!(VERBS.contains(&"learn"), "Should contain 'learn'");

        assert!(ADVERBS.contains(&"jovially"), "Should contain 'jovially'");
        assert!(ADVERBS.contains(&"quickly"), "Should contain 'quickly'");
        assert!(ADVERBS.contains(&"fully"), "Should contain 'fully'");

        assert!(PREPOSITIONS.contains(&"in"), "Should contain 'in'");
        assert!(PREPOSITIONS.contains(&"over"), "Should contain 'over'");
        assert!(PREPOSITIONS.contains(&"across"), "Should contain 'across'");
    }

    #[test]
    fn test_no_duplicate_words() {
        // Test each array has no duplicates
        let mut unique_adjectives = ADJECTIVES.to_vec();
        unique_adjectives.sort();
        unique_adjectives.dedup();
        assert_eq!(
            unique_adjectives.len(),
            ADJECTIVES.len(),
            "Adjectives should have no duplicates"
        );

        let mut unique_nouns = NOUNS.to_vec();
        unique_nouns.sort();
        unique_nouns.dedup();
        assert_eq!(
            unique_nouns.len(),
            NOUNS.len(),
            "Nouns should have no duplicates"
        );

        let mut unique_verbs = VERBS.to_vec();
        unique_verbs.sort();
        unique_verbs.dedup();
        assert_eq!(
            unique_verbs.len(),
            VERBS.len(),
            "Verbs should have no duplicates"
        );

        let mut unique_adverbs = ADVERBS.to_vec();
        unique_adverbs.sort();
        unique_adverbs.dedup();
        assert_eq!(
            unique_adverbs.len(),
            ADVERBS.len(),
            "Adverbs should have no duplicates"
        );

        let mut unique_prepositions = PREPOSITIONS.to_vec();
        unique_prepositions.sort();
        unique_prepositions.dedup();
        assert_eq!(
            unique_prepositions.len(),
            PREPOSITIONS.len(),
            "Prepositions should have no duplicates"
        );
    }

    #[test]
    fn test_words_are_lowercase() {
        // Test all words are lowercase (except hyphenated ones)
        for word in ADJECTIVES {
            assert!(
                word.chars().all(|c| c.is_lowercase() || c == '-'),
                "Adjective '{}' should be lowercase",
                word
            );
        }

        for word in NOUNS {
            assert!(
                word.chars().all(|c| c.is_lowercase() || c == '-'),
                "Noun '{}' should be lowercase",
                word
            );
        }

        for word in VERBS {
            assert!(
                word.chars().all(|c| c.is_lowercase() || c == '-'),
                "Verb '{}' should be lowercase",
                word
            );
        }

        for word in ADVERBS {
            assert!(
                word.chars().all(|c| c.is_lowercase() || c == '-'),
                "Adverb '{}' should be lowercase",
                word
            );
        }

        for word in PREPOSITIONS {
            assert!(
                word.chars().all(|c| c.is_lowercase() || c == '-'),
                "Preposition '{}' should be lowercase",
                word
            );
        }
    }

    #[test]
    fn test_dictionary_stats_new() {
        let stats = DictionaryStats::new();
        assert_eq!(stats.adjectives, ADJECTIVES.len());
        assert_eq!(stats.nouns, NOUNS.len());
        assert_eq!(stats.verbs, VERBS.len());
        assert_eq!(stats.adverbs, ADVERBS.len());
        assert_eq!(stats.prepositions, PREPOSITIONS.len());
    }

    #[test]
    fn test_dictionary_default() {
        let dict1 = Dictionary::default();
        let dict2 = Dictionary::new();

        assert_eq!(dict1.adjectives.len(), dict2.adjectives.len());
        assert_eq!(dict1.nouns.len(), dict2.nouns.len());
        assert_eq!(dict1.verbs.len(), dict2.verbs.len());
        assert_eq!(dict1.adverbs.len(), dict2.adverbs.len());
        assert_eq!(dict1.prepositions.len(), dict2.prepositions.len());
    }

    #[test]
    fn test_words_are_not_empty_strings() {
        for word in ADJECTIVES {
            assert!(!word.is_empty(), "Adjective should not be empty string");
        }

        for word in NOUNS {
            assert!(!word.is_empty(), "Noun should not be empty string");
        }

        for word in VERBS {
            assert!(!word.is_empty(), "Verb should not be empty string");
        }

        for word in ADVERBS {
            assert!(!word.is_empty(), "Adverb should not be empty string");
        }

        for word in PREPOSITIONS {
            assert!(!word.is_empty(), "Preposition should not be empty string");
        }
    }
}
