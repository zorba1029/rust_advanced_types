// 
// Higher-Ranked Types (HRT)
// 
use rust_higher_kined_types::with_lifetime::{process_any_lifetime, WordCounter, WithLifetime};
use std::fmt::Debug;

// 추가 예시를 위한 구현들
struct LengthCounter;
struct UppercaseConverter;
struct CharCounter;

impl<'a> WithLifetime<'a> for LengthCounter {
    type Output = usize;

    fn process(&self, input: &'a str) -> Self::Output {
        input.len()
    }
}

impl<'a> WithLifetime<'a> for UppercaseConverter {
    type Output = String;

    fn process(&self, input: &'a str) -> Self::Output {
        input.to_uppercase()
    }
}

impl<'a> WithLifetime<'a> for CharCounter {
    type Output = usize;

    fn process(&self, input: &'a str) -> Self::Output {
        input.chars().count()
    }
}

// HRT와 클로저 예시
fn apply_to_various_lifetimes<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> usize
{
    println!("    🎯 Testing closure with various lifetimes:");
    
    let local = String::from("local data with longer content");
    println!("      Local string result: {}", f(&local));
    
    println!("      Static string result: {}", f("static data"));
    
    // 중첩 스코프에서도 작동
    {
        let nested = String::from("nested scope");
        println!("      Nested scope result: {}", f(&nested));
    }
}

// 실용적 예시: 다양한 프로세서 체인
fn chain_processors<T1, T2>(first: T1, second: T2) -> Vec<String>
where
    for<'a> T1: WithLifetime<'a>,
    for<'a> <T1 as WithLifetime<'a>>::Output: Debug,
    for<'a> T2: WithLifetime<'a>,
    for<'a> <T2 as WithLifetime<'a>>::Output: Debug,
{
    let inputs = vec!["first input", "second input", "third input"];
    let mut results = Vec::new();
    
    for input in inputs {
        let result1 = first.process(input);
        let result2 = second.process(input);
        results.push(format!("Chain: {:?} -> {:?}", result1, result2));
    }
    
    results
}

