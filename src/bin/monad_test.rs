use rust_higher_kined_types::custom_types::functor_monad::*;
use std::io;

fn main() {
    println!("=== Higher-Kinded Types: Functor and Monad 예제 ===\n");
    
    loop {
        println!("실행할 함수를 선택하세요:");
        println!("1. Option Monad 기본 예제");
        println!("2. Result Monad 기본 예제");
        println!("3. Functor 매핑 예제");
        println!("4. Applicative 예제");
        println!("5. 체인 연산 예제 (Option)");
        println!("6. 체인 연산 예제 (Result)");
        println!("7. 복합 연산 예제");
        println!("8. 에러 처리 예제");
        println!("9. 실용적인 사용 예제");
        println!("10. 모든 예제 실행");
        println!("0. 종료");
        println!();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 읽기 실패");
        
        match input.trim() {
            "1" => test_option_monad(),
            "2" => test_result_monad(),
            "3" => test_functor_mapping(),
            "4" => test_applicative(),
            "5" => test_option_chaining(),
            "6" => test_result_chaining(),
            "7" => test_complex_operations(),
            "8" => test_error_handling(),
            "9" => test_practical_examples(),
            "10" => run_all_examples(),
            "0" => break,
            _ => println!("잘못된 선택입니다. 다시 선택해주세요.\n"),
        }
    }
    
    println!("프로그램을 종료합니다.");
}

fn test_option_monad() {
    println!("=== Option Monad 기본 예제 ===");
    
    // Option::pure 사용
    let value = Option::pure(42);
    println!("Option::pure(42) = {:?}", value);
    
    // bind 사용
    let result = value.bind(|x| Some(x * 2));
    println!("bind(|x| Some(x * 2)) = {:?}", result);
    
    // 연속된 bind 사용
    let final_result = Option::pure(5)
        .bind(|x| Some(x + 10))
        .bind(|x| Some(x * 2))
        .bind(|x| Some(format!("결과: {}", x)));
    println!("연속된 bind 결과: {:?}", final_result);
    
    // None 처리
    let none_result: Option<i32> = None;
    let none_bind = none_result.bind(|x| Some(x * 2));
    println!("None.bind() = {:?}", none_bind);
    
    println!("====================================\n");
}

fn test_result_monad() {
    println!("=== Result Monad 기본 예제 ===");
    
    // Result::pure 사용
    let value: Result<i32, &str> = Result::pure(42);
    println!("Result::pure(42) = {:?}", value);
    
    // bind 사용
    let result = value.bind(|x| Ok(x * 2));
    println!("bind(|x| Ok(x * 2)) = {:?}", result);
    
    // 에러가 있는 경우
    let error_result: Result<i32, &str> = Err("에러 발생");
    let error_bind = error_result.bind(|x| Ok(x * 2));
    println!("Err.bind() = {:?}", error_bind);
    
    // 연속된 bind 사용
    let final_result: Result<String, &str> = Result::pure(10)
        .bind(|x| Ok(x + 5))
        .bind(|x| Ok(x * 2))
        .bind(|x| Ok(format!("최종 결과: {}", x)));
    println!("연속된 bind 결과: {:?}", final_result);
    
    println!("====================================\n");
}

fn test_functor_mapping() {
    println!("=== Functor 매핑 예제 ===");
    
    // Option fmap
    let option_val = Some(10);
    let mapped_option = option_val.fmap(|x| x * 3);
    println!("Some(10).fmap(|x| x * 3) = {:?}", mapped_option);
    
    // Result fmap
    let result_val: Result<i32, &str> = Ok(20);
    let mapped_result = result_val.fmap(|x| x + 5);
    println!("Ok(20).fmap(|x| x + 5) = {:?}", mapped_result);
    
    // 타입 변환
    let string_mapped = Some(42).fmap(|x| format!("숫자: {}", x));
    println!("Some(42).fmap(to_string) = {:?}", string_mapped);
    
    println!("====================================\n");
}

