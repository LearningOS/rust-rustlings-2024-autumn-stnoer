// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,  // 用 Mutex 包装 jobs_completed
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 更新共享值
            let mut jobs_completed = status_shared.jobs_completed.lock().unwrap();
            *jobs_completed += 1;  // 使用解引用来更新值
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终的 jobs_completed 值
    let final_jobs_completed = status.jobs_completed.lock().unwrap();
    println!("jobs completed {}", *final_jobs_completed);  // 解引用以打印值
}
