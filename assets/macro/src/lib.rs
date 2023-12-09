use proc_macro::TokenStream;

mod browser_crate;
mod deprecated;
mod logger;
mod parse_macro_arguments;
mod tailwind;

#[proc_macro]
pub fn save_to_disk(input: TokenStream) -> TokenStream {
    deprecated::save_assets_to_disk(input)
}

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    // block_until_other_invocations_are_finished("build_tailwind");
    // tailwind::build(input)
    with_lock_file("build_tailwind", || tailwind::build(input))
}

#[proc_macro]
pub fn build_browser_crate(input: TokenStream) -> TokenStream {
    with_lock_file("build_browser_crate", || browser_crate::build(input))
}

// Prevents multiple invocations of the same macro from running at the same time.
// This is important since many of our macros write to the file system,
// and multiple invocations running at once can interfere with each other.
fn with_lock_file<T>(macro_name: &'static str, run_macro: impl FnOnce() -> T) -> T {
    let lock_file_name = format!("assets_macro_{}.lock", macro_name);
    let lock_file_path = std::env::temp_dir().join(lock_file_name);

    wait_for_lock_to_be_released(macro_name, &lock_file_path);

    // Acquire the lock.
    log::info!("Acquiring lock for {}", macro_name);
    touch(&lock_file_path).unwrap();

    // Run the macro.
    log::info!("Running macro {}", macro_name);
    let result = run_macro();

    // Release the lock.
    log::info!("Releasing lock for {}", macro_name);
    match std::fs::remove_file(&lock_file_path) {
        Ok(_) => {}
        Err(error) => {
            log::warn!("Error releasing lock for {}: {}", macro_name, error);
        }
    }

    result
}

fn wait_for_lock_to_be_released(macro_name: &'static str, lock_file_path: &std::path::Path) {
    use std::fs;
    use std::time::Duration;

    // This is necessary because sometimes our macro will get
    // invoked multiple times in quick succession and we
    // need to make sure one of them has the change to take
    // the lock before the others.
    sleep_for_random_amount_of_time();

    if !lock_file_path.exists() {
        return;
    }

    // Delete the lock file if it was created more than a minute ago.
    let metadata = fs::metadata(lock_file_path).unwrap();
    let creation_time = metadata.created().unwrap();
    let elapsed = std::time::SystemTime::now()
        .duration_since(creation_time)
        .unwrap();

    let one_minute = Duration::from_secs(60);

    if elapsed > one_minute {
        std::fs::remove_file(lock_file_path).unwrap();
        return;
    }

    // Wait for the lock file to be deleted.
    let start_time = std::time::Instant::now();
    let max_wait_time = Duration::from_secs(10);

    // Wait for another process to release the lock.
    log::info!(
        "Waiting for another process to release the lock {}",
        macro_name
    );

    loop {
        if !lock_file_path.exists() {
            break;
        }

        if start_time.elapsed() > max_wait_time {
            panic!("Waited too long for lock file to be deleted.");
        }

        sleep_for_random_amount_of_time();
    }
}

fn sleep_for_random_amount_of_time() {
    let sleep_duration_millis = rand::random::<u64>() % 500;
    let sleep_duration = std::time::Duration::from_millis(sleep_duration_millis);
    std::thread::sleep(sleep_duration);
}

fn touch(path: &std::path::Path) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
