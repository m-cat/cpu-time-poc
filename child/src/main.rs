use cpu_time::ProcessTime;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime};

const TARGET_CPU_TIME: Duration = Duration::from_secs(5);
const POLL_DELAY: Duration = Duration::from_secs(1);

static HASH_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn main() {
    let cpu_time_start = ProcessTime::now();
    let wall_clock_time_start = SystemTime::now();

    thread::spawn(|| {
        const HASHES_IN_LOOP: u64 = 10;

        let mut hasher = Sha3_256::new();

        loop {
            for _ in 0..HASHES_IN_LOOP {
                // Get random data.
                let data: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

                // Do a hash.
                hasher.update(&data);
            }

            // Increment hash counter.
            let mut counter = HASH_COUNTER.lock().unwrap();
            *counter += HASHES_IN_LOOP;
        }
    });

    loop {
        let cpu_time = cpu_time_start.elapsed();

        if cpu_time >= TARGET_CPU_TIME {
            let wall_clock_time = SystemTime::now()
                .duration_since(wall_clock_time_start)
                .unwrap();
            let hash_count = *HASH_COUNTER.lock().unwrap();

            println!(
                "{},{},{}",
                cpu_time.as_millis(),
                wall_clock_time.as_millis(),
                hash_count
            );
            std::process::exit(0);
        } else {
            thread::sleep(POLL_DELAY);
        }
    }
}
