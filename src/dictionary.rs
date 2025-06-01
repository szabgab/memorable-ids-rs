//! Dictionary of words for memorable ID generation
//!
//! Contains collections of English words categorized by part of speech.
//! Used to generate human-readable, memorable identifiers.
//!
//! @author Aris Ripandi
//! @license MIT

/// English adjectives (78 total)
/// Descriptive words that modify nouns
pub const ADJECTIVES: &[&str] = &[
    "cute",
    "dapper",
    "large",
    "small",
    "long",
    "short",
    "thick",
    "narrow",
    "deep",
    "flat",
    "whole",
    "low",
    "high",
    "near",
    "far",
    "fast",
    "quick",
    "slow",
    "early",
    "late",
    "bright",
    "dark",
    "cloudy",
    "warm",
    "cool",
    "cold",
    "windy",
    "noisy",
    "loud",
    "quiet",
    "dry",
    "clear",
    "hard",
    "soft",
    "heavy",
    "light",
    "strong",
    "weak",
    "tidy",
    "clean",
    "dirty",
    "empty",
    "full",
    "close",
    "thirsty",
    "hungry",
    "fat",
    "old",
    "fresh",
    "dead",
    "healthy",
    "sweet",
    "sour",
    "bitter",
    "salty",
    "good",
    "bad",
    "great",
    "important",
    "useful",
    "expensive",
    "cheap",
    "free",
    "difficult",
    "able",
    "rich",
    "afraid",
    "brave",
    "fine",
    "sad",
    "proud",
    "comfortable",
    "happy",
    "clever",
    "interesting",
    "famous",
    "exciting",
    "funny",
    "kind",
    "polite",
    "fair",
    "busy",
    "lazy",
    "lucky",
    "careful",
    "safe",
    "dangerous",
];

/// English nouns - animals and common objects (68 total)
/// Concrete things, animals, and objects
pub const NOUNS: &[&str] = &[
    "rabbit",
    "badger",
    "fox",
    "chicken",
    "bat",
    "deer",
    "snake",
    "hare",
    "hedgehog",
    "platypus",
    "mole",
    "mouse",
    "otter",
    "rat",
    "squirrel",
    "stoat",
    "weasel",
    "crow",
    "dove",
    "duck",
    "goose",
    "hawk",
    "heron",
    "kingfisher",
    "owl",
    "peacock",
    "pheasant",
    "pigeon",
    "robin",
    "rook",
    "sparrow",
    "starling",
    "swan",
    "ant",
    "bee",
    "butterfly",
    "dragonfly",
    "fly",
    "moth",
    "spider",
    "pike",
    "salmon",
    "trout",
    "frog",
    "newt",
    "toad",
    "crab",
    "lobster",
    "clam",
    "cockle",
    "mussel",
    "oyster",
    "snail",
    "cow",
    "dog",
    "donkey",
    "goat",
    "horse",
    "pig",
    "sheep",
    "ferret",
    "gerbil",
    "guinea-pig",
    "parrot",
    "book",
    "table",
    "chair",
    "lamp",
    "phone",
    "computer",
    "window",
    "door",
];

/// English verbs - present tense (40 total)
/// Action words in present tense form
pub const VERBS: &[&str] = &[
    "sing", "play", "knit", "flounder", "dance", "listen", "run", "talk",
    "cuddle", "sit", "kiss", "hug", "whimper", "hide", "fight", "whisper",
    "cry", "snuggle", "walk", "drive", "loiter", "feel", "jump", "hop", "go",
    "marry", "engage", "sleep", "eat", "drink", "read", "write", "swim", "fly",
    "climb", "build", "create", "explore", "discover", "learn",
];

/// English adverbs (27 total)
/// Words that modify verbs, adjectives, or other adverbs
pub const ADVERBS: &[&str] = &[
    "jovially",
    "merrily",
    "cordially",
    "carefully",
    "correctly",
    "eagerly",
    "easily",
    "fast",
    "loudly",
    "patiently",
    "quickly",
    "quietly",
    "slowly",
    "gently",
    "firmly",
    "softly",
    "boldly",
    "bravely",
    "calmly",
    "clearly",
    "closely",
    "deeply",
    "directly",
    "exactly",
    "fairly",
    "freely",
    "fully",
];

/// English prepositions (26 total)
/// Words that show relationships between other words
pub const PREPOSITIONS: &[&str] = &[
    "in", "on", "at", "by", "for", "with", "from", "to", "of", "about",
    "under", "over", "through", "between", "among", "during", "before",
    "after", "above", "below", "beside", "behind", "beyond", "within",
    "without", "across",
];

/// Dictionary statistics for combination calculations
#[derive(Debug, Clone, Copy)]
pub struct DictionaryStats {
    pub adjectives: usize,
    pub nouns: usize,
    pub verbs: usize,
    pub adverbs: usize,
    pub prepositions: usize,
}

impl DictionaryStats {
    /// Get dictionary statistics
    pub const fn new() -> Self {
        Self {
            adjectives: ADJECTIVES.len(),
            nouns: NOUNS.len(),
            verbs: VERBS.len(),
            adverbs: ADVERBS.len(),
            prepositions: PREPOSITIONS.len(),
        }
    }
}

/// All word collections grouped by type
#[derive(Debug, Clone)]
pub struct Dictionary {
    pub adjectives: &'static [&'static str],
    pub nouns: &'static [&'static str],
    pub verbs: &'static [&'static str],
    pub adverbs: &'static [&'static str],
    pub prepositions: &'static [&'static str],
    pub stats: DictionaryStats,
}

impl Dictionary {
    /// Create a new dictionary instance
    pub const fn new() -> Self {
        Self {
            adjectives: ADJECTIVES,
            nouns: NOUNS,
            verbs: VERBS,
            adverbs: ADVERBS,
            prepositions: PREPOSITIONS,
            stats: DictionaryStats::new(),
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the default dictionary instance
pub const fn get_dictionary() -> Dictionary {
    Dictionary::new()
}

/// Get dictionary statistics
pub const fn get_dictionary_stats() -> DictionaryStats {
    DictionaryStats::new()
}