fn test_applicative() {
    println!("=== Applicative 예제 ===");
    
    // Option Applicative - 함수 적용
    let value = Some(10);
    let func = Some(|x: i32| x * 2);
    let result = value.apply(func);
    println!("Some(10).apply(Some(|x| x * 2)) = {:?}", result);
    
    // Option Applicative - 함수가 None인 경우
    let value2 = Some(5);
    let func2: Option<fn(i32) -> i32> = None;
    let result2 = value2.apply(func2);
    println!("Some(5).apply(None) = {:?}", result2);
    
    // Option Applicative - 값이 None인 경우
    let value3: Option<i32> = None;
    let func3 = Some(|x: i32| x + 1);
    let result3 = value3.apply(func3);
    println!("None.apply(Some(|x| x + 1)) = {:?}", result3);
    
    // Result Applicative - 성공적인 적용
    let value4: Result<i32, &str> = Ok(20);
    let func4: Result<fn(i32) -> i32, &str> = Ok(|x| x / 2);
    let result4 = value4.apply(func4);
    println!("Ok(20).apply(Ok(|x| x / 2)) = {:?}", result4);
    
    // Result Applicative - 함수가 Err인 경우
    let value5: Result<i32, &str> = Ok(15);
    let func5: Result<fn(i32) -> i32, &str> = Err("함수 에러");
    let result5 = value5.apply(func5);
    println!("Ok(15).apply(Err('함수 에러')) = {:?}", result5);
    
    // Result Applicative - 값이 Err인 경우
    let value6: Result<i32, &str> = Err("값 에러");
    let func6: Result<fn(i32) -> i32, &str> = Ok(|x| x * 3);
    let result6 = value6.apply(func6);
    println!("Err('값 에러').apply(Ok(|x| x * 3)) = {:?}", result6);
    
    // 복합 Applicative 연산 예제
    println!("\n--- 복합 Applicative 연산 ---");
    let add_func = Some(|x: i32| move |y: i32| x + y);
    let multiply_func = Some(|x: i32| move |y: i32| x * y);
    
    let val1 = Some(5);
    let val2 = Some(3);
    
    // 부분 적용 함수 생성
    let partial_add = val1.apply(add_func);
    let add_result = val2.apply(partial_add);
    println!("Some(5 + 3) using applicative = {:?}", add_result);
    
    let partial_multiply = Some(7).apply(multiply_func);
    let multiply_result = Some(4).apply(partial_multiply);
    println!("Some(7 * 4) using applicative = {:?}", multiply_result);
    
    println!("====================================\n");
}

fn test_option_chaining() {
    println!("=== 체인 연산 예제 (Option) ===");
    
    let result = chain_option_operations(
        Some(5),
        |x| if x > 0 { Some(x * 2) } else { None },
        |x| if x < 20 { Some(x + 10) } else { None }
    );
    println!("chain_option_operations(Some(5), double_if_positive, add_10_if_small) = {:?}", result);
    
    let result2 = chain_option_operations(
        Some(15),
        |x| if x > 0 { Some(x * 2) } else { None },
        |x| if x < 20 { Some(x + 10) } else { None }
    );
    println!("chain_option_operations(Some(15), double_if_positive, add_10_if_small) = {:?}", result2);
    
    println!("=====================================\n");
}

fn test_result_chaining() {
    println!("=== 체인 연산 예제 (Result) ===");
    
    let result: Result<i32, &str> = chain_result_operations(
        Ok(5),
        |x| if x > 0 { Ok(x * 2) } else { Err("음수는 허용되지 않음") },
        |x| if x < 50 { Ok(x + 10) } else { Err("값이 너무 큼") }
    );
    println!("chain_result_operations(Ok(5), positive_double, bounded_add) = {:?}", result);
    
    let result2: Result<i32, &str> = chain_result_operations(
        Ok(30),
        |x| if x > 0 { Ok(x * 2) } else { Err("음수는 허용되지 않음") },
        |x| if x < 50 { Ok(x + 10) } else { Err("값이 너무 큼") }
    );
    println!("chain_result_operations(Ok(30), positive_double, bounded_add) = {:?}", result2);
    
    println!("=====================================\n");
}

