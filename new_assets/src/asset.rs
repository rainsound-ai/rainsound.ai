use crate::performance_budget::HowCloseToBudget;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub trait Asset {
    fn path(&self) -> &Path;

    fn bytes(&self) -> Vec<u8>;

    fn content_type(&self) -> String;

    fn save_to_disk(&self) {
        println!("Saving asset: {:?}", self.path());
        let path = self.path_on_disk();

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        let bytes = self.bytes();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn path_on_disk(&self) -> PathBuf {
        let path = self.path();

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        crate::built_assets_dir().join(&path)
    }

    fn check_performance_budget(&self) -> HowCloseToBudget {
        HowCloseToBudget::new(self)
    }

    // Used for enforcing performance budgets.
    fn load_time_budget(&self) -> Duration;
}
