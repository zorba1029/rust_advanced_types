//
// Advanced Trait Bounds with Higher-Ranked Types
// -- Higher-ranked trait bounds allow us to work with traits that have lifetime parameters 
//    in sophisticated ways:
//----------------------------------------------------------
use std::fmt::Debug;

// Define a trait with a lifetime parameter
pub trait WithLifetime<'a> {
    type Output;

    fn process(&self, input: &'a str) -> Self::Output;
}

// Higher-Ranked trait bound to make the lifetime flexible
pub fn process_any_lifetime<T>(processor: T) -> Vec<String>
where
    // For any lifetime 'a, T implements WithLifetime<'a>
    for<'a> T: WithLifetime<'a>,
    // The Output type must be Debug for any lifetime
    for<'a> <T as WithLifetime<'a>>::Output: Debug,
{
    let inputs = vec!["first", "second", "third"];
    inputs.iter()
        .map(|&s| {
            let result = processor.process(s);
            format!("Processed: {:?}", result)
        })
        .collect()
}

// Example implementation of WithLifetime
pub struct WordCounter;

impl<'a> WithLifetime<'a> for WordCounter {
    type Output = usize;

    fn process(&self, input: &'a str) -> Self::Output {
        input.split_whitespace().count()
    }
}