// 실제 사용 사례: 다양한 소스에서의 데이터 처리
fn process_from_multiple_sources<P>(processor: P)
where
    for<'a> P: WithLifetime<'a>,
    for<'a> <P as WithLifetime<'a>>::Output: Debug + Clone,
{
    println!("    📊 Processing from multiple data sources:");
    
    // 1. 정적 문자열
    let static_data = "Static configuration data";
    let result1 = processor.process(static_data);
    println!("      Static: {:?}", result1);
    
    // 2. 힙 할당 문자열
    let heap_data = String::from("Dynamically allocated content");
    let result2 = processor.process(&heap_data);
    println!("      Heap: {:?}", result2);
    
    // 3. 함수 내 지역 변수
    {
        let local_data = format!("Generated at {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        let result3 = processor.process(&local_data);
        println!("      Local: {:?}", result3);
    }
    
    // 4. 슬라이스에서
    let array_data = ["item1", "item2", "item3"];
    for (i, item) in array_data.iter().enumerate() {
        let result = processor.process(item);
        println!("      Array[{}]: {:?}", i, result);
    }
}

// 고급 HRT 패턴: 중첩된 라이프타임 처리
trait NestedProcessor<'a, 'b> {
    type Output;
    fn process_nested(&self, outer: &'a str, inner: &'b str) -> Self::Output;
}

struct CombiningProcessor;

impl<'a, 'b> NestedProcessor<'a, 'b> for CombiningProcessor {
    type Output = String;
    
    fn process_nested(&self, outer: &'a str, inner: &'b str) -> Self::Output {
        format!("Combined: '{}' + '{}'", outer, inner)
    }
}

fn use_nested_processor<P>(processor: P)
where
    for<'a, 'b> P: NestedProcessor<'a, 'b>,
    for<'a, 'b> <P as NestedProcessor<'a, 'b>>::Output: Debug,
{
    println!("    🔄 Testing nested lifetime processor:");
    
    let outer_data = String::from("outer scope data");
    let inner_data = "inner static data";
    
    let result = processor.process_nested(&outer_data, inner_data);
    println!("      Nested result: {:?}", result);
}

fn test_with_lifetime_higher_ranked_types() {
    println!("2. === Higher-Ranked Types (Advanced Lifetime Bounds) ===");
    println!();

    // 1. 기본 HRT 데모
    println!("[1] 🔗 Basic Higher-Ranked Types:");
    
    let word_counter = WordCounter;
    let results = process_any_lifetime(word_counter);
    println!("    Word counting results:");
    for result in &results {
        println!("      {}", result);
    }
    println!();

    // 2. 다양한 프로세서들
    println!("[2] 🛠️ Different Processor Types:");
    
    println!("  Length Counter:");
    let length_counter = LengthCounter;
    let length_results = process_any_lifetime(length_counter);
    for result in &length_results {
        println!("    {}", result);
    }
    
    println!("  Character Counter:");
    let char_counter = CharCounter;
    let char_results = process_any_lifetime(char_counter);
    for result in &char_results {
        println!("    {}", result);
    }
    
    println!("  Uppercase Converter:");
    let uppercase_converter = UppercaseConverter;
    let uppercase_results = process_any_lifetime(uppercase_converter);
    for result in &uppercase_results {
        println!("    {}", result);
    }
    println!();

    // 3. 클로저와 HRT
    println!("[3] 🎭 HRT with Closures:");
    apply_to_various_lifetimes(|s: &str| s.len());
    println!();
    
    apply_to_various_lifetimes(|s: &str| s.chars().filter(|c| c.is_alphabetic()).count());
    println!();

    // 4. 다중 소스 처리
    println!("[4] 📁 Multi-Source Processing:");
    process_from_multiple_sources(WordCounter);
    println!();

    // 5. 프로세서 체이닝
    println!("[5] ⛓️ Processor Chaining:");
    let chain_results = chain_processors(WordCounter, LengthCounter);
    println!("    Chain processing results:");
    for result in &chain_results {
        println!("      {}", result);
    }
    println!();

    // 6. 중첩 라이프타임 처리
    println!("[6] 🔄 Nested Lifetime Processing:");
    use_nested_processor(CombiningProcessor);
    println!();

    // 7. 함수 포인터와 HRT
    println!("[7] 🎯 Function Pointers with HRT:");
    
    // HRT를 사용하는 함수 포인터 타입
    type FlexibleFunction = for<'a> fn(&'a str) -> usize;
    
    fn string_length(s: &str) -> usize { s.len() }
    fn word_count(s: &str) -> usize { s.split_whitespace().count() }
    
    let functions: Vec<(&str, FlexibleFunction)> = vec![
        ("Length", string_length),
        ("Words", word_count),
    ];
    
    let test_inputs = ["Hello world", "Rust is awesome!", "Higher-Ranked Types"];
    
    println!("    📊 Function pointer results:");
    for input in test_inputs {
        println!("      Input: '{}'", input);
        for (name, func) in &functions {
            println!("        {}: {}", name, func(input));
        }
    }
    println!();

    // 8. 실용적 예시: 설정 파서
    println!("[8] ⚙️ Practical Example - Configuration Parser:");
    
    trait ConfigParser<'a> {
        type Output;
        fn parse_config(&self, config: &'a str) -> Self::Output;
    }
    
    struct SimpleConfigParser;
    
    impl<'a> ConfigParser<'a> for SimpleConfigParser {
        type Output = Vec<&'a str>;
        
        fn parse_config(&self, config: &'a str) -> Self::Output {
            config.lines().filter(|line| !line.trim().is_empty()).collect()
        }
    }
    
    fn parse_various_configs<P>(parser: P)
    where
        for<'a> P: ConfigParser<'a>,
        for<'a> <P as ConfigParser<'a>>::Output: Debug,
    {
        // 정적 설정
        let static_config = "setting1=value1\nsetting2=value2\n\nsetting3=value3";
        let result1 = parser.parse_config(static_config);
        println!("      Static config: {:?}", result1);
        
        // 동적 설정
        let dynamic_config = format!("server=localhost\nport={}\nssl=true", 8080);
        let result2 = parser.parse_config(&dynamic_config);
        println!("      Dynamic config: {:?}", result2);
    }
    
    parse_various_configs(SimpleConfigParser);
    println!();

    // 9. 타입 안전성 검증
    println!("[9] 🔒 Compile-Time Safety Verification:");
    println!("    ✅ All lifetime bounds are verified at compile time");
    println!("    ✅ No runtime overhead for lifetime polymorphism");
    println!("    ✅ Prevents lifetime-related bugs at compile time");
    println!();
    
    println!("    💡 Key Benefits of Higher-Ranked Types:");
    println!("      🎯 Maximum flexibility with lifetime parameters");
    println!("      🛡️ Compile-time lifetime safety guarantees");
    println!("      🚀 Zero-cost abstraction over lifetimes");
    println!("      🔧 Enables powerful generic programming patterns");
    println!();
    
    println!("    ❌ These would NOT compile (lifetime constraint violations):");
    println!("      ❌ Using processor with incompatible lifetime bounds");
    println!("      ❌ Mixing processors that don't satisfy for<'a> bounds");
    println!("      ❌ Attempting to store references beyond their lifetimes");
}

fn main() {
    test_with_lifetime_higher_ranked_types();
} 