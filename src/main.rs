use std::{fs::File, io::Write};

use mmm::run_app;

#[tokio::main]
async fn main() {
    let mmm_result = run_app("mmm").await;
    match mmm_result {
        Ok(path) => {
            let file_path = "/tmp/mmm.path";
            let mut file = File::create(file_path).expect("Failed to create or open the temp file");
            file.write_all(path.to_string_lossy().as_bytes())
                .expect("Failed to write to temp file");
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("An error ocurred, {}", err);
            std::process::exit(1)
        }
    }
}
