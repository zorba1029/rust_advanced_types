// 
// Higher-Kinded Types (HKT)
// 
use rust_higher_kined_types::container::Container;

// Now we can write generic code that works with any Container
fn double_container<C: Container<Item = i32>>(container: C) -> C::Mapped<i64> {
    container.map(|&x| x as i64 * 2)
}

fn test_container_higher_kinded_types() {
    println!("1. === Associated Type Constructors and Higher-Kinded Types ===");
    
    let option: Option<i32> = Some(1);
    let result: Result<i32, &str> = Ok(1);

    let doubled_option = double_container(option);
    let doubled_result = double_container(result);

    println!("    Doubled Option: {:?}", doubled_option);
    println!("    Doubled Result: {:?}", doubled_result);
}

fn main() {
    test_container_higher_kinded_types();
} 