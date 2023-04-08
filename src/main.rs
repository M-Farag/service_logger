use service_logger::{self, Config};

fn main() {
    println!("Service is running...");

    let args = Config::new().unwrap_or_else(
        |err| {
            eprintln!("Err: {}", err);
            std::process::exit(1);
        }
    );   

    let mut log_file = service_logger::get_an_appending_file_handler(&args.file_path).unwrap_or_else(
        |err| {
            eprintln!("Err: {}", err);
            std::process::exit(1);
        }
    );


    if let Err(err) =  service_logger::write_to_file(&mut log_file, &args.message, true) {
        eprintln!("Err: {}", err);
        std::process::exit(1);
    }
    
    println!("Service is done.");
}
