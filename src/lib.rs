//! Memorable ID Generator
//!
//! A flexible library for generating human-readable, memorable identifiers.
//! Uses combinations of adjectives, nouns, verbs, adverbs, and prepositions
//! with optional numeric/custom suffixes.
//!
//! @author Aris Ripandi
//! @license MIT

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub mod dictionary;

use dictionary::{ADJECTIVES, ADVERBS, NOUNS, PREPOSITIONS, VERBS};

/// Error types for memorable ID operations
#[derive(Error, Debug)]
pub enum MemorableIdError {
    #[error("Components must be between 1 and 5, got {0}")]
    InvalidComponentCount(usize),
    #[error("Invalid separator: cannot be empty")]
    InvalidSeparator,
    #[error("Failed to parse ID: {0}")]
    ParseError(String),
}

/// Type alias for suffix generator function
pub type SuffixGenerator = fn() -> Option<String>;

/// Configuration options for ID generation
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Number of word components (1-5, default: 2)
    pub components: usize,
    /// Suffix generator function (default: None)
    pub suffix: Option<SuffixGenerator>,
    /// Separator between parts (default: "-")
    pub separator: String,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            components: 2,
            suffix: None,
            separator: "-".to_string(),
        }
    }
}

/// Parsed ID components structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedId {
    /// Array of word components
    pub components: Vec<String>,
    /// Suffix part if detected, None otherwise
    pub suffix: Option<String>,
}

/// Collision scenario analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionScenario {
    /// Number of IDs in scenario
    pub ids: usize,
    /// Collision probability (0-1)
    pub probability: f64,
    /// Formatted percentage string
    pub percentage: String,
}

/// Collision analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionAnalysis {
    /// Total possible combinations
    pub total_combinations: u64,
    /// Array of collision scenarios
    pub scenarios: Vec<CollisionScenario>,
}

/// Generate a memorable ID
///
/// # Arguments
/// * `options` - Configuration options
///
/// # Returns
/// Generated memorable ID
///
/// # Examples
/// ```rust
/// use memorable_ids::{generate, GenerateOptions, suffix_generators};
///
/// // Default: 2 components, no suffix
/// let id = generate(GenerateOptions::default()).unwrap();
/// // Example: "cute-rabbit"
///
/// // 3 components
/// let id = generate(GenerateOptions {
///     components: 3,
///     ..Default::default()
/// }).unwrap();
/// // Example: "large-fox-swim"
///
/// // With numeric suffix
/// let id = generate(GenerateOptions {
///     components: 2,
///     suffix: Some(suffix_generators::number),
///     ..Default::default()
/// }).unwrap();
/// // Example: "quick-mouse-042"
///
/// // Custom separator
/// let id = generate(GenerateOptions {
///     components: 2,
///     separator: "_".to_string(),
///     ..Default::default()
/// }).unwrap();
/// // Example: "warm_duck"
/// ```
pub fn generate(options: GenerateOptions) -> Result<String, MemorableIdError> {
    if options.components < 1 || options.components > 5 {
        return Err(MemorableIdError::InvalidComponentCount(
            options.components,
        ));
    }

    if options.separator.is_empty() {
        return Err(MemorableIdError::InvalidSeparator);
    }

    let mut rng = rand::rng();
    let mut parts = Vec::new();

    // Component generators in order
    let component_arrays = [ADJECTIVES, NOUNS, VERBS, ADVERBS, PREPOSITIONS];

    // Generate requested number of components
    for i in 0..options.components {
        let array = component_arrays[i];
        let index = rng.random_range(0..array.len());
        parts.push(array[index].to_string());
    }

    // Add suffix if provided
    if let Some(suffix_fn) = options.suffix {
        if let Some(suffix_value) = suffix_fn() {
            parts.push(suffix_value);
        }
    }

    Ok(parts.join(&options.separator))
}

/// Default suffix generator - random 3-digit number
///
/// # Returns
/// Random number suffix (000-999)
///
/// # Examples
/// ```rust
/// use memorable_ids::default_suffix;
///
/// let suffix = default_suffix().unwrap(); // "042"
/// let suffix = default_suffix().unwrap(); // "789"
/// ```
pub fn default_suffix() -> Option<String> {
    let mut rng = rand::rng();
    Some(format!("{:03}", rng.random_range(0..1000)))
}

/// Parse a memorable ID back to its components
///
/// # Arguments
/// * `id` - The memorable ID to parse
/// * `separator` - Separator used (default: "-")
///
/// # Returns
/// Parsed components with structure
///
/// # Examples
/// ```rust
/// use memorable_ids::parse;
///
/// let parsed = parse("cute-rabbit-042", "-").unwrap();
/// // ParsedId { components: ["cute", "rabbit"], suffix: Some("042") }
///
/// let parsed = parse("large-fox-swim", "-").unwrap();
/// // ParsedId { components: ["large", "fox", "swim"], suffix: None }
/// ```
pub fn parse(id: &str, separator: &str) -> Result<ParsedId, MemorableIdError> {
    if id.is_empty() {
        return Err(MemorableIdError::ParseError(
            "ID cannot be empty".to_string(),
        ));
    }

    let parts: Vec<String> =
        id.split(separator).map(|s| s.to_string()).collect();

    if parts.is_empty() {
        return Err(MemorableIdError::ParseError("No parts found".to_string()));
    }

    let mut result = ParsedId {
        components: Vec::new(),
        suffix: None,
    };

    // Last part is likely suffix if it's numeric
    if let Some(last_part) = parts.last() {
        if last_part.chars().all(|c| c.is_ascii_digit()) {
            result.suffix = Some(last_part.clone());
            result.components = parts[..parts.len() - 1].to_vec();
        } else {
            result.components = parts;
        }
    }

    Ok(result)
}