fn test_complex_operations() {
    println!("=== 복합 연산 예제 ===");
    
    // Option 복합 연산
    let complex_option = Some(3)
        .bind(|x| Some(x * x))  // 제곱
        .bind(|x| if x > 5 { Some(x) } else { None })  // 조건부 필터
        .bind(|x| Some(format!("결과: {}", x)));  // 문자열 변환
    println!("복합 Option 연산: {:?}", complex_option);
    
    // Result 복합 연산
    let complex_result: Result<String, &str> = Ok(4)
        .bind(|x| Ok(x * x))  // 제곱
        .bind(|x| if x > 10 { Ok(x) } else { Err("값이 너무 작음") })  // 조건부 검사
        .bind(|x| Ok(format!("성공: {}", x)));  // 문자열 변환
    println!("복합 Result 연산: {:?}", complex_result);
    
    println!("=====================================\n");
}

fn test_error_handling() {
    println!("=== 에러 처리 예제 ===");
    
    // 나눗셈 함수 (0으로 나누기 방지)
    fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("0으로 나눌 수 없습니다")
        } else {
            Ok(a / b)
        }
    }
    
    // 제곱근 함수 (음수 방지)
    fn safe_sqrt(x: i32) -> Result<f64, &'static str> {
        if x < 0 {
            Err("음수의 제곱근은 계산할 수 없습니다")
        } else {
            Ok((x as f64).sqrt())
        }
    }
    
    // 성공적인 연산
    let success_result = Ok(100)
        .bind(|x| safe_divide(x, 4))  // 100 / 4 = 25
        .bind(|x| safe_sqrt(x))       // sqrt(25) = 5.0
        .bind(|x| Ok(format!("최종 결과: {:.2}", x)));
    println!("성공적인 연산: {:?}", success_result);
    
    // 에러가 발생하는 연산
    let error_result = Ok(100)
        .bind(|x| safe_divide(x, 0))  // 0으로 나누기 에러
        .bind(|x| safe_sqrt(x))       // 이 연산은 실행되지 않음
        .bind(|x| Ok(format!("최종 결과: {:.2}", x)));
    println!("에러가 발생하는 연산: {:?}", error_result);
    
    println!("=====================================\n");
}

fn test_practical_examples() {
    println!("=== 실용적인 사용 예제 ===");
    
    // 사용자 데이터 처리 예제
    #[derive(Debug)]
    struct User {
        name: String,
        age: i32,
        email: String,
    }
    
    fn validate_age(age: i32) -> Result<i32, &'static str> {
        if age >= 0 && age <= 150 {
            Ok(age)
        } else {
            Err("유효하지 않은 나이입니다")
        }
    }
    
    fn validate_email(email: &str) -> Result<String, &'static str> {
        if email.contains('@') {
            Ok(email.to_string())
        } else {
            Err("유효하지 않은 이메일 주소입니다")
        }
    }
    
    fn create_user(name: String, age: i32, email: String) -> Result<User, &'static str> {
        validate_age(age)
            .bind(|valid_age| validate_email(&email).bind(|valid_email| {
                Ok(User {
                    name,
                    age: valid_age,
                    email: valid_email,
                })
            }))
    }
    
    // 유효한 사용자 생성
    let valid_user = create_user("김철수".to_string(), 25, "kim@example.com".to_string());
    println!("유효한 사용자: {:?}", valid_user);
    
    // 잘못된 나이로 사용자 생성
    let invalid_age_user = create_user("이영희".to_string(), -5, "lee@example.com".to_string());
    println!("잘못된 나이: {:?}", invalid_age_user);
    
    // 잘못된 이메일로 사용자 생성
    let invalid_email_user = create_user("박민수".to_string(), 30, "invalid-email".to_string());
    println!("잘못된 이메일: {:?}", invalid_email_user);
    
    println!("=====================================\n");
}

fn run_all_examples() {
    println!("=== 모든 예제 실행 ===\n");
    
    test_option_monad();
    test_result_monad();
    test_functor_mapping();
    test_applicative();
    test_option_chaining();
    test_result_chaining();
    test_complex_operations();
    test_error_handling();
    test_practical_examples();
    
    println!("모든 예제 실행 완료!\n");
}