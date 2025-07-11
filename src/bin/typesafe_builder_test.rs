//
// Type-Safe Builder Pattern
// -- Let's create a type-safe builder pattern with compile-time validation:
// 
use rust_higher_kined_types::custom_types::{PersonBuilder, Person};

fn test_typesafe_builder_pattern() {
    println!("6. === Type-Safe Builder Pattern ===");
    println!();

    // 1. 기본 빌더 패턴 시연
    println!("[1] 🏗️ Basic Builder Pattern:");
    
    // 완전한 Person 객체 생성
    let person = PersonBuilder::new()
        .name("Alice Smith".to_string())
        .age(30)
        .email("alice@example.com".to_string())
        .build();
    
    println!("    ✅ Successfully built person:");
    println!("      Name: {}", person.name());
    println!("      Age: {}", person.age());
    println!("      Email: {}", person.email());
    println!("      Debug: {:?}", person);
    println!();

    // 2. 순서에 상관없이 필드 설정
    println!("[2] 🔀 Field Order Independence:");
    
    let person2 = PersonBuilder::new()
        .email("bob@example.com".to_string())  // 이메일 먼저
        .name("Bob Johnson".to_string())       // 그 다음 이름
        .age(25)                               // 마지막에 나이
        .build();
    
    println!("    ✅ Built person with different field order:");
    println!("      Name: {}", person2.name());
    println!("      Age: {}", person2.age());
    println!("      Email: {}", person2.email());
    println!();

    // 3. 빌더 상태 확인
    println!("[3] 🔍 Builder State Inspection:");
    
    let partial_builder = PersonBuilder::new()
        .name("Charlie Brown".to_string());
    
    println!("    Partial builder state:");
    println!("      Has name: {}", partial_builder.has_name());
    println!("      Has age: {}", partial_builder.has_age());
    println!("      Has email: {}", partial_builder.has_email());
    
    let complete_builder = partial_builder
        .age(35)
        .email("charlie@example.com".to_string());
    
    println!("    Complete builder state:");
    println!("      Has name: {}", complete_builder.has_name());
    println!("      Has age: {}", complete_builder.has_age());
    println!("      Has email: {}", complete_builder.has_email());
    
    let person3 = complete_builder.build();
    println!("    ✅ Final person: {:?}", person3);
    println!();

    // 4. 타입 안전성 데모
    println!("[4] 🔒 Type Safety Demonstration:");
    rust_higher_kined_types::custom_types::typesafe_builder::demonstrate_builder_safety();
    println!();

    // 5. 실용적 사용 예시
    println!("[5] 🛠️ Practical Usage Examples:");
    
    // 여러 사람 생성
    let people = vec![
        PersonBuilder::new()
            .name("David Wilson".to_string())
            .age(42)
            .email("david@company.com".to_string())
            .build(),
        PersonBuilder::new()
            .name("Emma Davis".to_string())
            .age(28)
            .email("emma@startup.io".to_string())
            .build(),
        PersonBuilder::new()
            .name("Frank Miller".to_string())
            .age(55)
            .email("frank@enterprise.org".to_string())
            .build(),
    ];
    
    println!("    📋 Created {} people:", people.len());
    for (i, person) in people.iter().enumerate() {
        println!("      {}. {} ({}) - {}", 
                 i + 1, person.name(), person.age(), person.email());
    }
    println!();

    // 6. 컴파일 타임 안전성 예시
    println!("[6] 💡 Compile-Time Safety Examples:");
    println!("    ❌ These operations would NOT compile:");
    println!("    ❌ PersonBuilder::new().build()                 // Missing all fields");
    println!("    ❌ PersonBuilder::new().name(...).build()       // Missing age and email");
    println!("    ❌ PersonBuilder::new().name(...).age(...).build() // Missing email");
    println!("    ✅ Only PersonBuilder::new().name(...).age(...).email(...).build() compiles!");
    println!();

    // 7. 메모리 효율성
    println!("[7] 💾 Memory Efficiency:");
    println!("    📊 Type information exists only at compile time:");
    println!("      PersonBuilder size: {} bytes", std::mem::size_of::<PersonBuilder<(), (), ()>>());
    println!("      Person size: {} bytes", std::mem::size_of::<Person>());
    println!("      Zero runtime overhead for type safety!");
    println!();

    // 8. 고급 패턴 활용
    println!("[8] 🎭 Advanced Pattern Usage:");
    
    // 함수에서 빌더 반환
    fn create_template_builder() -> PersonBuilder<(), (), ()> {
        PersonBuilder::new()
    }
    
    // 실제 사용 예시
    let template = create_template_builder();
    let final_person = template
        .name("Generic Employee".to_string())
        .age(30)
        .email("employee@company.com".to_string())
        .build();
    
    println!("    🏭 Template-based person: {}", final_person.name());
    println!("    ✅ Builder pattern enables flexible object construction!");
}

fn main() {
    test_typesafe_builder_pattern();
}