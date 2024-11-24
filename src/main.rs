use qrcode_generator::QrCodeEcc;

fn main() {
    std::fs::create_dir_all("test_output").unwrap_or_else(|e| {
        eprintln!("Error creating test directory: {}", e);
        std::process::exit(1);
    });

    let result = qrcode_generator::to_png_to_file(
        "Hello world!",
        QrCodeEcc::Low,
        1024,
        "test_output/file_output.png",
    );

    match result {
        Ok(_) => println!("QR code generated successfully!"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
