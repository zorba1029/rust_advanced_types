//
// Advanced Type System Features in Practice
// -- Let's combine multiple advanced features to create a type-safe builder pattern 
//    with compile-time validation:

use std::marker::PhantomData;

// Define type-level states for our builder
pub struct Incomplete;
pub struct Complete;

// Type-level validation traits
pub trait HasName {}
pub trait HasAge {}
pub trait HasEmail {}

// Marker structs for tracking what fields have been set
pub struct WithName;
pub struct WithAge;
pub struct WithEmail;

impl HasName for WithName {}
impl HasAge for WithAge {}
impl HasEmail for WithEmail {}

// The final Person struct
#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

// Builder with type-level state tracking
pub struct PersonBuilder<Name, Age, Email> {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
    _phantom: PhantomData<(Name, Age, Email)>,
}

// Starting state - nothing set
impl PersonBuilder<(), (), ()> {
    pub fn new() -> Self {
        Self {
            name: None,
            age: None,
            email: None,
            _phantom: PhantomData,
        }
    }
}

// Setting name
impl<Age, Email> PersonBuilder<(), Age, Email> {
    pub fn name(mut self, name: String) -> PersonBuilder<WithName, Age, Email> {
        self.name = Some(name);
        PersonBuilder {
            name: self.name,
            age: self.age,
            email: self.email,
            _phantom: PhantomData,
        }
    }
}

// Setting age
impl<Name, Email> PersonBuilder<Name, (), Email> {
    pub fn age(mut self, age: u32) -> PersonBuilder<Name, WithAge, Email> {
        self.age = Some(age);
        PersonBuilder {
            name: self.name,
            age: self.age,
            email: self.email,
            _phantom: PhantomData,
        }
    }
}

// Setting email
impl<Name, Age> PersonBuilder<Name, Age, ()> {
    pub fn email(mut self, email: String) -> PersonBuilder<Name, Age, WithEmail> {
        self.email = Some(email);
        PersonBuilder {
            name: self.name,
            age: self.age,
            email: self.email,
            _phantom: PhantomData,
        }
    }
}

// Only allow build when all fields are set
impl PersonBuilder<WithName, WithAge, WithEmail> {
    pub fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
            email: self.email.unwrap(),
        }
    }
}

// Optional: Provide convenience methods for validation
impl<Name, Age, Email> PersonBuilder<Name, Age, Email> {
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub fn has_age(&self) -> bool {
        self.age.is_some()
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }
}

/// 타입 안전성 검증 예제
/// 
/// 필수 필드 누락을 컴파일 타임에 방지하는 것을 확인한다.
pub fn demonstrate_builder_safety() {
    println!("🔒 Demonstrating compile-time builder safety:");
    
    let _builder = PersonBuilder::new();
    // _builder.build(); // ❌ This would NOT compile! Missing required fields
    
    let _builder_with_name = _builder.name("Alice".to_string());
    // _builder_with_name.build(); // ❌ This would NOT compile! Still missing age and email
    
    println!("✅ Type system prevents incomplete object construction!");
    println!("✅ Only builders with ALL required fields can call build()!");
}
    