//
// Generic Associated Types (GATs)
// 
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
    
    println!("--- String Stream with position ---");
    // position과 함께 출력하는 버전
    stream.reset_position();
    while let Some((word, pos)) = stream.next_with_position() {
        println!("    Word: {}, position: {}", word, pos);
    }
    println!("    Final position: {}", stream.position);

    println!("--- Int Stream ---");
    let mut int_stream = IntStream {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };

    while let Some(num) = int_stream.next() {
        println!("    Number: {}", num);
    }

    println!("--- Int Stream with position ---");
    int_stream.reset_position();
    while let Some((num, pos)) = int_stream.next_with_position() {
        println!("    Number: {}, position: {}", num, pos);
    }
    println!("    Final position: {}", int_stream.position);
}

fn main() {
    test_generic_associated_types();
} 