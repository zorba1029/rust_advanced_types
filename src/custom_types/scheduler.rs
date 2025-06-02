//
// Type-Level State Machines with Phantom Data
//
// -- Using PhantomData to encode state transitions at the type level
// This ensures state machine invariants are checked at compile time

use std::marker::PhantomData;

// State types - these exist only at the type level
pub struct Uninitialized;
pub struct Initialized;
pub struct Running;
pub struct Stopped;

// Task representation
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub priority: u8,
}

impl Task {
    pub fn new(id: u32, name: &str, priority: u8) -> Self {
        Task {
            id,
            name: name.to_string(),
            priority,
        }
    }
}

// Scheduler with phantom type parameter for state
pub struct Scheduler<State> {
    tasks: Vec<Task>,
    current_task: Option<Task>,
    _state: PhantomData<State>,
}

// Implementation for Uninitialized state
impl Scheduler<Uninitialized> {
    /// Create a new scheduler in uninitialized state
    pub fn new() -> Self {
        println!("📋 Creating new scheduler...");
        Scheduler {
            tasks: Vec::new(),
            current_task: None,
            _state: PhantomData,
        }
    }

    /// Initialize the scheduler - transitions to Initialized state
    pub fn initialize(self) -> Scheduler<Initialized> {
        println!("🔧 Initializing scheduler...");
        Scheduler {
            tasks: self.tasks,
            current_task: self.current_task,
            _state: PhantomData,
        }
    }
}

// Implementation for Initialized state
impl Scheduler<Initialized> {
    /// Add a task to the scheduler
    pub fn add_task(mut self, task: Task) -> Self {
        println!("   ➕ Adding task: {} (priority: {})", task.name, task.priority);
        self.tasks.push(task);
        self
    }

    /// Start the scheduler - transitions to Running state
    pub fn start(mut self) -> Scheduler<Running> {
        println!("    🚀 Starting scheduler with {} tasks...", self.tasks.len());
        
        // Sort tasks by priority (higher priority first)
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        Scheduler {
            tasks: self.tasks,
            current_task: self.current_task,
            _state: PhantomData,
        }
    }

    /// Get the number of tasks
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

// Implementation for Running state
impl Scheduler<Running> {
    /// Execute the next task
    pub fn execute_next(mut self) -> Self {
        if let Some(task) = self.tasks.pop() {
            println!("⚡ Executing task: {} (ID: {})", task.name, task.id);
            self.current_task = Some(task);
        } else {
            println!("✅ No more tasks to execute");
        }
        self
    }

    /// Get current running task
    pub fn current_task(&self) -> Option<&Task> {
        self.current_task.as_ref()
    }

    /// Check if there are more tasks
    pub fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }

    /// Stop the scheduler - transitions to Stopped state
    pub fn stop(self) -> Scheduler<Stopped> {
        println!("⏹️ Stopping scheduler...");
        Scheduler {
            tasks: self.tasks,
            current_task: self.current_task,
            _state: PhantomData,
        }
    }

    /// Pause and return to Initialized state for reconfiguration
    pub fn pause(self) -> Scheduler<Initialized> {
        println!("⏸️ Pausing scheduler for reconfiguration...");
        Scheduler {
            tasks: self.tasks,
            current_task: None, // Clear current task when pausing
            _state: PhantomData,
        }
    }
}

// Implementation for Stopped state
impl Scheduler<Stopped> {
    /// Get execution summary
    pub fn get_summary(&self) -> String {
        let completed_task = self.current_task.as_ref()
            .map(|t| format!("Last executed: {}", t.name))
            .unwrap_or_else(|| "No tasks executed".to_string());
        
        format!("📊 Scheduler Summary - Remaining tasks: {}, {}", 
                self.tasks.len(), completed_task)
    }

    /// Reset to initialized state for reuse
    pub fn reset(self) -> Scheduler<Initialized> {
        println!("🔄 Resetting scheduler...");
        Scheduler {
            tasks: Vec::new(),
            current_task: None,
            _state: PhantomData,
        }
    }

    /// Restart with current tasks
    pub fn restart(self) -> Scheduler<Running> {
        println!("🔁 Restarting scheduler...");
        Scheduler {
            tasks: self.tasks,
            current_task: None,
            _state: PhantomData,
        }
    }
}

// Common implementations for all states
impl<State> Scheduler<State> {
    /// Get remaining task count (available in all states)
    pub fn remaining_tasks(&self) -> usize {
        self.tasks.len()
    }
}

// Demonstration of type-level state enforcement
pub fn demonstrate_type_safety() {
    println!("🔒 Demonstrating compile-time state safety:");
    
    let scheduler = Scheduler::new();
    // scheduler.start(); // ❌ This would not compile! Can't start uninitialized scheduler
    
    let initialized_scheduler = scheduler.initialize();
    // scheduler.execute_next(); // ❌ This would not compile! Can't execute on non-running scheduler
    
    let running_scheduler = initialized_scheduler.start();
    // scheduler.add_task(task); // ❌ This would not compile! Can't add tasks to running scheduler
    
    let running_scheduler = running_scheduler.execute_next();
    // scheduler.get_summary(); // ❌ This would not compile! Can't get summary on stopped scheduler

    let stopped_scheduler = running_scheduler.stop();
    // scheduler.pause(); // ❌ This would not compile! Can't pause stopped scheduler

    let running_scheduler = stopped_scheduler.restart();
    // scheduler.add_task(task); // ❌ This would not compile! Can't add tasks to stopped scheduler

    let _stopped_scheduler = running_scheduler.stop();
    // let running_scheduler = restarted_scheduler.start();
    println!("✅ All state transitions are compile-time verified!");
}
