use rust_higher_kined_types::gat::{StringStream, Stream, IntStream};

fn test_generic_associated_types() {
    println!("4. === Generic Associated Types (GATs) ===");

    println!("--- String Stream ---");
    let mut stream = StringStream {
        data: "Hello world from the stream".to_string(),
        position: 0,
    };

    while let Some(word) = stream.next() {
        println!("    Word: {}", word);
    }

    println!("--- Int Stream ---");
    let mut int_stream = IntStream {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };

    while let Some(num) = int_stream.next() {
        println!("    Number: {}", num);
    }
}

fn main() {
    test_generic_associated_types();
} 