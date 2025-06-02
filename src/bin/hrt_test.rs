use rust_higher_kined_types::with_lifetime::{process_any_lifetime, WordCounter};

fn test_with_lifetime_higher_ranked_types() {
    println!("2. === Advanced Trait Bounds with Higher-Ranked Types ===");

    let processor = WordCounter;
    let results = process_any_lifetime(processor);
    println!("    Results: {:?}", results);
}

fn main() {
    test_with_lifetime_higher_ranked_types();
} 