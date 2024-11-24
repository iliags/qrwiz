use clap::Parser;
use qrwiz::config::Config;

fn main() {
    let config = Config::parse();

    println!("{:?}", config);

    match config.config_file {
        Some(file) => {
            println!("Config file: {}", file);
        }
        None => {
            println!("No config file specified");
        }
    }

    /*
    use qrcode_generator::QrCodeEcc;
    std::fs::create_dir_all("test_output").unwrap_or_else(|e| {
        eprintln!("Error creating test directory: {}", e);
        std::process::exit(1);
    });

    let result = qrcode_generator::to_png_to_file(
        "Hello world!",
        QrCodeEcc::Low,
        128,
        "test_output/file_output.png",
    );

    match result {
        Ok(_) => {
            println!("QR code generated successfully!");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
     */
}
