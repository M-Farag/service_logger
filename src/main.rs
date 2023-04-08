use service_logger::{self, Config};

fn main() {
    println!("Service is running...");
    let args = Config::new().unwrap_or_else(
        |err| {
            eprintln!("Err: {}", err);
            std::process::exit(1);
        }
    );    
}
