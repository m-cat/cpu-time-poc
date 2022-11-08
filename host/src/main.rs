use comfy_table::Table;
use once_cell::sync::Lazy;
use std::{process, sync::Mutex};

const CHILD_LOCATION: &str = "../child/target/debug/child";

const MAX_NUM_PROCESSES: u32 = 32;

fn main() {
    // Add top row of table.
    let mut table = Table::new();
    table.set_header(vec![
        "# children",
        "avg. cpu time (ms)",
        "avg. wall time (ms)",
        "avg. # hashes",
        "hashes / cpu time",
        "hashes / wall time",
    ]);

    static CPU_TIME_COUNT: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
    static WALL_CLOCK_TIME_COUNT: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
    static HASH_COUNT: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));

    for num_processes in 1..=MAX_NUM_PROCESSES {
        println!("Running {num_processes} simultaneous children...");

        let mut thread_handles = vec![];

        // Reset counts.
        *CPU_TIME_COUNT.lock().unwrap() = 0;
        *WALL_CLOCK_TIME_COUNT.lock().unwrap() = 0;
        *HASH_COUNT.lock().unwrap() = 0;

        // New iteration; run specified number of processes in parallel.
        for _ in 0..num_processes {
            thread_handles.push(std::thread::spawn(|| {
                let output = process::Command::new(CHILD_LOCATION)
                    .output()
                    .expect("failed to execute process");

                // Get child's reported CPU time, wall clock time, and hash count.
                let s = std::str::from_utf8(&output.stdout).unwrap();
                let v: Vec<&str> = s.trim().split(",").collect();

                // Add results to running counts for this iteration.
                *CPU_TIME_COUNT.lock().unwrap() += v[0].parse::<u128>().unwrap();
                *WALL_CLOCK_TIME_COUNT.lock().unwrap() += v[1].parse::<u128>().unwrap();
                *HASH_COUNT.lock().unwrap() += v[2].parse::<u128>().unwrap();
            }));
        }

        // Wait for all processes to finish.
        for thread_handle in thread_handles.into_iter() {
            thread_handle.join().unwrap();
        }

        // Add a row to the table with the following columns.
        //
        // - number of simultaneous children
        // - average cpu time (ms)
        // - average wall clock time (ms)
        // - average number of hashes
        // - number of hashes/cpu time
        // - number of hashes/wall clock time
        let num_processes = num_processes.into();
        let cpu_time = *CPU_TIME_COUNT.lock().unwrap();
        let wall_clock_time = *WALL_CLOCK_TIME_COUNT.lock().unwrap();
        let hash_count = *HASH_COUNT.lock().unwrap();
        table.add_row(vec![
            num_processes,
            cpu_time / num_processes,
            wall_clock_time / num_processes,
            hash_count / num_processes,
            hash_count / cpu_time,
            hash_count / wall_clock_time,
        ]);
    }

    println!("{table}");
}
