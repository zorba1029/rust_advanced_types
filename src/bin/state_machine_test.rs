// 
// Type-Level State Machines with Phantom Data
// 
use rust_higher_kined_types::state_machine::{Scheduler, Task, demonstrate_state_machine_safety};

fn test_scheduler_type_level_state_machines() {
    println!("3. === Type-Level State Machines with Phantom Data ===");
    println!();

    // 1. 기본 상태 전환 데모
    println!("[1] 🎯 Basic State Transitions:");
    
    // 체이닝으로 상태 전환과 태스크 추가를 한 번에 처리
    let scheduler = Scheduler::new()
        .initialize()
        .add_task(Task::new(1, "Initialize Database", 5))
        .add_task(Task::new(2, "Load Configuration", 8))
        .add_task(Task::new(3, "Start Web Server", 10))
        .add_task(Task::new(4, "Run Health Check", 3));
    
    println!("    📊 Added {} tasks", scheduler.task_count());
    println!();

    // 스케줄러 시작 및 작업 실행
    println!("[2] 🏃 Execution Phase:");
    let mut scheduler = scheduler.start();
    
    // 모든 태스크 실행
    while scheduler.has_tasks() {
        scheduler = scheduler.execute_next();
        if let Some(current) = scheduler.current_task() {
            println!("    🔍 Current task: {} (Priority: {})", current.name, current.priority);
        }
    }
    
    // 마지막 태스크 실행 시도
    scheduler = scheduler.execute_next();
    println!();

    // 스케줄러 정지 및 요약
    println!("[3] ⏹️ Shutdown Phase:");
    let scheduler = scheduler.stop();
    println!("    {}", scheduler.get_summary());
    println!();

    // 2. 고급 상태 전환 데모
    println!("[4] 🔄 Advanced State Management:");
    
    // 새로운 스케줄러로 pause/resume 데모
    let scheduler = Scheduler::new()
        .initialize()
        .add_task(Task::new(5, "Backup Data", 7))
        .add_task(Task::new(6, "Send Notifications", 4));

    let mut scheduler = scheduler.start();
    scheduler = scheduler.execute_next(); // 하나 실행
    
    // 실행 중 일시정지
    let scheduler = scheduler.pause();
    println!("    🔧 Reconfiguring paused scheduler...");
    
    // 재구성 후 재시작
    let scheduler = scheduler
        .add_task(Task::new(7, "Emergency Task", 9))
        .start();
    
    println!("    📈 Restarted with {} remaining tasks", scheduler.remaining_tasks());
    println!();

    // 3. 타입 안전성 데모
    println!("[5] 🔒 Demonstrating compile-time state safety:");
    demonstrate_state_machine_safety();
    println!();

    // 4. 에러 방지 예시 (주석으로 설명)
    println!("[6] 💡 Compile-time Safety Examples:");
    println!("    ❌ These would NOT compile:");
    println!("    ❌ Scheduler::new().start()           // Can't start uninitialized");
    println!("    ❌ scheduler.initialize().execute()   // Can't execute non-running");
    println!("    ❌ running_scheduler.add_task(task)   // Can't modify running scheduler");
    println!("    ✅ All state violations caught at compile time!");
}

fn main() {
    test_scheduler_type_level_state_machines();
} 