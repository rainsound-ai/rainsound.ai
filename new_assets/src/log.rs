use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "build")] {
    pub fn log(message: &str) {
        println!("{}", message);
    }
} else {
    use worker::*;
    pub fn log(message: &str) {
        console_debug!("{}", message);
    }
}
}
