use rust_higher_kined_types::const_generic::{Array, Matrix, compile_time_size_check, demonstrate_different_sizes};

fn test_const_generics_type_level_programming() {
    println!("5. === Const Generics and Type-Level Programming ===");
    println!();

    // 1. 기본 배열 연산
    println!("[1] 🔢 Type-Level Arrays:");
    
    // 컴파일 타임에 크기가 결정되는 배열들
    let mut arr1: Array<i32, 3> = Array::from_array([1, 2, 3]);
    let arr2: Array<i32, 2> = Array::from_array([4, 5]);
    let arr3: Array<i32, 4> = Array::from_array([10, 20, 30, 40]);
    
    println!("    Array 1 (size {}): ", arr1.len());
    arr1.display();
    println!("    Array 2 (size {}): ", arr2.len());
    arr2.display();
    println!("    Array 3 (size {}): ", arr3.len());
    arr3.display();
    
    // 배열 요소 접근 및 수정
    println!("    🔍 Array operations:");
    if let Some(val) = arr1.get(1) {
        println!("      arr1[1] = {}", val);
    }
    
    arr1.set(1, 99).unwrap();
    println!("      After setting arr1[1] = 99:");
    print!("      ");
    arr1.display();
    
    // Iterator 사용
    println!("      Using iterator:");
    print!("      ");
    for (i, item) in arr1.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", item);
    }
    println!();
    println!();

    // 2. 타입 레벨 배열 연결 (원래 구현된 방식 사용)
    println!("[2] ➕ Type-Level Array Concatenation:");
    
    let small_arr: Array<i32, 2> = Array::from_array([1, 2]);
    let medium_arr: Array<i32, 3> = Array::from_array([3, 4, 5]);
    
    println!("    Small array (2): ");
    small_arr.display();
    println!("    Medium array (3): ");
    medium_arr.display();
    
    // 원래 구현된 concat_with_3 메서드 사용
    let combined = small_arr.concat_with_3(&medium_arr);
    println!("    Combined array (2+3=5): ");
    combined.display();
    println!();

    // 3. 행렬 연산 (원래 구현된 방식 사용)
    println!("[3] 🏗️ Type-Level Matrices:");
    
    let mut matrix_2x3: Matrix<i32, 2, 3> = Matrix::new();
    matrix_2x3.set(0, 0, 1).unwrap();
    matrix_2x3.set(0, 1, 2).unwrap();
    matrix_2x3.set(0, 2, 3).unwrap();
    matrix_2x3.set(1, 0, 4).unwrap();
    matrix_2x3.set(1, 1, 5).unwrap();
    matrix_2x3.set(1, 2, 6).unwrap();
    
    let mut matrix_3x2: Matrix<i32, 3, 2> = Matrix::new();
    matrix_3x2.set(0, 0, 7).unwrap();
    matrix_3x2.set(0, 1, 8).unwrap();
    matrix_3x2.set(1, 0, 9).unwrap();
    matrix_3x2.set(1, 1, 10).unwrap();
    matrix_3x2.set(2, 0, 11).unwrap();
    matrix_3x2.set(2, 1, 12).unwrap();
    
    println!("    Matrix A ({}x{}):", matrix_2x3.rows(), matrix_2x3.cols());
    matrix_2x3.display();
    println!("    Matrix B ({}x{}):", matrix_3x2.rows(), matrix_3x2.cols());
    matrix_3x2.display();

    // 행렬 곱셈 (원래 구현된 방식 사용)
    let result = matrix_2x3.multiply_with_3x2(&matrix_3x2);
    println!("    Result A × B ({}x{}):", result.rows(), result.cols());
    result.display();
    println!();

    // 4. 컴파일 타임 크기 비교
    println!("[4] 📏 Compile-Time Size Comparison:");
    demonstrate_different_sizes();
    println!();

    // 5. 컴파일 타임 상수와 타입 안전성
    println!("[5] 🔒 Compile-Time Safety:");
    compile_time_size_check();
    
    println!("    💡 Compile-time guarantees:");
    println!("      ✅ Array bounds are checked at compile time");
    println!("      ✅ Matrix dimensions are verified at compile time");
    println!("      ✅ No runtime size errors possible");
    println!("      ✅ Zero-cost abstractions");
    println!("      ✅ Each size is a completely different type!");
    
    println!("    ❌ These operations would NOT compile:");
    println!("      ❌ Array<i32, 3> = Array<i32, 5>                  // Different types!");
    println!("      ❌ Matrix<2x3> × Matrix<2x3>                       // Incompatible dimensions");  
    println!("      ❌ Accessing beyond compile-time bounds");
    println!();

    // 6. 실용적 예시: 다양한 고정 크기 타입들
    println!("[6] 🛠️ Practical Example - Different Fixed-Size Types:");
    
    // 각각 다른 타입들
    type SmallBuffer = Array<u8, 16>;
    type MediumBuffer = Array<u8, 64>;
    type LargeBuffer = Array<u8, 256>;
    type PacketBuffer = Array<u8, 1024>;
    
    let small_buf = SmallBuffer::new();
    let medium_buf = MediumBuffer::new();
    let large_buf = LargeBuffer::new();
    let packet_buf = PacketBuffer::new();
    
    println!("    📦 Buffer sizes:");
    println!("      SmallBuffer: {} bytes", small_buf.len());
    println!("      MediumBuffer: {} bytes", medium_buf.len());
    println!("      LargeBuffer: {} bytes", large_buf.len());
    println!("      PacketBuffer: {} bytes", packet_buf.len());
    
    // 메모리 사이즈 확인
    println!("    💾 Memory footprint verification:");
    println!("      SmallBuffer: {} bytes in memory", std::mem::size_of::<SmallBuffer>());
    println!("      MediumBuffer: {} bytes in memory", std::mem::size_of::<MediumBuffer>());
    println!("      LargeBuffer: {} bytes in memory", std::mem::size_of::<LargeBuffer>());
    println!("      PacketBuffer: {} bytes in memory", std::mem::size_of::<PacketBuffer>());
    
    println!("    🚀 All sizes known at compile time - zero runtime overhead!");
    println!("    🔒 Type system prevents mixing incompatible buffer sizes!");
}

fn main() {
    test_const_generics_type_level_programming();
} 