/// Calculate total possible combinations for given configuration
///
/// # Arguments
/// * `components` - Number of word components (1-5)
/// * `suffix_range` - Range of suffix values (default: 1 for no suffix)
///
/// # Returns
/// Total possible unique combinations
///
/// # Examples
/// ```rust
/// use memorable_ids::calculate_combinations;
///
/// let total = calculate_combinations(2, 1); // 5,304 (2 components, no suffix)
/// let total = calculate_combinations(2, 1000); // 5,304,000 (2 components + 3-digit suffix)
/// let total = calculate_combinations(3, 1); // 212,160 (3 components, no suffix)
/// ```
pub fn calculate_combinations(components: usize, suffix_range: u64) -> u64 {
    let stats = get_dictionary_stats();
    let component_sizes = [
        stats.adjectives as u64,
        stats.nouns as u64,
        stats.verbs as u64,
        stats.adverbs as u64,
        stats.prepositions as u64,
    ];

    let mut total = 1u64;
    for i in 0..components.min(5) {
        total = total.saturating_mul(component_sizes[i]);
    }

    total.saturating_mul(suffix_range)
}

/// Calculate collision probability using Birthday Paradox
///
/// # Arguments
/// * `total_combinations` - Total possible combinations
/// * `generated_ids` - Number of IDs to generate
///
/// # Returns
/// Collision probability (0-1)
///
/// # Examples
/// ```rust
/// use memorable_ids::calculate_collision_probability;
///
/// // For 2 components (5,304 total), generating 100 IDs
/// let prob = calculate_collision_probability(5304, 100); // ~0.0093 (0.93%)
///
/// // For 3 components (212,160 total), generating 10,000 IDs
/// let prob = calculate_collision_probability(212160, 10000); // ~0.00235 (0.235%)
/// ```
pub fn calculate_collision_probability(
    total_combinations: u64,
    generated_ids: usize,
) -> f64 {
    if generated_ids >= total_combinations as usize {
        return 1.0;
    }
    if generated_ids <= 1 {
        return 0.0;
    }

    // Birthday paradox approximation: 1 - e^(-n²/2N)
    let n = generated_ids as f64;
    let total = total_combinations as f64;
    let exponent = -(n * n) / (2.0 * total);
    1.0 - exponent.exp()
}

/// Get collision analysis for different ID generation scenarios
///
/// # Arguments
/// * `components` - Number of components
/// * `suffix_range` - Suffix range (1 for no suffix)
///
/// # Returns
/// Analysis with total combinations and collision probabilities
///
/// # Examples
/// ```rust
/// use memorable_ids::get_collision_analysis;
///
/// let analysis = get_collision_analysis(2, 1);
/// // CollisionAnalysis {
/// //   total_combinations: 5304,
/// //   scenarios: [
/// //     CollisionScenario { ids: 100, probability: 0.0093, percentage: "0.93%" },
/// //     CollisionScenario { ids: 500, probability: 0.218, percentage: "21.8%" },
/// //     ...
/// //   ]
/// // }
/// ```
pub fn get_collision_analysis(
    components: usize,
    suffix_range: u64,
) -> CollisionAnalysis {
    let total = calculate_combinations(components, suffix_range);
    let test_sizes = [50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000];

    let scenarios: Vec<CollisionScenario> = test_sizes
        .iter()
        .filter(|&&size| (size as u64) < (total * 80 / 100)) // Only show realistic scenarios
        .map(|&size| {
            let probability = calculate_collision_probability(total, size);
            CollisionScenario {
                ids: size,
                probability,
                percentage: format!("{:.2}%", probability * 100.0),
            }
        })
        .collect();

    CollisionAnalysis {
        total_combinations: total,
        scenarios,
    }
}

/// Collection of predefined suffix generators
pub mod suffix_generators {
    use super::*;

    /// Random 3-digit number (000-999)
    /// Adds 1,000x multiplier to total combinations
    pub fn number() -> Option<String> {
        let mut rng = rand::rng();
        Some(format!("{:03}", rng.random_range(0..1000)))
    }

    /// Random 4-digit number (0000-9999)
    /// Adds 10,000x multiplier to total combinations
    pub fn number4() -> Option<String> {
        let mut rng = rand::rng();
        Some(format!("{:04}", rng.random_range(0..10000)))
    }

    /// Random 2-digit hex (00-ff)
    /// Adds 256x multiplier to total combinations
    pub fn hex() -> Option<String> {
        let mut rng = rand::rng();
        Some(format!("{:02x}", rng.random_range(0..256)))
    }

    /// Last 4 digits of current timestamp
    /// Adds ~10,000x multiplier (time-based, not truly random)
    pub fn timestamp() -> Option<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Some(format!("{:04}", now % 10000))
    }

    /// Random lowercase letter (a-z)
    /// Adds 26x multiplier to total combinations
    pub fn letter() -> Option<String> {
        let mut rng = rand::rng();
        let letter = (b'a' + rng.random_range(0..26)) as char;
        Some(letter.to_string())
    }
}

// Re-export dictionary for external use
pub use dictionary::{
    get_dictionary, get_dictionary_stats, Dictionary, DictionaryStats,
};
