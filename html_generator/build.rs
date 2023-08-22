use std::{fs, io::Write};

fn main() {
    let mut file = fs::File::create("../target/build_time.txt").unwrap();
    let build_time = chrono::Local::now().to_rfc3339();
    write!(file, "{}", build_time).ok();
}
